#![allow(unused_imports)]
use crate::bindings::world_gpio_app::{GpioApp, RunnerLinuxHostCtx, RunnerSimHostCtx};
use anyhow::{Error, Result};
use chrono::Utc;
use clap::Parser;
use futures::executor::block_on;
use std::time::{Duration, Instant};
use std::{path::PathBuf, sync::Arc};
use tokio::{sync::Barrier, time::sleep_until};
use wasmtime::{
    component::{Component, Linker},
    AsContext, Config, Engine, Store,
};
use wasmtime_wasi::preview2::{wasi, Table, WasiCtxBuilder};

enum HostImpl {
    Linux,
    Sim,
}

impl std::str::FromStr for HostImpl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux" => Ok(Self::Linux),
            "sim" => Ok(Self::Sim),
            _ => Err(anyhow::anyhow!("invalid host implementation")),
        }
    }
}

#[derive(Clone, Debug)]
enum ComponentSource {
    Rs,
    Js,
}

impl std::fmt::Display for ComponentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentSource::Rs => write!(f, "rust"),
            ComponentSource::Js => write!(f, "javascript"),
        }
    }
}

impl std::str::FromStr for ComponentSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Self::Rs),
            "javascript" => Ok(Self::Js),
            _ => Err(anyhow::anyhow!("invalid guest language")),
        }
    }
}

#[derive(Parser)]
#[structopt(name = "run", trailing_var_arg = true)]
pub struct RunCommand {
    component: PathBuf,

    #[structopt(long = "impl", default_value = "linux")]
    host_impl: HostImpl,

    // Here for profiling
    #[structopt(long = "instances", default_value = "1")]
    instances: usize,
    // Here for profiling
    #[structopt(long = "source", default_value = "rust")]
    guest_source: ComponentSource,
}

impl RunCommand {
    pub fn execute(&mut self) -> Result<()> {
        match self.host_impl {
            HostImpl::Sim => {
                let env = SimEnvironment::new(&self.component, self.guest_source.clone())?;
                let mut handles = Vec::with_capacity(self.instances);

                for i in 0..self.instances {
                    let e = env.clone();
                    let handle = std::thread::spawn(move || {
                        block_on(e.run_instance(&i.to_string())).expect("failed to run instance");
                    });
                    handles.push(handle);
                }

                for handle in handles {
                    handle
                        .join()
                        .expect("Couldn't join on the associated thread");
                }
            }
            HostImpl::Linux => {
                let e = LinuxEnvironment::new(&self.component)?;
                block_on(e.run_instance("0")).expect("failed to run instance");
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
struct LinuxEnvironment {
    engine: Engine,
    component: Component,
    linker: Arc<Linker<RunnerLinuxHostCtx>>,
}

impl LinuxEnvironment {
    pub fn new(component_path: &PathBuf) -> Result<Self, Error> {
        let mut config = Config::new();
        config
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable)
            .wasm_component_model(true)
            .async_support(true)
            .allocation_strategy(wasmtime::InstanceAllocationStrategy::OnDemand) // resources are allocated at instantiation time and immediately deallocated when the instance is dropped
            .strategy(wasmtime::Strategy::Cranelift)
            .cranelift_opt_level(wasmtime::OptLevel::SpeedAndSize);

        let engine = Engine::new(&config).unwrap();
        let mut linker = Linker::new(&engine);
        let component = Component::from_file(&engine, component_path)?;

        // bind everything from the wasi command world
        wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)?;

        // binding host
        crate::bindings::world_gpio_app::thesis::gpio::types::add_to_linker(
            &mut linker,
            |ctx: &mut RunnerLinuxHostCtx| ctx,
        )?;

        Ok(Self {
            engine,
            linker: Arc::new(linker),
            component,
        })
    }

    pub async fn run_instance(self, instance_number: &str) -> Result<(), Error> {
        let mut table = Table::new();
        let wasi = WasiCtxBuilder::new()
            .inherit_stderr()
            .inherit_stdin()
            .inherit_stdout()
            .inherit_stdio()
            .push_env("INSTANCE_NUMBER", instance_number)
            .build(&mut table)?;

        let mut store = Store::new(&self.engine, RunnerLinuxHostCtx { wasi, table });

        let (gpio, _instance) = crate::bindings::world_gpio_app::GpioApp::instantiate_async(
            &mut store,
            &self.component,
            &self.linker,
        )
        .await?;

        let _res = gpio.call_start(&mut store).await?;
        println!("res: {:?}", _res);
        Ok(())
    }
}

#[derive(Clone)]
struct SimEnvironment {
    engine: Engine,
    component: Component,
    component_source_lang: ComponentSource,
    linker: Arc<Linker<RunnerSimHostCtx>>,
}

impl SimEnvironment {
    pub fn new(component_path: &PathBuf, source: ComponentSource) -> Result<Self, Error> {
        let mut config = Config::new();
        config
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable)
            .wasm_component_model(true)
            .async_support(true)
            .max_wasm_stack(10 * 1024 * 1024) // 10MB
            .async_stack_size(2 << 31) // 200MB
            .allocation_strategy(wasmtime::InstanceAllocationStrategy::OnDemand) // resources are allocated at instantiation time and immediately dealloced when the instance is dropped
            .strategy(wasmtime::Strategy::Cranelift)
            .cranelift_opt_level(wasmtime::OptLevel::SpeedAndSize);

        let engine = Engine::new(&config).unwrap();
        let mut linker = Linker::new(&engine);

        let component = Component::from_file(&engine, component_path)?;

        // bind everything from the wasi command world
        wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)?;

        // bind gpio bindings
        crate::bindings::world_gpio_app::thesis::gpio::types::add_to_linker(
            &mut linker,
            |ctx: &mut RunnerSimHostCtx| ctx,
        )?;

        Ok(Self {
            engine,
            linker: Arc::new(linker),
            component,
            component_source_lang: source,
        })
    }

    pub async fn run_instance(self, instance_number: &str) -> Result<(), Error> {
        let mut table = Table::new();
        let wasi = WasiCtxBuilder::new()
            .inherit_stderr()
            .inherit_stdin()
            .inherit_stdout()
            .inherit_stdio()
            .push_env("INSTANCE_NUMBER", instance_number)
            .build(&mut table)?;

        let host_state = RunnerSimHostCtx { wasi, table };
        let mut store = Store::new(&self.engine, host_state);

        let (gpio_world, _instance) =
            GpioApp::instantiate_async(&mut store, &self.component, &self.linker.clone()).await?;
        let _res = gpio_world.call_start(&mut store).await?;

        let start = Instant::now();

        let _ = gpio_world.call_host_to_guest(&mut store).await?;
        println!(
            "instance={},action=host-to-guest-{},elapsed={:?}",
            instance_number,
            self.component_source_lang,
            start.elapsed().as_micros()
        );

        Ok(())
    }
}

use super::thesis::gpio::types;
use std::hint::black_box;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView};

#[async_trait::async_trait]
impl types::Host for RunnerSimHostCtx {
    async fn gpio_init(
        &mut self,
        _port: u32,
        _pin: u32,
        _mode: types::Mode,
    ) -> std::result::Result<std::result::Result<u32, u32>, wasmtime::Error> {
        Ok(Ok(1))
    }

    async fn gpio_deinit(
        &mut self,
        _device: u32,
    ) -> std::result::Result<std::result::Result<(), u32>, wasmtime::Error> {
        println!(">>> called gpio_deinit (sim)");
        Ok(Ok(()))
    }
    async fn large_static(
        &mut self,
    ) -> std::result::Result<std::result::Result<(), u32>, wasmtime::Error> {
        const ARRAY_SIZE: usize = 5 * 1_024 * 1_024;
        static LARGE_STATIC: [u8; ARRAY_SIZE] = [0; ARRAY_SIZE];
        black_box(LARGE_STATIC);
        Ok(Ok(()))
    }
}

pub(crate) struct RunnerSimHostCtx {
    pub(crate) wasi: WasiCtx,
    pub(crate) table: Table,
}

impl WasiView for RunnerSimHostCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

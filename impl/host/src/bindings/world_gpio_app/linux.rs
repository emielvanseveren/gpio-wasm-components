use super::thesis::gpio::types;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use std::hint::black_box;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView};

#[async_trait::async_trait]
impl types::Host for RunnerLinuxHostCtx {
    async fn gpio_init(
        &mut self,
        port: u32,
        pin: u32,
        _mode: types::Mode,
    ) -> std::result::Result<std::result::Result<u32, u32>, wasmtime::Error> {
        // here we want to do the actual binding with the os device

        let mut chip = Chip::new("/dev/gpiochip0")?;
        let output = chip.get_line(pin)?;

        // 0 is the default value it should have when it is configured as an output
        let output_handle = output.request(LineRequestFlags::OUTPUT, 0, "who-is-using-this")?;

        output_handle.set_value(1)?;

        println!(">>> called gpio_init (linux)");
        Ok(Ok(1))
    }

    async fn gpio_deinit(
        &mut self,
        _device: u32,
    ) -> std::result::Result<std::result::Result<(), u32>, wasmtime::Error> {
        println!(">>> called gpio_deinit (linux)");
        Ok(Ok(()))
    }

    async fn large_static(
        &mut self,
    ) -> std::result::Result<std::result::Result<(), u32>, wasmtime::Error> {
        const ARRAY_SIZE: usize = 5 * 1_000_000;
        static LARGE_STATIC_ARRAY: [u8; ARRAY_SIZE] = [0; ARRAY_SIZE];
        black_box(LARGE_STATIC_ARRAY);
        Ok(Ok(()))
    }
}

pub(crate) struct RunnerLinuxHostCtx {
    pub(crate) wasi: WasiCtx,
    pub(crate) table: Table,
}

impl WasiView for RunnerLinuxHostCtx {
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

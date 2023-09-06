wasmtime::component::bindgen!({
    path: "../wit",
    world: "gpio-app",
    async: true,    // wasmtime-wasi currently only has an async implementation
    tracing: false  // Adds calls to tracing::span! before each import or export is called to log arguments and return values.

});

use wasmtime_wasi::preview2::{Table, TableError};

mod linux;
mod sim;

pub(crate) use linux::RunnerLinuxHostCtx;
pub(crate) use sim::RunnerSimHostCtx;

// temporary placeholder
pub(crate) struct Device {
    line: u32,
}

// TODO: should probably be a line
trait TableFs {
    fn push_device(&mut self, device: Device) -> Result<u32, TableError>;
    fn delete_device(&mut self, device_handle: u32) -> Result<Device, TableError>;
    fn is_device(&self, device_handle: u32) -> bool;
    fn get_device(&self, device_handle: u32) -> Result<&Device, TableError>;
}

impl TableFs for Table {
    fn is_device(&self, device_handle: u32) -> bool {
        self.is::<u32>(device_handle)
    }
    fn delete_device(&mut self, device_handle: u32) -> Result<Device, TableError> {
        self.delete(device_handle)
    }
    fn push_device(&mut self, device: Device) -> Result<u32, TableError> {
        self.push(Box::new(device))
    }
    fn get_device(&self, device_handle: u32) -> Result<&Device, TableError> {
        self.get(device_handle)
    }
}

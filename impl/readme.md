# Introduction

This implementation tends to show that the Component Model can simplify embedded application development.
For illustrative purposes, consider a scenario where a guest is tasked with controlling an LED. As a simple test,
the guest controls an LED. How one controls the LED depends on the underlying host system. Each host system has their own world.
Each world consists out of imports and exports. The imports are implemented by the runtime.
The guest can now use these imports. So here, the worlds only have a few interfaces for GPIO. E.g. to set a pin to high.

## Folder structure

- `guest-rs` contains code that creates the guest component written in Rust (including the actual application code)
- `guest-js` contains code that creates the guest component written in JavaScript (including the actual application code)
- `host` contains the wasmtime runtime that can run components:
  - Contains embedded linux host GPIO-world implementation
  - Contains simulated host GPIO-world implementation
- `wit` folder contains the wit world interface
- `native` is a native Rust application with the same implementation as the guest code.

## Usage

### Prerequisites

```sh
# Add the wasm32 platform's standard library compilation target.
rustup target add wasm32-wasi

# Install wasm-tools to transform a module into a component.
# Use --force to override if you've previously installed it.
cargo install wasm-tools --version 1.0.37

# Install js dependencies
cd ./guest-js && npm install
```

### Execution

All scripts can be found in the `Makefile`. E.g. to run the guest-rs Linux implementation: `make run-linux-rs`.

#### Memory usage (basic)

```sh
  massif-visualizer massif.out.{PID}
```

## Additional information

- The module `wasi_snapshot_preview1.reactor.wasm` acts as a bridge connecting the WASI `preview1` ABI to `preview2` ABI.

- `wasi_snapshot_preview1.COMMAND.wasm` adapter is solely for programs that are CLI style and use \_start (which initialize guest libc and then call the user's main)

- Encountering the error `error: component-type version 3 does not match supported version 3` implies an outdatd version of reactor.wasm

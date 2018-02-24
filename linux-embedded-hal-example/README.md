
The file src/main.rs demonstrates how to use the platform
agnostic ADC driver [adc-mcp3008](https://crates.io/crates/adc-mcp3008)
together with the [embedded-hal](https://crates.io/crates/embedded-hal)
implementation [linux-embedded-hal](https://crates.io/crates/linux-embedded-hal).

You need a Raspberry Pi (or similar) board  to run
the code.

You can compile the code by running:

```
cargo build --target armv7-unknown-linux-gnueabihf --release
```

The executable will be in the folder *target/armv7-unknown-linux-gnueabihf/release/*


The code will simply read from channel 0 of the MCP3008 ADC and display the 
result on stdout.

Please check out [Cross compiling Rust for the Raspberry Pi on Linux](https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658)
if you are new to writing Rust code for Raspberry Pi and similar systems.





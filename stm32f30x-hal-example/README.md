
The file src/main.rs demonstrates how to use the platform
agnostic ADC driver [adc-mcp3008](https://crates.io/crates/adc-mcp3008)
together with the [embedded-hal](https://crates.io/crates/embedded-hal)
implementation [stm32f30x-hal](https://crates.io/crates/stm32f30x-hal).

You need an STM32F3Discovery (or compatible) board to run
the code.

You can compile the code by running:

```
make release
```

The code can be flashed by running:

```
make flash_release
```

You need the [st-flash](https://github.com/texane/stlink) utility to flash
the code.

The code will simply read from channel 0 of the MCP3008 ADC and send the
data out through the serial port. You can print it out by running:

```
python recv.py
```

The Python *serial* module should be installed.

A USB-to-serial converted should be used to interface the UART pins of
the discovery board with the USB port of the PC/Laptop.


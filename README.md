
This repository contains programs demonstrating the use of
the platform agnostic [adc-mcp3008](https://github.com/pcein/adc-mcp3008)
driver written in Rust.

The programs basically read from channel 0 of the ADC and displays the
data either by printing it to stdout (Raspberry Pi etc) or sending it out
through a serial port (which can then be captured and printed by a Python
script).

## Platforms

- The directory *linux-embedded-hal-example* contains code which will run
  on a Raspberry Pi or any such embedded Linux board which has an
  SPI interface.

- The directory *stm32f3f30x-hal-example* contains code which will
  run on an stm32f3discovery board.



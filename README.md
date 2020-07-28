# hd44780_ntb

This is an implementation of a hardware driver for a
[HD44780](https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller/)
type LCD controller write in
[Rust](https://www.rust-lang.org/) which uses the
[linux-embedded-hal](https://github.com/rust-embedded/linux-embedded-hal)
library.
It exposes a simple trait-based API for the command set which minimizes the
coupling between the hardware driver (GPIO, I2C, etc) and the code that passes
commands and data to the display.

## Table Of Contents

  * [Getting Started](#getting-started)
  * [Using The Crate](#using-the-crate)
  * [Examples](#examples)
  * [Contributing](#contributing)

## Getting Started

You will need to have a recent version of Rust installed.
Any version of Rust that supports version 0.2 of the Linux embedded hal should
work but versions from 1.39 to 1.45 of Rust have been used during initial
development on both the nightly and release channels.
Earlier versions might work as well but have not been tested.

Development can be done on an OS (GPIO, I2C, etc) that Rust supports but the
expected target would typically be something like a Raspberry Pi, STM32, or
another embeddable system which can run Linux.
All initial development has been done with a combination of a laptop running
Windows 10 and a 4GB Raspberry Pi 4 running the Raspberry Pi OS (Raspbian).

### Using The Crate

To use the crate in your own project all you need to do is include it in
`[dependencies]` of you project's `Cargo.toml` file like you would any other
crate.

```toml
[dependencies]
hd44780-ntb = "0.0.4"
```

## Examples

You will find examples in the `examples` directory. The Raspberry Pi GPIO
examples were used for testing during initial development on a RPi 4.

## Contributing

Contributors are welcome.
I would like to see the hardware drives extended beyond just the Raspberry Pi
but I currently don't have access to other hardware for development or testing.
Make sure you have read the
[Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
All contributed code will be considered to also be contributed under a MIT
license.
Please include your information in a comment on all code files for the copyright
etc.

## License

All code is available under the [MIT](https://opensource.org/licenses/MIT) license.
You can find a copy of the license in the [LICENSE](LICENSE) file.

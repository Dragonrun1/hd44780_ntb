// MIT License
//
// Copyright © 2020-present, Michael Cummings <mgcummings@yahoo.com>.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//! Generic blocking driver that uses GPIO pins.
//!
//! This is a very simple blocking bit-bang way of doing things which is
//! commonly used with many micro-controllers.

use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::OutputPin;
use crate::{DisplayMode, EntryMode, FunctionMode, Result};
use crate::cmd::HD44780;
use crate::error::HdError::{IncorrectDataLen, SetOutputPin};
// use crate::write::{RegisterSelect, RegisterSelect::Cmnd, Write};
use crate::write::RegisterSelect::{self, Cmnd, Data};
use std::io::Write;

/// This is the driver used for direct GPIO pin connected HD44780 displays.
///
/// The HD44780 display normally has a 16 inline connector.
/// The only pins of concern in this driver are the `RS`, `E`, and the data bus
/// pins `D0` through `D7`.
/// When the display is going to be used with a 4 bit (pin) data bus make sure
/// the data output pins are connected to `D4` through `D7` and __NOT__ to the
/// lower data pins `D0` through `D4`.
///
/// # Remarks
///
/// This driver assumes that the `RW` input on the display is pulled to `GND`
/// forcing the display into `Write` mode at all times.
///
/// The driver can be switched between 4 and 8 bit (pin) interface by just
/// changing the number of pins given in `data` parameter to the
/// [new()](GpioDriver::new())
/// function when creating a new instance.
#[derive(Debug)]
pub struct GpioDriver<RS, EN, DP, D>
where
    RS: OutputPin,
    EN: OutputPin,
    DP: OutputPin,
    D: DelayUs<u16>,
{
    rs: RS,
    e: EN,
    data: Vec<DP>,
    delay: D,
}

impl<RS, EN, DP, D> GpioDriver<RS, EN, DP, D>
where
    RS: OutputPin,
    EN: OutputPin,
    DP: OutputPin,
    D: DelayUs<u16>,
{
    /// Create a new instance of driver.
    ///
    /// The HD44780 display normally has a 16 inline connector.
    ///
    /// # Arguments
    ///
    /// * `rs` - An already setup output GPIO pin that is connected to the
    /// register select input on display.
    /// * `e` - An already setup output GPIO pin that is connected to the
    /// enable input on display.
    /// * `data` - An already setup array or Vec of GPIO output pins that are
    /// connected to the data inputs of the display. Only 4 or 8 pins should be
    /// used.
    ///
    /// The driver assumes that the RW pin
    ///
    /// # Examples
    /// For examples of using the driver in both 4 and 8 bit modes have look at
    /// the
    /// [Raspberry Pi 4 bit](../../../../../../examples/rpi4bit/main.rs)
    /// and
    /// [Raspberry Pi 8 bit](../../../../../../examples/rpi8bit/main.rs)
    /// examples.
    pub fn new(rs: RS, e: EN, data: Vec<DP>, delay: D) -> GpioDriver<RS, EN, DP, D>
// where
    //     P: &'a mut Vec<&'a mut DP>,
    {
        GpioDriver {
            rs,
            e,
            // data: data.into(),
            data,
            delay,
        }
    }
}

impl<RS, EN, DP, D> Write for GpioDriver<RS, EN, DP, D>
where
    RS: OutputPin,
    EN: OutputPin,
    DP: OutputPin,
    D: DelayUs<u16>,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf {
            match self.data.len() {
                4 => {
                    let nibble = (byte & 0b1111_0000u8) >> 4;
                    Self::set_bus_bits(nibble, &mut self.data[..])?;
                    self.enable_bit_toggle()?;
                }
                8 => {
                    // Nothing special needs to be done for 8 bit bus.
                }
                _ => return Err(IncorrectDataLen.into()),
            }
            // Write lower nibble or full byte as needed.
            Self::set_bus_bits(*byte, &mut self.data[..])?;
            self.enable_bit_toggle()?;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// impl<RS, EN, DP, D> Write<D> for GpioDriver<RS, EN, DP, D>
// where
//     RS: OutputPin,
//     EN: OutputPin,
//     DP: OutputPin,
//     D: DelayUs<u16>,
// {
//     fn write(&mut self, byte: u8, ctrl: RegisterSelect, delay: &mut D) -> Result {
//         self.set_control_bits(ctrl)?;
//         match self.data.len() {
//             4 => {
//                 let nibble = (byte & 0b1111_0000u8) >> 4;
//                 Self::set_bus_bits(nibble, &mut self.data[..])?;
//                 self.enable_bit_toggle(delay)?;
//             }
//             8 => {
//                 // Nothing special needs to be done for 8 bit bus.
//             }
//             _ => return Err(IncorrectDataLen),
//         }
//         // Write lower nibble or full byte as needed.
//         Self::set_bus_bits(byte, &mut self.data[..])?;
//         self.enable_bit_toggle(delay)?;
//         Ok(())
//     }
// }

impl<RS, EN, DP, D> HD44780 for GpioDriver<RS, EN, DP, D>
where
    RS: OutputPin,
    EN: OutputPin,
    DP: OutputPin,
    D: DelayUs<u16>,
{
    fn command(&mut self, byte: u8) -> Result {
        self.set_control_bits(Cmnd)?;
        self.write(&[byte])?;
        self.set_control_bits(Data)?;
        Ok(())
    }
    /// Used to initialize the display into a know state.
    ///
    /// Normally the display controller's power on reset sets up the display
    /// into a known state.
    /// In cases where the reset hasn't done so correctly or another program has
    /// left the display in an unknown state this method can be used to get the
    /// display into a known state.
    /// This method can also be used to switch between 4 and 8 bit (pin) data
    /// bus modes.
    fn init(
        &mut self,
        fs_mode: Option<FunctionMode>,
        dc_mode: Option<DisplayMode>,
        ems_mode: Option<EntryMode>,
    ) -> Result {
        let fs = fs_mode.unwrap_or_default();
        let dc = dc_mode.unwrap_or_default();
        let ems = ems_mode.unwrap_or_default();
        // Insure display has had time to stabilize if just powered on.
        // This takes between 15 to 40ms depending on supplied voltage.
        // 40ms + 10% fudge factor.
        self.delay.delay_us(44000u16);
        // The display can be in any of 3 states:
        let mut cmd = 0x33u8;
        self.command(cmd)?;
        // self.write(cmd, Cmnd, delay)?;
        // Wait at least 4.1ms before issuing next instruction.
        // 4.1ms + 10% fudge factor.
        self.delay.delay_us(4510u16);
        // Phase 2.
        match self.data.len() {
            4 => {
                if fs.contains(FunctionMode::BITS_8) {
                    return Err(IncorrectDataLen);
                }
                cmd = 0x32;
                self.command(cmd)?;
                // self.write(cmd, Cmnd, delay)?;
                // Wait at least 100us before sending next command.
                // 100us + 10% fudge factor.
                self.delay.delay_us(110u16);
            }
            8 => {
                cmd = 0x33;
                self.command(cmd)?;
                // self.write(cmd, Cmnd, delay)?;
                // Wait at least 100us before sending next command.
                // 100us + 10% fudge factor.
                self.delay.delay_us(110u16);
            }
            _ => {
                return Err(IncorrectDataLen);
            }
        }
        self.function_set(&fs)?;
        self.display_control(&dc)?;
        self.entry_mode_set(&ems)?;
        self.clear_display()?;
        Ok(())
    }
}

impl<RS, EN, DP, D> GpioDriver<RS, EN, DP, D>
where
    RS: OutputPin,
    EN: OutputPin,
    DP: OutputPin,
    D: DelayUs<u16>,
{
    fn enable_bit_toggle(&mut self) -> Result
    {
        self.e.set_low().map_err(|_| SetOutputPin("enable"))?;
        // Give other pins some setup time before `en` toggle.
        self.delay.delay_us(1u16);
        self.e.set_high().map_err(|_| SetOutputPin("enable"))?;
        // Minimum time is ~1us but give it a little extra to ensure it is seen.
        self.delay.delay_us(1u16);
        self.e.set_low().map_err(|_| SetOutputPin("enable"))?;
        // Given a little time to ensure low state is seen.
        self.delay.delay_us(1u16);
        Ok(())
    }
    fn set_control_bits(&mut self, ctrl: RegisterSelect) -> Result {
        match ctrl {
            RegisterSelect::Cmnd => {
                self.rs
                    .set_low()
                    .map_err(|_| SetOutputPin("register select"))?;
            }
            RegisterSelect::Data => {
                self.rs
                    .set_high()
                    .map_err(|_| SetOutputPin("register select"))?;
            }
        }
        Ok(())
    }
    fn set_bus_bits(byte: u8, bus: &mut [DP]) -> Result {
        let mut mask = 0b0000_00001;
        let mut bit: u8;
        for pin in bus {
            bit = byte & mask;
            if bit == mask {
                pin.set_high().map_err(|_| SetOutputPin("data"))?;
            } else {
                pin.set_low().map_err(|_| SetOutputPin("data"))?;
            }
            mask <<= 1;
        }
        Ok(())
    }
}

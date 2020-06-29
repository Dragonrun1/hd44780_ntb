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

use crate::{Result, COMMAND_WAIT};
use embedded_hal::blocking::delay::DelayUs;
use std::fmt::Debug;

pub trait Write<D>
where
    D: DelayUs<u16>,
{
    /// The primary function required to write to the actual display.
    ///
    /// This function MUST BE implemented by all instances.
    ///
    /// ```edition2018,ignore
    /// lcd.write(data, RegisterSelect::Data, delay)?;
    /// ```
    fn write(&mut self, byte: u8, ctrl: RegisterSelect, delay: &mut D) -> Result;
    /// Convenience method which makes showing whole messages a lot easier.
    fn write_str(&mut self, str: &str, delay: &mut D) -> Result {
        for byte in str.as_bytes() {
            if *byte != 0x0Au8 {
                self.write(*byte, RegisterSelect::Data, delay)?;
            } else {
                self.write(0xC0u8, RegisterSelect::Cmnd, delay)?;
            }
            delay.delay_us(COMMAND_WAIT);
        }
        Ok(())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum RegisterSelect {
    Cmnd = 0u8,
    Data = 1u8,
}

impl Default for RegisterSelect {
    fn default() -> Self {
        RegisterSelect::Data
    }
}

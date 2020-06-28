// New BSD License
//
// Copyright © 2020-present, Michael Cummings <mgcummings@yahoo.com>.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the copyright holder nor the names of its
//       contributors may be used to endorse or promote products derived from
//       this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
//! This is a simple example of how to use library for a raspberry pi.
//!
//! The example was written assuming Raspbian but should work with other Linuxes
//! with little or no change.
//!
//! # Examples
//! To build the example use:
//! ```sh, no_run
//! cargo build --example rpi4bit
//! ```
//! Then to run use:
//! ```sh, no_run
//! sudo ./target/debug/examples rpi4bit
//! ```

use anyhow::{Context, Result};
use hd44780_ntb::{DisplayMode, EntryMode, FunctionMode, GpioDriver, HD44780};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::{Delay, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

const MESSAGE_DELAY: u64 = 2;
/// Some common default GPIO pin numbers
/// This are the same as use in the
/// [SunFounder Super Kit](https://www.sunfounder.com/learn/category/Super_Kit_V2_for_RaspberryPi.html).
const PIN_D4: u64 = 25;
const PIN_D5: u64 = 24;
const PIN_D6: u64 = 23;
const PIN_D7: u64 = 18;
const PIN_E: u64 = 22;
const PIN_RS: u64 = 27;
/// Additional default GPIO pin numbers used for 8 bit.
const PIN_D0: u64 = 21;
const PIN_D1: u64 = 20;
const PIN_D2: u64 = 16;
const PIN_D3: u64 = 12;

/// Entry point of example.
//noinspection DuplicatedCode
fn main() -> Result<()> {
    println!("setup");
    let (rs, e, data) = setup()?;
    println!("data length: {}", data.len());
    let mut lcd = GpioDriver::new(rs, e, data, Delay);
    let dc = Some(DisplayMode::DISPLAY_ON);
    let ems = Some(EntryMode::ENTRY_LEFT | EntryMode::ENTRY_SHIFT_DECREMENT);
    let fm = Some(FunctionMode::LINES_2);
    lcd.init(fm, dc, ems)
        .context("Failed to initialize display instance")?;
    display_loop(&mut lcd)?;
    lcd.return_home()
        .context("Failed to home the display")?;
    println!("destroy");
    destroy()
}

/// Resets GPIO pins as inputs and releases them back to the OS.
//noinspection DuplicatedCode
fn destroy() -> Result<()> {
    let rs = Pin::new(PIN_RS);
    let e = Pin::new(PIN_E);
    rs.set_direction(Direction::In)
        .context("Failed to set direction on register select pin")?;
    e.set_direction(Direction::In)
        .context("Failed to set direction on enable pin")?;
    rs.unexport()
        .context("Failed to un-export register select pin")?;
    e.unexport().context("Failed to un-export enable pin")?;
    let mut pin: Pin;
    let pin_numbers = [
        PIN_D0, PIN_D1, PIN_D2, PIN_D3, PIN_D4, PIN_D5, PIN_D6, PIN_D7,
    ];
    for num in pin_numbers.iter() {
        pin = Pin::new(*num);
        pin.set_direction(Direction::In)
            .context(format!("Failed to set direction on data pin: {}", num))?;
        pin.unexport()
            .context(format!("Failed to export data pin: {}", num))?;
    }
    Ok(())
}

/// Main display loop for messages.
//noinspection DuplicatedCode
fn display_loop(lcd: &mut GpioDriver<Pin, Pin, Pin, Delay>) -> Result<()> {
    for _ in 0..5 {
        lcd.clear_display()
            .context("Failed to clear the display")?;
        let mut message = "May the Rust ...\n... be with you!";
        println!("{}", message);
        lcd.write(message.as_ref())
            .context("Failed to write string")?;
        sleep(Duration::from_secs(MESSAGE_DELAY));
        lcd.clear_display()
            .context("Failed to clear the display")?;
        message = "Ferris says \"Hi\"";
        println!("{}", message);
        lcd.write(message.as_ref())
            .context("Failed to write string")?;
        sleep(Duration::from_secs(MESSAGE_DELAY));
    }
    Ok(())
}

/// Gets the GPIO pins from OS and sets them up as outputs.
//noinspection DuplicatedCode
fn setup() -> Result<(Pin, Pin, Vec<Pin>)> {
    let rs = Pin::new(PIN_RS);
    let e = Pin::new(PIN_E);
    rs.export()
        .context("Failed to export register select pin")?;
    e.export().context("Failed to export enable pin")?;
    rs.set_direction(Direction::High)
        .context("Failed to set direction and level on register select pin")?;
    e.set_direction(Direction::Low)
        .context("Failed to set direction and level on enable pin")?;
    let mut data = Vec::<Pin>::new();
    let pin_numbers = [
        PIN_D0, PIN_D1, PIN_D2, PIN_D3, PIN_D4, PIN_D5, PIN_D6, PIN_D7,
    ];
    for num in pin_numbers.iter() {
        let pin = Pin::new(*num);
        pin.export()
            .context(format!("Failed to export data pin: {}", num))?;
        pin.set_direction(Direction::Out)
            .context(format!("Failed to set direction on data pin: {}", num))?;
        data.push(pin);
    }
    Ok((rs, e, data))
}

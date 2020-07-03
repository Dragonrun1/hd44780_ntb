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
//! This is a simple example of how to use library for a raspberry pi.
//!
//! The example was written assuming Raspbian but should work with other Linuxes
//! with little or no change.
//!
//! The example assumes a 16x2 display but should work with any two line display
//! just some of the display move stuff might not look right on other sizes.
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
use hd44780_ntb::{DisplayMode, EntryMode, FunctionMode, GpioDriver, ShiftMode, HD44780};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::{Delay, Pin};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

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
    let ems = Some(EntryMode::ENTRY_LEFT | EntryMode::ENTRY_SHIFT_CURSOR);
    let fm = Some(FunctionMode::LINES_2);
    lcd.init(fm, dc, ems)
        .context("Failed to initialize display instance")?;
    display_loop(&mut lcd)?;
    lcd.return_home().context("Failed to home the display")?;
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
    for _ in 0..3 {
        // First clear the display.
        lcd.clear_display().context("Failed to clear the display")?;
        // Write first line.
        let mut message = "May the Rust ...";
        println!("{}", message);
        lcd.write(message.as_bytes())
            .context("Failed to write string")?;
        // Move to second line.
        lcd.set_dd_ram_addr(0x40)
            .context("Failed to move to second line")?;
        // Write the second line.
        message = "... be with you!";
        println!("{}", message);
        lcd.write(message.as_bytes())
            .context("Failed to write string")?;
        // Wait a couple seconds so message can be seen.
        sleep(Duration::from_secs(MESSAGE_DELAY));
        // Clear the display again.
        lcd.clear_display().context("Failed to clear the display")?;
        // Write the another longer message message.
        message = "Watch me move right and then left!";
        let remainder = message.len() - 16;
        println!("{}", message);
        lcd.write(message.as_bytes())
            .context("Failed to write string")?;
        // Wait a moment so first part of message can be seen.
        sleep(Duration::from_millis(500));
        // Move the display left.
        for _ in 0..remainder {
            let sm = ShiftMode::DISPLAY_MOVE | ShiftMode::MOVE_LEFT;
            lcd.cursor_shift(sm).context("Failed to shift display")?;
            // Short pause between moves.
            sleep(Duration::from_millis(500));
        }
        // Wait a couple seconds so message can be seen.
        sleep(Duration::from_secs(MESSAGE_DELAY));
        // Move the display back right.
        for _ in 0..remainder {
            let sm = ShiftMode::DISPLAY_MOVE | ShiftMode::MOVE_RIGHT;
            lcd.cursor_shift(sm).context("Failed to shift display")?;
            // Short pause between shifts.
            sleep(Duration::from_millis(500));
        }
        // Wait a couple seconds so message can be seen.
        sleep(Duration::from_secs(MESSAGE_DELAY));
        // Clear the display again.
        lcd.clear_display().context("Failed to clear the display")?;
        // Write the final message.
        message = "Ferris says \"Hi\"";
        println!("{}", message);
        lcd.write(message.as_bytes())
            .context("Failed to write string")?;
        // Wait a couple seconds so message can be seen.
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

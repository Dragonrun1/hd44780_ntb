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
//! Contains the HD44780 display controller command set trait and associated
//! parameter types.

use crate::write::RegisterSelect::Cmnd;
use crate::write::Write;
use crate::{Result, COMMAND_WAIT};
use embedded_hal::blocking::delay::DelayUs;

/// Complete command set for HD44780 display controller.
///
/// Refer to Hitachi HD44780U's data sheet for more information.
pub trait HD44780<D>: Write<D>
where
    D: DelayUs<u16>,
{
    /// Clear the display.
    ///
    /// Clears the DDRAM and sets the address to 0.
    ///
    /// ```edition2018,ignore
    /// lcd.clear_display(delay)?;
    /// ```
    fn clear_display(&mut self, delay: &mut D) -> Result {
        let cmd: u8 = Self::CLEAR_DISPLAY;
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(1600u16);
        Ok(())
    }
    /// Used to shift the display or the cursor to the left or right.
    ///
    /// ```edition2018,ignore
    /// // The same as `ShiftMode::default()`
    /// let sm = ShiftMode::CURSOR_MOVE | ShiftMode::MOVE_RIGHT
    /// lcd.cursor_shift(sm, delay)?;
    /// ```
    fn cursor_shift(&mut self, mode: &ShiftMode, delay: &mut D) -> Result {
        let cmd: u8 = Self::CURSOR_SHIFT | mode.bits();
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(COMMAND_WAIT);
        Ok(())
    }
    /// Set display on/off controls.
    ///
    /// Turns on/off the display, cursor and enables/disables character blinking.
    ///
    /// ```edition2018,ignore
    /// let dm = DisplayMode::DISPLAY_ON | DisplayMode::CURSOR_ON;
    /// lcd.display_control(dm, delay)?;
    /// ```
    fn display_control(&mut self, mode: &DisplayMode, delay: &mut D) -> Result {
        let cmd = Self::DISPLAY_CONTROL | mode.bits();
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(COMMAND_WAIT);
        Ok(())
    }
    /// Sets data cursor direction and display shifting.
    ///
    /// Sets which way the cursor moves after each read/write and if the display
    /// should shift or not.
    ///
    /// ```edition2018,ignore
    /// // EntryMode::default() == EntryMode::ENTRY_SHIFT_INCREMENT
    /// lcd.entry_mode_set(EntryMode::default(), delay)?;
    /// ```
    fn entry_mode_set(&mut self, mode: &EntryMode, delay: &mut D) -> Result {
        let cmd: u8 = Self::ENTRY_MODE_SET | mode.bits();
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(COMMAND_WAIT);
        Ok(())
    }
    /// Used to initialize the interface size (4, 8 bit), display line count, and font.
    ///
    /// Normally would be called only once in a new/constructor type function
    /// for the instance.
    ///
    /// ```edition2018,ignore
    /// let im = InitMode::DATA_4BIT | InitMode::LINES_2;
    /// lcd.function_set(im, delay).?;
    /// ```
    fn function_set(&mut self, mode: &FunctionMode, delay: &mut D) -> Result {
        if mode.contains(FunctionMode::LINES_2) && mode.contains(FunctionMode::DOTS_5X10) {
            todo!("Need to handle illegal combination here")
        }
        let cmd: u8 = Self::FUNCTION_SET | mode.bits();
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(COMMAND_WAIT);
        Ok(())
    }
    /// Reset the cursor to home position.
    ///
    /// Does not clear the DDRAM it just resets the address to 0 and un-shifts
    /// the display.
    ///
    /// ```edition2018,ignore
    /// lcd.return_home(delay)?;
    /// ```
    fn return_home(&mut self, delay: &mut D) -> Result {
        let cmd: u8 = Self::RETURN_HOME;
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(1600u16);
        Ok(())
    }
    /// Set CGRAM(Custom Char) address.
    ///
    /// ```edition2018,ignore
    /// // Start of 2nd character.
    /// let location = 0x09;
    /// lcd.set_cg_ram_addr(location, delay)?;
    /// ```
    fn set_cg_ram_addr(&mut self, address: &u8, delay: &mut D) -> Result {
        let address = address & 0b0011_1111;
        let cmd: u8 = Self::SET_CG_RAM_ADDR | address;
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(COMMAND_WAIT);
        Ok(())
    }
    /// Set DDRAM(Display) address.
    ///
    /// ```edition2018,ignore
    /// // Start of the 2nd line on 2 line display.
    /// let location = 0x40;
    /// lcd.set_dd_ram_addr(location, &mut delay)?;
    /// ```
    fn set_dd_ram_addr(&mut self, address: &u8, delay: &mut D) -> Result {
        let address = address & 0b0111_1111;
        let cmd: u8 = Self::SET_DD_RAM_ADDR | address;
        self.write(cmd, Cmnd, delay)?;
        // Wait for command to finish.
        delay.delay_us(COMMAND_WAIT);
        Ok(())
    }
    fn init(
        &mut self,
        delay: &mut D,
        fs_mode: Option<FunctionMode>,
        dc_mode: Option<DisplayMode>,
        ems_mode: Option<EntryMode>,
    ) -> Result;
    // Commands
    const CLEAR_DISPLAY: u8 = 0x01;
    const CURSOR_SHIFT: u8 = 0x10;
    const DISPLAY_CONTROL: u8 = 0x08;
    const ENTRY_MODE_SET: u8 = 0x04;
    const FUNCTION_SET: u8 = 0x20;
    const RETURN_HOME: u8 = 0x02;
    const SET_CG_RAM_ADDR: u8 = 0x40;
    const SET_DD_RAM_ADDR: u8 = 0x80;
}

// DisplayMode
bitflags! {
    /// Display mode bit flags use by display control command.
    #[derive(Default)]
    pub struct DisplayMode: u8 {
        const BLINK_OFF = 0x00;
        const BLINK_ON = 0x01;
        const CURSOR_OFF = 0x00;
        const CURSOR_ON = 0x02;
        const DISPLAY_OFF = 0x00;
        const DISPLAY_ON = 0x04;
    }
}

// EntryMode
bitflags! {
    pub struct EntryMode: u8 {
        const ENTRY_LEFT = 0x02;
        const ENTRY_RIGHT = 0x00;
        const ENTRY_SHIFT_DECREMENT = 0x00;
        const ENTRY_SHIFT_INCREMENT = 0x01;
    }
}

impl Default for EntryMode {
    fn default() -> Self {
        EntryMode::ENTRY_RIGHT | EntryMode::ENTRY_SHIFT_INCREMENT
    }
}

// FunctionMode
bitflags! {
    #[derive(Default)]
    pub struct FunctionMode: u8 {
        const BITS_4 = 0x00;
        const BITS_8 = 0x10;
        const LINES_1 = 0x00;
        // If 2 lines is chosen then font dot option is ignored and 5x8 is always used.
        const LINES_2 = 0x08;
        const DOTS_5X8 = 0x00;
        // Ignored if 2 line display is chosen.
        const DOTS_5X10 = 0x04;
    }
}

// ShiftMode
bitflags! {
    pub struct ShiftMode: u8 {
        const CURSOR_MOVE = 0x00;
        const DISPLAY_MOVE = 0x08;
        const MOVE_LEFT = 0x00;
        const MOVE_RIGHT = 0x04;
    }
}

impl Default for ShiftMode {
    fn default() -> Self {
        ShiftMode::CURSOR_MOVE | ShiftMode::MOVE_RIGHT
    }
}

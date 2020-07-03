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
//! Contains the HD44780 display controller command set trait and associated
//! parameter types.

// use crate::write::RegisterSelect::Cmnd;
// use crate::write::Write;
use crate::Result;
// use embedded_hal::blocking::delay::DelayUs;
use crate::error::HdError::InvalidLineAndFontMode;
use std::io::Write;

/// Complete command set for HD44780 display controller.
///
/// Refer to Hitachi HD44780 datasheet for more information.
pub trait HD44780: Write {
    //
    // ## Per driver required stuff ##
    //
    /// Delay time constant used to ensure HD44780 can finish processing command.
    ///
    /// Minimum should be > 37µs @ 270KHz per HD44780 datasheet.
    ///
    /// Some commands like [clear_display()] and [return_home()] multiple
    /// this value by some factor to achieve their required delays.
    ///
    /// [clear_display()]: #method.clear_display
    /// [return_home()]: #method.return_home
    ///
    const COMMAND_DELAY: u16;
    /// Provides an interface to send commands through the HD44780 driver.
    ///
    /// This is __NOT__ part of the actual HD44780 command set but a necessary
    /// method to interface with all drivers.
    ///
    /// Provides a place for drivers to do any needed common command processing.
    ///
    /// The implementation of the command set in this trait will use this method
    /// after doing any needed per command processing.
    ///
    /// # Arguments
    /// * `byte` - The command being written to HD44780 hardware.
    /// * `delay` - The expected delay(µs) after sending command so hardware has
    /// time to process it.
    ///
    /// Implementing drivers may ignore `delay` if the under-laying interface
    /// is slower than the command time.
    ///
    /// For example I²C@100KHz should not need to worry about the delay on most
    /// commands as the time to send a single byte along with address etc should
    /// be greater.
    ///
    /// Care should be taken though when handling commands like
    /// [clear_display()], [return_home()], and commands sent from [init()] as
    /// they are expected to take long enough that additional time
    /// will be needed for processing in the HD44780.
    ///
    /// [clear_display()]: #method.clear_display
    /// [return_home()]: #method.return_home
    /// [init()]: #method.init
    ///
    fn command(&mut self, byte: u8, delay: u16) -> Result;
    /// Used to initialize the display into a know state.
    ///
    /// Normally the display controller's power on reset sets up the display
    /// into a known state.
    /// In cases where the reset hasn't done so correctly or another program has
    /// left the display in an unknown state this method can be used to get the
    /// display into a known state.
    /// This method can also be used to switch between 4 and 8 bit (pin) data
    /// bus modes.
    ///
    /// The best description I found of how to do the necessary 4 or 8 bit mode
    /// selection can be found in the Wikipedia [HD44780] article.
    ///
    /// [HD44780]: https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller#Mode_selection
    ///
    fn init<FSM, DCM, EMSM>(&mut self, fs_mode: FSM, dc_mode: DCM, ems_mode: EMSM) -> Result
    where
        FSM: Into<Option<FunctionMode>>,
        DCM: Into<Option<DisplayMode>>,
        EMSM: Into<Option<EntryMode>>;
    //
    // ## Shouldn't need to change these in driver implementations. ##
    //
    /// Clear the display.
    ///
    /// From HD44780 datasheet:
    /// Clears entire display and sets DD RAM address 0 in address counter.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// lcd.clear_display()?;
    /// ```
    fn clear_display(&mut self) -> Result {
        let cmd: u8 = Self::CLEAR_DISPLAY;
        // Per HD44780 datasheet clear takes 1.52ms.
        // 37µs * 42 = 1.554ms for a little fudge factor.
        let delay = Self::COMMAND_DELAY * 42;
        self.command(cmd, delay)
    }
    /// Used to shift the display or the cursor to the left or right.
    ///
    /// From HD44780 datasheet:
    /// Moves cursor and shifts display without changing DD RAM contents.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// // The same as `ShiftMode::default()`
    /// let sm = ShiftMode::CURSOR_MOVE | ShiftMode::MOVE_RIGHT
    /// lcd.cursor_shift(sm)?;
    /// ```
    fn cursor_shift(&mut self, mode: ShiftMode) -> Result {
        let cmd: u8 = Self::CURSOR_SHIFT | mode.bits();
        self.command(cmd, Self::COMMAND_DELAY)
    }
    /// Set display on/off controls.
    ///
    /// From HD44780 datasheet:
    /// Sets entire display on/off,cursor on/off , and blinking of cursor
    /// position character.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// let dm = DisplayMode::DISPLAY_ON | DisplayMode::CURSOR_ON;
    /// lcd.display_control(dm)?;
    /// ```
    fn display_control(&mut self, mode: DisplayMode) -> Result {
        let cmd = Self::DISPLAY_CONTROL | mode.bits();
        self.command(cmd, Self::COMMAND_DELAY)
    }
    /// Sets data cursor direction and display shifting.
    ///
    /// From HD44780 datasheet:
    /// Sets cursor move direction and specifies display shift.
    /// These operations are performed during data write and read.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// // EntryMode::default() == EntryMode::ENTRY_SHIFT_INCREMENT
    /// lcd.entry_mode_set(EntryMode::default())?;
    /// ```
    fn entry_mode_set(&mut self, mode: EntryMode) -> Result {
        let cmd: u8 = Self::ENTRY_MODE_SET | mode.bits();
        self.command(cmd, Self::COMMAND_DELAY)
    }
    /// Used to initialize the interface size (4, 8 bit), display line count, and font.
    ///
    /// Normally would be called only once in a new/constructor type function
    /// for the instance.
    ///
    /// From HD44780 datasheet:
    /// Sets interface data length, number of display lines, and character font.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// let im = InitMode::DATA_4BIT | InitMode::LINES_2;
    /// lcd.function_set(im).?;
    /// ```
    ///
    /// # Errors
    /// Returns an error when 2 lines and 5x10 font modes are selected together
    /// as that is not supported by the hardware.
    fn function_set(&mut self, mode: FunctionMode) -> Result {
        if mode.contains(FunctionMode::LINES_2) && mode.contains(FunctionMode::DOTS_5X10) {
            return Err(InvalidLineAndFontMode);
        }
        let cmd: u8 = Self::FUNCTION_SET | mode.bits();
        self.command(cmd, Self::COMMAND_DELAY)
    }
    /// Reset the cursor to home position.
    ///
    /// From HD44780 datasheet:
    /// Sets DD RAM address 0 in address counter.
    /// Also returns display from being shifted to original position.
    /// DD RAM contents remain unchanged.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// lcd.return_home()?;
    /// ```
    fn return_home(&mut self) -> Result {
        let cmd: u8 = Self::RETURN_HOME;
        // Per HD44780 datasheet home takes 1.52ms.
        // 37µs * 42 = 1.554ms for a little fudge factor.
        let delay = Self::COMMAND_DELAY * 42;
        self.command(cmd, delay)
    }
    /// Set CG RAM(Custom Char) address.
    ///
    /// From HD44780 datasheet:
    /// Sets CG RAM address.
    /// CG RAM data is sent and received after this setting.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// // Start of 2nd character.
    /// let location = 0x09;
    /// lcd.set_cg_ram_addr(location)?;
    /// ```
    fn set_cg_ram_addr(&mut self, address: u8) -> Result {
        let address = address & 0b0011_1111;
        let cmd: u8 = Self::SET_CG_RAM_ADDR | address;
        self.command(cmd, Self::COMMAND_DELAY)
    }
    /// Set DD RAM(Display) address.
    ///
    /// From HD44780 datasheet:
    /// Sets DD RAM address.
    /// DD RAM data is sent and received after this setting.
    ///
    /// # Examples
    /// ```edition2018,ignore
    /// // Start of the 2nd line on 2 line display.
    /// let location = 0x40;
    /// lcd.set_dd_ram_addr(location)?;
    /// ```
    fn set_dd_ram_addr(&mut self, address: u8) -> Result {
        let address = address & 0b0111_1111;
        let cmd: u8 = Self::SET_DD_RAM_ADDR | address;
        self.command(cmd, Self::COMMAND_DELAY)
    }
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
    /// Display mode bit flags used with [display_control()] command.
    ///
    /// [display_control()]: trait.HD44780.html#method.display_control
    ///
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
    /// Entry mode bit flags used with [entry_mode_set()] command.
    ///
    /// [entry_mode_set()]: trait.HD44780.html#method.entry_mode_set
    ///
    pub struct EntryMode: u8 {
        const ENTRY_LEFT = 0x02;
        const ENTRY_RIGHT = 0x00;
        const ENTRY_SHIFT_CURSOR = 0x00;
        const ENTRY_SHIFT_DISPLAY = 0x01;
    }
}

impl Default for EntryMode {
    fn default() -> Self {
        EntryMode::ENTRY_LEFT | EntryMode::ENTRY_SHIFT_CURSOR
    }
}

// FunctionMode
bitflags! {
    /// Function mode bit flags used with [function_set()] command.
    ///
    /// [function_set()]: trait.HD44780.html#method.function_set
    ///
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
    /// Shift mode bit flags used with [cursor_shift()] command.
    ///
    /// [cursor_shift()]: trait.HD44780.html#method.cursor_shift
    ///
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

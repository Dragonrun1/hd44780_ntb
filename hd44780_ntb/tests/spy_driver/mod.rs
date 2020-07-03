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

use bitflags::_core::convert::Into as bfInto;
use hd44780_ntb::{DisplayMode, EntryMode, FunctionMode, Result, HD44780};
use std::convert::Into;
use std::io::{Result as IOResult, Write};
use std::time::Instant;

#[derive(Debug, Default)]
pub struct SpyDriver {
    commands: Vec<Command>,
    init_command: Option<(
        Instant,
        Option<FunctionMode>,
        Option<DisplayMode>,
        Option<EntryMode>,
    )>,
    writes: Vec<(Instant, Vec<u8>)>,
}

impl Write for SpyDriver {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.writes.push((Instant::now(), buf.into()));
        Ok(buf.len())
    }
    fn flush(&mut self) -> IOResult<()> {
        Ok(())
    }
}

impl HD44780 for SpyDriver {
    const COMMAND_DELAY: u16 = 0;
    fn command(&mut self, byte: u8, delay: u16) -> Result {
        self.commands.push(Command {
            when: Instant::now(),
            byte,
            delay,
        });
        Ok(())
    }
    fn init<FSM, DCM, EMSM>(&mut self, fs_mode: FSM, dc_mode: DCM, ems_mode: EMSM) -> Result
    where
        FSM: Into<Option<FunctionMode>>,
        DCM: Into<Option<DisplayMode>>,
        EMSM: Into<Option<EntryMode>>,
    {
        self.init_command = Some((
            Instant::now(),
            fs_mode.into(),
            dc_mode.into(),
            ems_mode.into(),
        ));
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Command {
    when: Instant,
    byte: u8,
    delay: u16,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum CommandKind {
    Init,
    ClearDisplay,
    ReturnHome,
    EntryModeSet,
    DisplayControl,
    CursorShift,
    FunctionSet,
    SetCgRamAddr,
    SetDdRamAddr,
}

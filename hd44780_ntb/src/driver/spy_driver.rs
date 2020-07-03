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
//! Contains a test driver and associated structs that does __NOT__ connect to any hardware.

use crate::{DisplayMode, EntryMode, FunctionMode, Result, HD44780};
use std::io::{Result as IOResult, Write};
use std::time::Instant;

/// A very basic testing driver that records arguments given for commands and writes.
#[derive(Debug, Default)]
pub struct SpyDriver {
    /// List of commands sent to driver.
    pub commands: Vec<Command>,
    /// Init command is handled differently because of extra arguments.
    ///
    /// Since init isn't a single command to the hardware but a special sequence
    /// of commands and mode settings that is used to reset the hardware it is
    /// handled differently here as well.
    pub init_command: Option<(
        Instant,
        Option<FunctionMode>,
        Option<DisplayMode>,
        Option<EntryMode>,
    )>,
    /// Any non-command data writes.
    ///
    /// Both writes to CG RAM and DD RAM end up here as the actual hardware
    /// determines which is being written by proceeding command that was given.
    pub writes: Vec<(Instant, Vec<u8>)>,
}

impl SpyDriver {
    pub fn new() -> Self {
        SpyDriver {
            commands: vec![],
            init_command: None,
            writes: vec![],
        }
    }
}

impl Write for SpyDriver {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.writes.push((Instant::now(), Vec::from(buf)));
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

/// Structure in [SpyDriver] to record commands.
///
/// [SpyDriver]: struct.SpyDriver.html
///
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Command {
    when: Instant,
    byte: u8,
    delay: u16,
}

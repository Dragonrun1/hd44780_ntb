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
//! A common set of error and result type used in the library.

use thiserror::Error;

/// Provides a shared set of error types.
#[derive(Error, Debug)]
pub enum HdError {
    /// Used if data bus given is not 4 or 8 bits long.
    #[error("Data must be 4 or 8 OutputPins")]
    IncorrectDataLen,
    /// Used if given output GPIO pin can not be set.
    #[error("Could not set {0} output pin")]
    SetOutputPin(&'static str),
    #[error("Write failed")]
    Write(#[from] std::io::Error),
}

impl From<HdError> for std::io::Error {
    fn from(he: HdError) -> Self {
        he.into()
    }
}

/// Common result used as return type from functions in library.
pub type Result = std::result::Result<(), HdError>;

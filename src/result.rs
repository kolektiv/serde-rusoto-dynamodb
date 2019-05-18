// Result Types

// -----------------------------------------------------------------------------

// Error

// Implementation of a very simple combined Error type for both Serialization
// and Deserialization, making error handling simpler, and with a convenient
// constructor function.

use serde::{de::Error as SerdeDeError, ser::Error as SerdeSerError};
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as StdFmtResult},
};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> StdFmtResult {
        Display::fmt(&self.message, f)
    }
}

impl SerdeDeError for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error {
            message: format!("{}", msg),
        }
    }
}

impl SerdeSerError for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error {
            message: format!("{}", msg),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

// -----------------------------------------------------------------------------

// Result

// A simple Result type with unary generic arity, pre-setting the Error generic
// parameter to be our simple custom Error type.

use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

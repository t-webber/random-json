//! Module for handling errors.

extern crate alloc;
use alloc::fmt;
use core::result;
use std::io;

/// Custom error types for the crate, for better error handling.
#[derive(Debug)]
#[expect(dead_code, reason = "used as return of main")]
pub enum Error {
    /// Error from `serde_json` when deserializing JSON data to a pretty string.
    DeserializeJson(serde_json::Error),
    /// Error from the dialoguer crate during user interaction.
    DialogueIo(dialoguer::Error),
    /// File could not be found or accessed.
    FileNotFound {
        /// The path to the file that could not be found
        file: String,
        /// The underlying I/O error that caused the failure
        error: io::Error,
    },
    /// The data type provided to the generator isn't recognised.
    InvalidDataType(String),

    /// File exists but is in an invalid format, that makes the deserialization
    /// fail.
    InvalidFile {
        /// The path to the file with invalid format
        file: String,
        /// The JSON parsing error that occurred
        error: serde_json::Error,
    },
    /// Invalid schema type specified.
    ///
    /// This means a unsupported JSON feature was present, such as booleans,
    /// undefined, numbers, etc.
    InvalidSchemaType(String),
    /// Error occurred while writing JSON-format generated data to output.
    JsonWrite(fmt::Error),
    /// User tried to use both `--list` and `--interactive` options, which is
    /// not allowed.
    ListAndInteractiveConflict,
    /// General I/O error from terminal interaction.
    TerminalIo(io::Error),
}

impl Error {
    /// Helper function to create an [`Self::InvalidFile`] error with a specific
    /// file name.
    pub fn invalid_file(file: String) -> impl FnOnce(serde_json::Error) -> Self {
        |error: serde_json::Error| Self::InvalidFile { file, error }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::TerminalIo(value)
    }
}

impl From<dialoguer::Error> for Error {
    fn from(value: dialoguer::Error) -> Self {
        Self::DialogueIo(value)
    }
}

/// Convenient result type alias for this crate.
pub type Res<T = (), E = Error> = result::Result<T, E>;

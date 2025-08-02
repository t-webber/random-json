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
    InvalidJson(serde_json::Error),
    /// Invalid schema type specified.
    ///
    /// This means a unsupported JSON feature was present, such as booleans,
    /// undefined, numbers, etc.
    InvalidSchemaType(String),
    /// Error occurred while writing JSON-format generated data to output.
    JsonWriteString(fmt::Error),
    /// User tried to use both `--list` and `--interactive` options, which is
    /// not allowed.
    ListAndInteractiveConflict,
    /// General I/O error from terminal interaction.
    TerminalIo(io::Error),
}

impl Error {
    /// Get a nice and user-friendly error in case of failures.
    pub fn display(&self, debug: bool) -> String {
        let repr = format!("\x1b[31mError:\x1b[0m \x1b[33m{}\x1b[0m\n", self.repr());

        if debug {
            format!(
                "{repr}\nError type: {self:?}\n\x1b[3mIf you think this is a bug, please report it here: https://github.com/t-webber/fake-json/issues/new. Thanks!\x1b[0m",
            )
        } else {
            format!("{repr}\nUse the --debug flag for more information",)
        }
    }

    /// Helper function to create an [`Self::FileNotFound`] error with a
    /// specific file name.
    pub fn file_not_found(file: String) -> impl FnOnce(io::Error) -> Self {
        |error: io::Error| Self::FileNotFound { file, error }
    }

    /// Get a nice and user-friendly error in case of failures.
    fn repr(&self) -> String {
        match self {
            Self::JsonWriteString(_) |
            Self::DeserializeJson(_) => "Internal error occured.".to_owned(),
        Self::FileNotFound { file, .. } => format!("{file} couldn't be found, ensure it exists and is accessible! You can also use the --json option to "),
            Self::InvalidDataType(data_type) => format!("{data_type} isn't a valid data type. You can use --list to display all the valid data types, or --interactive to fuzzy search in all the data types!"),
            Self::InvalidJson (_) => "The provided JSON wasn't in a valid JSON format.".to_owned(),
            Self::InvalidSchemaType(invalid_type) => format!("your schema contains {invalid_type} which is not supported. The values must be strings with the name of the data type, or an array or an object of those strings."),
            Self::ListAndInteractiveConflict => "You can't use --interface (-i) and --list (-l) at the same time! Using solely -i will give you an interactive list from which you can choose the data types!".to_owned(),
            Self::DialogueIo(_) |
            Self::TerminalIo(_) =>
                "An error occurred whilst interacting with your terminal. ".to_owned(),
        }
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

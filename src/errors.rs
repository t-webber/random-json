//! Module for handling errors.

extern crate alloc;
use alloc::fmt;
use core::num::TryFromIntError;
use core::result;
use std::io;

/// Explanatory text on how to use arrays.
const ARRAY_SYNTAX: &str = r#"

An array must contain a data_type as a first argument, and may contain two more arguments: if it contains 2 arguments, the second must be an integer, and indicated the size of the produced array. If it contains 3 arguments, the second and third must be integers and are the bounds of the array.

Examples:
["FreeEmail"]               // produces a random number of emails
["FirstName", 2]            // produces an array containing 2 first name
["LicencePlate", 1, 10]     // produces an array with between 1 and 9 licence plates
"#;

/// Custom error types for the crate, for better error handling.
#[derive(Debug)]
#[expect(dead_code, reason = "used as return of main")]
pub enum Error {
    /// Failed to convert JSON number to usize
    ArrayInvalidLength {
        /// Conversion error
        error: TryFromIntError,
        /// Original number parsed from the JSON
        original: u64,
    },
    /// First argument of array was missing
    ArrayMissingDataType,
    /// Error from the dialoguer crate during user interaction.
    DialogueIo(dialoguer::Error),
    /// User parsed 2 custom data types with the same name.
    DuplicateDataType(String),
    /// Failed to parse a JSON node into an integer
    ExpectedInteger(serde_json::Value),
    /// Provided a user defined data type with no values.
    FakerDefEmpty,
    /// Missing colon in user defined data type.
    FakerDefMissingColon,
    /// Too many colons in user defined data type.
    FakerDefTooManyColons,
    /// File could not be found or accessed.
    FileNotFound {
        /// The path to the file that could not be found
        file: String,
        /// The underlying I/O error that caused the failure
        error: io::Error,
    },
    /// The data type provided to the generator isn't recognised.
    InvalidDataType(String),
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
    /// Faled to parse a JSON number into an unsigned integer
    NumberNotAnInteger(serde_json::Number),
    /// File exists but is in an invalid format, that makes the deserialization
    /// fail.
    SerdeDeserializeJson(serde_json::Error),
    /// Error from `serde_json` when deserializing JSON data to a pretty string.
    SerdeSerializeJson(serde_json::Error),
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
                            Self::SerdeSerializeJson(_) => "Internal error occured.".to_owned(),
            Self::FileNotFound { file, .. } => format!("{file} couldn't be found, ensure it exists and is accessible! You can also use the --json option to "),
            Self::InvalidDataType(data_type) => format!("{data_type} isn't a valid data type.\nUse -l to list the valid data types, -i to fuzzy search the data types, or -u to define your own data types!"),
            Self::SerdeDeserializeJson (_) => "The provided JSON wasn't in a valid JSON format.".to_owned(),
            Self::InvalidSchemaType(invalid_type) => format!("your schema contains {invalid_type} which is not supported. The values must be strings with the name of the data type, or an array or an object of those strings."),
            Self::ListAndInteractiveConflict => "You can't use --interface (-i) and --list (-l) at the same time! Using solely -i will give you an interactive list from which you can choose the data types!".to_owned(),
            Self::DialogueIo(_) |
                            Self::TerminalIo(_) =>
                                "An error occurred whilst interacting with your terminal. ".to_owned(),
            Self::ArrayMissingDataType => format!("invalid array syntax: missing data type.{ARRAY_SYNTAX}"),
            Self::ExpectedInteger(value) => format!("invalid aray syntax: expected integer, found {value}.{ARRAY_SYNTAX}"),
            Self::NumberNotAnInteger(number) => format!("invalid array syntax: expected integer, found {number}.{ARRAY_SYNTAX}"),
            Self::ArrayInvalidLength{original, ..} => format!("{original} is too large to be the length of an array.{ARRAY_SYNTAX}"),
            Self::FakerDefMissingColon => ("Data types must be given with the format -u 'DataTypeName:Value1|Value2|Value3'").to_owned(),
            Self::FakerDefTooManyColons => "To pass multiple user-defined data types, pass multiple times the `-u` options: -u 'Type1:Value1|Value2' -u 'Type2:Value3|Value4'".to_owned(),
            Self::FakerDefEmpty => "The provided data type has no values, use -u 'DataTypeName:Value1|Value2|Value3'".to_owned(),
            Self::DuplicateDataType(data_type) => format!("{data_type} was provided twice with the `-u` option. Please use different names."),
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

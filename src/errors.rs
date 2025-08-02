use std::{io, result};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Unreachable(&'static str),
    DialogIo(dialoguer::Error),
    Io(io::Error),
    InvalidDataType(String),
    FileNotFound {
        file: String,
        error: io::Error,
    },
    InvalidFile {
        file: String,
        error: serde_json::Error,
    },
}

impl Error {
    pub fn invalid_file(file: String) -> impl FnOnce(serde_json::Error) -> Self {
        |error: serde_json::Error| Self::InvalidFile { file, error }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<dialoguer::Error> for Error {
    fn from(value: dialoguer::Error) -> Self {
        Self::DialogIo(value)
    }
}

pub type Res<T = (), E = Error> = result::Result<T, E>;

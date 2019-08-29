use std::error::Error as StdError;
use std::{fmt, io, result};

use clap;
use reqwest::{self, UrlError};
use serde_json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Argument(String),
    Http(reqwest::Error),
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    Io(io::Error),
    Json(serde_json::Error),
    Parser(clap::Error),
    ToStr(reqwest::header::ToStrError),
    Url(UrlError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.message)
    }
}

impl Error {
    fn new(kind: ErrorKind) -> Self {
        let message = match kind {
            ErrorKind::Argument(ref arg) => format!("An invalid argument was provided: {}", arg),
            ErrorKind::Http(ref err) => format!(
                "An error occurred while making an HTTP request: {}",
                err.description()
            ),
            ErrorKind::Io(ref err) => format!("An I/O error occured: {}", err.description()),
            ErrorKind::InvalidHeaderName(ref err) => format!(
                "An invalid header name was specified: {}",
                err.description()
            ),
            ErrorKind::InvalidHeaderValue(ref err) => format!(
                "An invalid header name was specified: {}",
                err.description()
            ),
            ErrorKind::Json(ref err) => format!(
                "An error occurred while parsing a JSON argument: {}",
                err.description()
            ),
            ErrorKind::Parser(ref err) => format!(
                "An error occurred while parsing the command-line arguments: {}",
                err.description()
            ),
            ErrorKind::ToStr(ref err) => format!(
                "An HTTP response header could not be converted to a string: {}",
                err.description()
            ),
            ErrorKind::Url(ref err) => format!(
                "An error occurred while parsing the URL: {}",
                err.description()
            ),
        };

        Error {
            message: message,
            kind: kind,
        }
    }

    pub fn argument_error(arg: &str) -> Self {
        Error::new(ErrorKind::Argument(String::from(arg)))
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self.kind {
            ErrorKind::Argument(_) => None,
            ErrorKind::Http(ref err) => Some(err),
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::InvalidHeaderName(ref err) => Some(err),
            ErrorKind::InvalidHeaderValue(ref err) => Some(err),
            ErrorKind::Json(ref err) => Some(err),
            ErrorKind::Parser(ref err) => Some(err),
            ErrorKind::ToStr(ref err) => Some(err),
            ErrorKind::Url(ref err) => Some(err),
        }
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::new(ErrorKind::Parser(err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<reqwest::header::InvalidHeaderName> for Error {
    fn from(err: reqwest::header::InvalidHeaderName) -> Error {
        Error::new(ErrorKind::InvalidHeaderName(err))
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Error {
        Error::new(ErrorKind::InvalidHeaderValue(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::new(ErrorKind::Json(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::new(ErrorKind::Http(err))
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(err: reqwest::header::ToStrError) -> Error {
        Error::new(ErrorKind::ToStr(err))
    }
}

impl From<UrlError> for Error {
    fn from(err: UrlError) -> Error {
        Error::new(ErrorKind::Url(err))
    }
}

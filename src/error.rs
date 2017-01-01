use std::{fmt, io, result};
use std::error::Error as StdError;

use clap;
use hyper;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    Http(hyper::error::Error),
    Io(io::Error),
    Parser(clap::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.message)
    }
}

impl Error {
    fn new(kind: ErrorKind) -> Self {
        let message = match kind {
            ErrorKind::Http(ref err) => {
                format!("An error occurred while making an HTTP request: {}",
                        err.description())
            }
            ErrorKind::Io(ref err) => format!("An I/O error occured: {}", err.description()),
            ErrorKind::Parser(ref err) => {
                format!("An error occurred while parsing the command-line arguments: {}",
                        err.description())
            }
        };

        Error {
            message: message,
            kind: kind,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&StdError> {
        match self.kind {
            ErrorKind::Http(ref err) => Some(err),
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Parser(ref err) => Some(err),
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::new(ErrorKind::Http(err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::new(ErrorKind::Parser(err))
    }
}

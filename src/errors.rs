use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use openssl::ssl::error::SslError;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Io(IoError),
    Ssl(SslError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Internal(ref message) => f.write_str(message),
            Error::Io(ref err) => err.fmt(f),
            Error::Ssl(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Internal(ref message) => message,
            Error::Io(ref err) => err.description(),
            Error::Ssl(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Ssl(ref err) => Some(err),
            Error::Internal(_) => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error::Ssl(err)
    }
}
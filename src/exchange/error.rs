use super::super::reqwest::Error as RequestError;
extern crate serde_json;
use self::serde_json::error::Error as SerdeError;

use std::result;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DeserializationError,
    SerdeError(SerdeError),
    RequestError(RequestError),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RequestError(ref err) => err.fmt(f),
            Error::SerdeError(ref err) => err.fmt(f),
            Error::DeserializationError => {
                write!(f, "Failed to deserialize object returned from exchange")
            }
        }
    }
}

use std::error::Error as StdError;
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RequestError(ref err) => err.description(),
            Error::SerdeError(ref err) => err.description(),
            Error::DeserializationError => "Failed to deserialize",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::RequestError(ref err) => Some(err),
            Error::SerdeError(ref err) => Some(err),
            Error::DeserializationError => None,
        }
    }
}


impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::RequestError(err)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::SerdeError(err)
    }
}

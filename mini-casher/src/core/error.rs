use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use std::string::FromUtf8Error;
use std::sync::{MutexGuard, PoisonError};
use bytes::Bytes;
use thiserror::Error;


pub type CashError = crate::core::error::Error;


#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    SocketRead(String),
    SocketWrite(String),
    Incomplete,
    Protocol(String),
    CommandParse(String),
    Storage(String),
}


impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::Protocol(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::SocketRead(value.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::CommandParse(value.to_string())
    }
}

impl From<PoisonError<std::sync::MutexGuard<'_, HashMap<String, bytes::Bytes>>>> for Error {
    fn from(value: PoisonError<MutexGuard<'_, HashMap<String, Bytes>>>) -> Self {
        Error::Storage(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match &self {
            Error::SocketRead(value) => value,
            Error::SocketWrite(value) => value,
            Error::Incomplete => "",
            Error::Protocol(value) => value,
            Error::CommandParse(value) => value,
            Error::Storage(value) => value,
        };

        write!(f, "{message}")
    }
}
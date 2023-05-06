use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use thiserror::Error;

pub type ServerError = crate::error::Error;


#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    Hyper(String),
    SerdeJson(String),
    Cash(String)
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Error::Hyper(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJson(value.to_string())
    }
}

impl From<mini_casher::core::error::Error> for Error {
    fn from(value: mini_casher::core::error::Error) -> Self {
        Error::Cash(value.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::Cash(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}
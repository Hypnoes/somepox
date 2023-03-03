use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GeneralError(String);

impl Error for GeneralError {}

impl Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for GeneralError {
    fn from(value: std::io::Error) -> Self {
        GeneralError(format!("{}", value))
    }
}

impl From<String> for GeneralError {
    fn from(value: String) -> Self {
        GeneralError(value)
    }
}

impl From<&str> for GeneralError {
    fn from(value: &str) -> Self {
        GeneralError(value.to_string())
    }
}

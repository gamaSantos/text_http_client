use std::fmt::{self};

use surf::Error;

#[derive(Debug)]
pub enum RequestError {
    CouldNotReadFile,
    TomlParserError { message: String },
    BuildError { property_name: String },
    HttpError { inner: Error },
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::CouldNotReadFile => write!(f, "could not read the file"),
            RequestError::TomlParserError { message } => {
                write!(f, "could not parse the file {0}", message)
            }
            RequestError::BuildError {
                property_name: field,
            } => write!(f, "missing {0} information", field),
            RequestError::HttpError { inner } => write!(f, "{inner}"),
        }
    }
}

impl std::error::Error for RequestError {}

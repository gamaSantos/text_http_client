use std::fmt;

#[derive(Debug)]
pub enum RequestError {
    CouldNotReadFile,
    TomlParserError { message: String },
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::CouldNotReadFile => write!(f, "could not read the file"),
            RequestError::TomlParserError { message: _ } => {
                write!(f, "could not parse the file")
            }
        }
    }
}

impl std::error::Error for RequestError {}

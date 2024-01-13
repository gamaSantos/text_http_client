use std::{collections::HashMap, fmt, fs, path::Path};

use clap::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestMessage {
    header: Option<RequestHeader>,
    body: Option<String>,
}

#[derive(Deserialize)]
pub struct RequestHeader {
    method: Option<String>,
    host: Option<String>,
    path: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl RequestMessage {
    pub fn from_file(file_path: &str) -> Result<RequestMessage, RequestMessageParseError> {
        let file_path = Path::new(file_path);
        if file_path.exists() == false {
            return Err(RequestMessageParseError::FileNotFound);
        }
        let read_file_result = fs::read_to_string(file_path);

        let file_text = match read_file_result {
            Ok(t) => t,
            Err(_) => return Err(RequestMessageParseError::CouldNotReadFile),
        };

        let parsed = toml::from_str::<RequestMessage>(&file_text);

        return parsed.map_err(|e| -> RequestMessageParseError {
            return RequestMessageParseError::TomlParserError {
                message: e.message().to_string(),
            };
        });
    }
}
#[derive(Debug)]
pub enum RequestMessageParseError {
    FileNotFound,
    CouldNotReadFile,
    TomlParserError { message: String },
}

impl fmt::Display for RequestMessageParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestMessageParseError::FileNotFound => write!(f, "file not found"),
            RequestMessageParseError::CouldNotReadFile => write!(f, "could not read the file"),
            RequestMessageParseError::TomlParserError { message: _ } => {
                write!(f, "could not parse the file")
            }
        }
    }
}

impl std::error::Error for RequestMessageParseError {}

#[test]
fn from_file_should_return_file_not_found_when_file_does_not_exist() {
    let invalid_path = "./replaca_with_random/new_guid.toml";
    let maybe_message = RequestMessage::from_file(invalid_path);
    assert!(maybe_message.is_err());

    match maybe_message {
        Ok(_) => panic!("should never be executed?"),
        Err(e) => assert!(matches!(e, RequestMessageParseError::FileNotFound)),
    };
}

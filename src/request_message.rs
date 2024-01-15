use std::collections::HashMap;

use serde::Deserialize;

use crate::request_error::RequestError;

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
    pub fn from_text(file_text: &str) -> Result<RequestMessage, RequestError> {
        let parsed = toml::from_str::<RequestMessage>(&file_text);

        return parsed.map_err(|e| -> RequestError {
            return RequestError::TomlParserError {
                message: e.message().to_string(),
            };
        });
    }
}

#[test]
fn from_text_should_return_parser_error_when_text_is_not_toml() {
    let input = "invalid toml";
    let maybe_message = RequestMessage::from_text(input);
    assert!(maybe_message.is_err());

    match maybe_message {
        Ok(_) => panic!("should never be executed?"),
        Err(e) => assert!(matches!(e, RequestError::TomlParserError { message: _ })),
    };
}

#[test]
fn from_text_should_parse_body() {
    let input = "body = 'test body'";
    let maybe_message = RequestMessage::from_text(input);
    assert!(maybe_message.is_ok_and(|x| x.body.is_some_and(|h| h == "test body".to_string())));
}


#[test]
fn from_text_should_parse_header() {
    let input = "body = 'test body'";
    let maybe_message = RequestMessage::from_text(input);
    assert!(maybe_message.is_ok_and(|x| x.body.is_some_and(|h| h == "test body".to_string())));
}

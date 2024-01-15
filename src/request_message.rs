use std::collections::HashMap;

use serde::Deserialize;

use crate::request_error::RequestError;

#[derive(Deserialize)]
pub struct RequestMessage {
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
    method: Option<String>,
    host: Option<String>,
    path: Option<String>,
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
fn from_text_should_parse_values() {
    let input = r#"
        method = "GET"
        host = "http://localhost:5000"
        path = "/"
        body = '{"fake_json":"value"}'
        
        [headers]
        accept = "application/json"
        authorization = "simple_token"

        "#;
    let maybe_message = RequestMessage::from_text(input);
    assert!(maybe_message.is_ok());

    let message = maybe_message.unwrap();
    assert!(message.method.is_some_and(|x| x == "GET"));
    assert!(message.host.is_some_and(|x| x == "http://localhost:5000"));
    assert!(message.path.is_some_and(|x| x == "/"));
    assert!(message.body.is_some_and(|x| x == r#"{"fake_json":"value"}"#));
    
    assert!(message
        .headers
        .is_some_and(|h| h.contains_key("authorization")));

}

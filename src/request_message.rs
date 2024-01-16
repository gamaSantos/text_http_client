use std::collections::HashMap;

use serde::Deserialize;

use crate::request_error::RequestError;

#[derive(Debug, Deserialize)]
pub struct RequestMessage {
    pub method: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
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

    pub fn merge_with(&self, new_message: RequestMessage) -> RequestMessage {
        fn increment_header(
            opt_headers: Option<HashMap<String, String>>,
            mut target_headers: HashMap<String, String>,
        ) -> HashMap<String, String> {
            if let Some(cur_headers) = opt_headers {
                for (k, v) in cur_headers {
                    target_headers.insert(k, v);
                }
            }
            return target_headers;
        }

        let method = new_message.method.or(self.method.clone());
        let host = new_message.host.or(self.host.clone());
        let path = new_message.path.or(self.path.clone());
        let body = new_message.body.or(self.body.clone());

        let copied_values = increment_header(self.headers.clone(), HashMap::new());
        let incremented = increment_header(new_message.headers, copied_values);

        let headers = Some(incremented);
        return RequestMessage {
            method,
            host,
            path,
            body,
            headers,
        };
    }
}

#[test]
fn from_text_should_return_parser_error_when_text_is_not_toml() {
    let input = "invalid toml";
    let maybe_message = RequestMessage::from_text(input);
    assert!(maybe_message.is_err_and(|e| matches!(e, RequestError::TomlParserError { message: _ })));

    // match maybe_message {
    //     Ok(_) => panic!("should never be executed?"),
    //     Err(e) => assert!(matches!(e, RequestError::TomlParserError { message: _ })),
    // };
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
    assert!(message
        .body
        .is_some_and(|x| x == r#"{"fake_json":"value"}"#));

    assert!(message
        .headers
        .is_some_and(|h| h.contains_key("authorization")));
}

#[test]
fn increment_should_replace_value() {
    let input_a = r#"
        method = "GET"
        host = "http://localhost:5000"
        "#;
    let inptu_b = r#"
    method = "POST"
    "#;
    let message_a =
        RequestMessage::from_text(input_a).expect("sample message should have been parsed");
    let message_b =
        RequestMessage::from_text(inptu_b).expect("sample message should have been parsed");

    let merged_message = message_a.merge_with(message_b);
    assert!(merged_message.method.is_some_and(|x| x == "POST"))
}

#[test]
fn increment_should_add_value() {
    let input_a = r#"
        method = "GET"
        host = "http://localhost:5000"
        
        [headers]
        accept = "application/json"
        "#;
    let inptu_b = r#"
    path = "/resource"

    [headers]
    authorization = "simple_token"
    "#;
    let message_a =
        RequestMessage::from_text(input_a).expect("sample message should have been parsed");
    let message_b =
        RequestMessage::from_text(inptu_b).expect("sample message should have been parsed");

    let merged_message: RequestMessage = message_a.merge_with(message_b);
    assert!(merged_message.path.is_some_and(|x| x == "/resource"));

    assert!(merged_message
        .headers
        .is_some_and(|h| h.contains_key("authorization")));
}

#[test]
fn increment_should_not_erase_original_headers_when_not_replaced() {
    let input_a = r#"
        method = "GET"
        host = "http://localhost:5000"
        
        [headers]
        accept = "application/json"
        "#;
    let inptu_b = r#"
    path = "/resource"

    [headers]
    authorization = "simple_token"
    "#;
    let message_a =
        RequestMessage::from_text(input_a).expect("sample message should have been parsed");
    let message_b =
        RequestMessage::from_text(inptu_b).expect("sample message should have been parsed");

    let merged_message: RequestMessage = message_a.merge_with(message_b);
    assert!(merged_message.path.is_some_and(|x| x == "/resource"));

    assert!(merged_message
        .headers
        .is_some_and(|h| h.contains_key("accept")));
}

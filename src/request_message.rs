use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

use crate::request_error::RequestError;

pub enum HttpVerb {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Options,
    Patch,
}

impl Display for HttpVerb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HttpVerb::Get => write!(f, "GET"),
            HttpVerb::Head => write!(f, "HEAD"),
            HttpVerb::Post => write!(f, "POST"),
            HttpVerb::Put => write!(f, "PUT"),
            HttpVerb::Delete => write!(f, "DELETE"),
            HttpVerb::Options => write!(f, "OPTIONS"),
            HttpVerb::Patch => write!(f, "PATCH"),
        }
    }
}

pub struct RequestMessage {
    pub method: HttpVerb,
    pub url: String,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestMessageBuilder {
    method: Option<String>,
    host: Option<String>,
    path: Option<String>,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl RequestMessage {
    pub fn from_text(file_text: &str) -> Result<RequestMessageBuilder, RequestError> {
        let parsed = toml::from_str::<RequestMessageBuilder>(file_text);

        return parsed.map_err(|e| -> RequestError {
            return RequestError::TomlParserError {
                message: e.message().to_string(),
            };
        });
    }
}

impl Display for RequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "method: {}", self.method)?;
        writeln!(f, "url: {}", self.url)?;
        for kv in &self.headers {
            writeln!(f, "{0}:{1}", kv.0, kv.1)?;
        }
        writeln!(f, "\nbody: {}", self.body)?;
        Ok(())
    }
}

impl RequestMessageBuilder {
    pub fn merge_with(&self, new_message: RequestMessageBuilder) -> RequestMessageBuilder {
        fn increment_header(
            opt_headers: Option<HashMap<String, String>>,
            mut target_headers: HashMap<String, String>,
        ) -> HashMap<String, String> {
            if let Some(cur_headers) = opt_headers {
                for (k, v) in cur_headers {
                    target_headers.insert(k, v);
                }
            }
            target_headers
        }

        let method = new_message.method.or(self.method.clone());
        let host = new_message.host.or(self.host.clone());
        let path = new_message.path.or(self.path.clone());
        let body = new_message.body.or(self.body.clone());

        let copied_values = increment_header(self.headers.clone(), HashMap::new());
        let incremented = increment_header(new_message.headers, copied_values);

        let headers = Some(incremented);
        RequestMessageBuilder {
            method,
            host,
            path,
            body,
            headers,
        }
    }

    pub fn into_message(self) -> Result<RequestMessage, RequestError> {
        let host = match self.host {
            Some(x) => x,
            None => {
                return Err(RequestError::BuildError {
                    property_name: "host".to_owned(),
                })
            }
        };
        let path = match self.path {
            Some(x) => x,
            None => {
                return Err(RequestError::BuildError {
                    property_name: "path".to_owned(),
                })
            }
        };

        let method_candidate = match self.method {
            Some(x) => x,
            None => {
                return Err(RequestError::BuildError {
                    property_name: "method".to_owned(),
                })
            }
        };

        return Ok(RequestMessage {
            method: parse_method(method_candidate),
            url: host + &path,
            body: self.body.unwrap_or("".to_string()),
            headers: self.headers.unwrap_or_default(),
        });

        fn parse_method(candidate: String) -> HttpVerb {
            match candidate.to_uppercase().as_str() {
                "GET" => HttpVerb::Get,
                "HEAD" => HttpVerb::Head,
                "POST" => HttpVerb::Post,
                "PUT" => HttpVerb::Put,
                "DELETE" => HttpVerb::Delete,
                "OPTIONS" => HttpVerb::Options,
                "PATCH" => HttpVerb::Patch,
                _ => HttpVerb::Head,
            }
        }
    }
}

#[test]
fn from_text_should_return_parser_error_when_text_is_not_toml() {
    let input = "invalid toml";
    let maybe_message = RequestMessage::from_text(input);
    assert!(maybe_message.is_err_and(|e| matches!(e, RequestError::TomlParserError { message: _ })));
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

    let merged_message = message_a.merge_with(message_b);
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

    let merged_message = message_a.merge_with(message_b);
    assert!(merged_message.path.is_some_and(|x| x == "/resource"));

    assert!(merged_message
        .headers
        .is_some_and(|h| h.contains_key("accept")));
}

#[test]
fn to_message_should_copy_values() {
    let input = r#"
        method = "GET"
        host = "http://localhost:5000"
        path = "/"
        body = '{"fake_json":"value"}'
        
        [headers]
        accept = "application/json"
        authorization = "simple_token"

        "#;
    let builder = RequestMessage::from_text(input).unwrap();
    let message = builder.into_message().unwrap();
    assert!(matches!(message.method, HttpVerb::Get));
    assert_eq!(message.url, "http://localhost:5000/");
    assert_eq!(message.body, r#"{"fake_json":"value"}"#);
    assert_eq!(message.headers.len(), 2);
    for kv in message.headers.iter() {
        assert!(
            (kv.0 == "accept" && kv.1 == "application/json")
                || (kv.0 == "authorization" && kv.1 == "simple_token")
        );
    }
}

use std::time::Instant;

use crate::response_message::ResponseMessage;
use crate::{request_error::RequestError, request_message::RequestMessage};

pub async fn send(request_message: RequestMessage) -> Result<ResponseMessage, RequestError> {
    println!("{}", request_message);
    let mut request = match request_message.method {
        crate::request_message::HttpVerb::GET => surf::get(request_message.url).build(),
        crate::request_message::HttpVerb::HEAD => surf::head(request_message.url).build(),
        crate::request_message::HttpVerb::POST => surf::post(request_message.url)
            .body_string(request_message.body)
            .build(),
        crate::request_message::HttpVerb::PUT => surf::put(request_message.url)
            .body_string(request_message.body)
            .build(),
        crate::request_message::HttpVerb::DELETE => surf::delete(request_message.url)
            .body_string(request_message.body)
            .build(),
        crate::request_message::HttpVerb::OPTIONS => surf::options(request_message.url).build(),
        crate::request_message::HttpVerb::PATCH => surf::patch(request_message.url)
            .body_string(request_message.body)
            .build(),
    };

    for kv in request_message.headers.iter() {
        let header_key = kv.0.as_str();
        let header_value = kv.1;
        request.set_header(header_key, header_value);
    }

    let client = surf::client();
    let started_at = Instant::now();

    let result = client.send(request).await;

    return match result {
        Ok(mut response) => {
            let status = response.status() as u16;
            let time = started_at.elapsed().as_millis();
            let body_read = response.body_string().await;
            let mut headers = Vec::new();

            for header_name in response.header_names() {
                let maybe_value = response.header(header_name);
                if let Some(value) = maybe_value {
                    headers.push(format!("{}: {}", header_name, value))
                }
            }

            return match body_read {
                Ok(body) => Ok(ResponseMessage {
                    status,
                    time_in_ms: time,
                    body,
                    headers,
                }),
                Err(inner) => Err(RequestError::HttpError { inner }),
            };
        }
        Err(inner) => Err(RequestError::HttpError { inner }),
    };
}

use crate::{request_message::RequestMessage, request_error::RequestError};

pub async fn send(request_message: RequestMessage) -> Result<String, RequestError> {
    let mut request =  match request_message.method {
        crate::request_message::HttpVerb::GET => surf::get(request_message.url).build(),
        crate::request_message::HttpVerb::HEAD => surf::head(request_message.url).build(),
        crate::request_message::HttpVerb::POST => surf::post(request_message.url).body_string(request_message.body).build(),
        crate::request_message::HttpVerb::PUT => surf::put(request_message.url).body_string(request_message.body).build(),
        crate::request_message::HttpVerb::DELETE => surf::delete(request_message.url).body_string(request_message.body).build(),
        crate::request_message::HttpVerb::OPTIONS => surf::options(request_message.url).build(),
        crate::request_message::HttpVerb::PATCH => surf::patch(request_message.url).body_string(request_message.body).build(),
    };
    
    
    for kv in request_message.headers.iter(){
        let header_key = kv.0.as_str();
        let header_value = kv.1;
        request.set_header(header_key, header_value);
    }

    let client = surf::client();
    let resutl = client.recv_string(request).await;
    return resutl.map_err(|inner|  RequestError::HttpError { inner });
}

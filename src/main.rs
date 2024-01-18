use std::fs;

use clap::Parser;
use request_error::RequestError;
use request_message::RequestMessage;
use async_std::task;

mod http_client;
mod request_error;
mod request_message;

#[derive(Parser)]
#[command(name = "Text Http Client")]
#[command(version = "0.1")]
#[command(about = "Send an http request described in a toml file", long_about = None)]
struct Cli {
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    let read_file_result = read_file(&cli.file_path);
    let result = read_file_result
        .and_then(|toml_text| RequestMessage::from_text(&toml_text))
        .and_then(|rmb| rmb.to_message())
        .and_then(|message| task::block_on(http_client::send(message)));
        
     
    match result {
        Ok(message) => {
            println!("{message}");
        }
        Err(e) => println!("{e}"),
    }
}


fn read_file(file_path: &str) -> Result<String, RequestError> {
    let result = fs::read_to_string(file_path);
    return match result {
        Ok(value) => Ok(value),
        Err(_) => Err(RequestError::CouldNotReadFile),
    };
}

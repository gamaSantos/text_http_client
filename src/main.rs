use std::{env, fs};

use async_std::task;
use clap::Parser;
use request_error::RequestError;
use request_message::{RequestMessage, RequestMessageBuilder};

mod http_client;
mod request_error;
mod request_message;

#[derive(Parser)]
#[command(name = "Text Http Client")]
#[command(version = "0.2")]
#[command(about = "Send an http request described in a toml file", long_about = None)]
struct Cli {
    file_path: String,
    #[arg(short, long, default_value = "environment.toml")]
    base_file_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let base_builder = read_base_file(cli.base_file_path);
    let read_file_result = read_file(&cli.file_path);
    let result = read_file_result
        .and_then(|toml_text| RequestMessage::from_text(&toml_text))
        .and_then(|rmb| {
            if let Some(base) = base_builder {
                return Ok(base.merge_with(rmb));
            }
            println!("no base file");
            return Ok(rmb);
        })
        .and_then(|rmb| rmb.to_message())
        .and_then(|message| task::block_on(http_client::send(message)));

    match result {
        Ok(message) => {
            println!("{message}");
        }
        Err(e) => println!("{e}"),
    }
}

fn read_base_file(path: Option<String>) -> Option<RequestMessageBuilder> {
    if let Some(file_path) = path {
        let mut path =
            env::current_dir().expect("should have read access to current working directory");
        path.push(file_path);
        return match fs::read_to_string(path) {
            Ok(v) => match RequestMessage::from_text(&v) {
                Ok(b) => Some(b),
                Err(_) => None,
            },
            Err(_) => None,
        };
    }
    return None;
}

fn read_file(file_path: &str) -> Result<String, RequestError> {
    let result = fs::read_to_string(file_path);
    return match result {
        Ok(value) => Ok(value),
        Err(_) => Err(RequestError::CouldNotReadFile),
    };
}

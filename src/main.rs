use std::{env, fs};

use async_std::task;
use clap::Parser;
use logger::Verbosity;
use request_error::RequestError;
use request_message::{RequestMessage, RequestMessageBuilder};

mod http_client;
mod logger;
mod request_error;
mod request_message;
mod response_message;

#[derive(Parser)]
#[command(name = "Text Http Client")]
#[command(version = "0.2.2")]
#[command(about = "Send an http request described in a toml file", long_about = None)]
struct Cli {
    //toml file path with the arguments
    file_path: String,

    //base toml file that will be merged with the main file
    #[arg(short, long, default_value = "environment.toml")]
    base_file_path: Option<String>,

    //level of information that will be printed
    #[arg(long, default_value = "normal")]
    verbosity: Option<Verbosity>,
}

fn main() {
    let cli = Cli::parse();
    let base_builder = read_base_file(cli.base_file_path);
    let read_file_result = read_file(&cli.file_path);
    if let Some(level) = cli.verbosity {
        crate::logger::init(level);
    }
    let result = read_file_result
        .and_then(|toml_text| RequestMessage::from_text(&toml_text))
        .map(|rmb| {
            if let Some(base) = base_builder {
                return base.merge_with(rmb);
            }
            logger::log_msg("could not parse base file", Verbosity::Normal);
            rmb
        })
        .and_then(|rmb| rmb.into_message())
        .and_then(|message| task::block_on(http_client::send(message)));

    match result {
        Ok(message) => {
            logger::log(&message, Verbosity::Minimal);
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
    None
}

fn read_file(file_path: &str) -> Result<String, RequestError> {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(value) => Ok(value),
        Err(_) => Err(RequestError::CouldNotReadFile),
    }
}

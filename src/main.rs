use std::path::Path;

use clap::Parser;

use crate::request_message::RequestMessage;

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

    let http_message = RequestMessage::from_file(&cli.file_path);



    println!("path: {:?}", cli.file_path);
}

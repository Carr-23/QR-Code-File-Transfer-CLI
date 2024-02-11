use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs::File;

#[derive(Parser)]
struct CLIArgs {
    path: std::path::PathBuf,
}

fn send_request(buffer: File) {
    let client = reqwest::blocking::Client::new();
    let url = "https://transfer.sh";

    let mut headers = HeaderMap::new();

    headers.insert("Max-Downloads", HeaderValue::from_static("1"));
    headers.insert("Max-Days", HeaderValue::from_static("1"));

    let response = client
        .post(url)
        .headers(headers)
        .body(buffer)
        .send()
        .expect("Failed to send request");

    println!("Success! {:?}", response.text());
}

fn read_file(path: &std::path::PathBuf) -> File {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could Not Open File: {}", err);
            std::process::exit(1);
        }
    };
    return file;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLIArgs::parse();

    let buffer: File = read_file(&args.path);
    send_request(buffer);
    println!("Path: {:?}", args.path);

    Ok(())
}

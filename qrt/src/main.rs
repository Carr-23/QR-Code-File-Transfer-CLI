use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
struct CLIArgs {
    path: std::path::PathBuf,
}

fn send_request(buffer: Vec<u8>, url: String) -> String {
    let client = reqwest::blocking::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Max-Downloads", HeaderValue::from_static("1"));
    headers.insert("Max-Days", HeaderValue::from_static("1"));

    let response = client
        .put(url)
        .headers(headers)
        .body(buffer)
        .send()
        .unwrap();

    response.text().unwrap()
}

fn file_to_vec8(mut file: File) -> Vec<u8> {
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(buffer) => buffer,
        Err(err) => {
            eprintln!("Could not convert to Vec8: {}", err);
            std::process::exit(1);
        }
    };
    buffer
}

fn read_file(path: &std::path::PathBuf) -> File {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could Not Open File: {}", err);
            std::process::exit(1);
        }
    };
    file
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLIArgs::parse();
    let encoded_file_name = urlencoding::encode(args.path.file_name().unwrap().to_str().unwrap());
    let mut host_url: String = String::from("https://transfer.sh/");
    host_url.push_str(&encoded_file_name);

    let file = read_file(&args.path);
    let buffer: Vec<u8> = file_to_vec8(file);

    let file_url = send_request(buffer, host_url);

    println!("URL: {:?}", file_url);

    Ok(())
}

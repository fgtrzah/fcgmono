pub mod fileapi;

use fileapi::fileapi::File;
use serde_json;
use std::env;
use reqwest; // 0.10.0
use tokio; // 0.2.6

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn inspect_file_struct(json: String) -> File {
    let model: File = serde_json::from_str(&json).unwrap();

    model
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    
    let file = match env::var_os("FILE") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$FILE is not set")
    };

    let token = match env::var_os("FIGMA_TOKEN") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$FIGMA_TOKEN is not set")
    };
    
    let res = client
        .get(format!("https://api.figma.com/v1/files/{}?depth=1", file))
        .header("X-FIGMA-TOKEN", token)
        .send()
        .await?;

    let text_body = res
        .text()
        .await?;
    println!("{:?}", text_body);

    let model = inspect_file_struct(text_body);

    println!("{:?}", model.styles);
    
    Ok(())
}

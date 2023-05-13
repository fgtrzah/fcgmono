use std::env;
use reqwest; // 0.10.0
use tokio; // 0.2.6

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    
    let file = match env::var_os("FILE") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$FILE is not set")
    };
    println!("{}", file);

    let token = match env::var_os("FIGMA_TOKEN") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$FIGMA_TOKEN is not set")
    };
    println!("{}", token);
    
    let res = client
        .get(format!("https://api.figma.com/v1/files/{}?depth=1", file))
        .header("X-FIGMA-TOKEN", token)
        .send()
        .await?;

    println!("{}", res.text().await?);
    
    Ok(())
}
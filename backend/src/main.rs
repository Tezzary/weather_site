
use reqwest::{self, header::USER_AGENT};
use tokio;
use std::fs::File;
use std::io::prelude::*;

const REQUEST_URL: &str = "http://www.bom.gov.au/radar/IDR023.T.202502281319.png";

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    let result = get_req().await;
    
}

async fn get_req() -> Result<(),  Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0".parse().unwrap());
    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

    let image_bytes = client.get(REQUEST_URL)
    .send()
    .await?
    .bytes()
    .await?;

    let mut file = File::create("test.png")?;
    let data: Result<Vec<_>, _> = image_bytes.bytes().collect();
    file.write_all(&data.unwrap())?;
    Ok(())
}
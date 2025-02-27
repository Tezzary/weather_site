
use reqwest::{self, header::USER_AGENT};
use tokio;

const REQUEST_URL: &str = "http://www.bom.gov.au/radar/IDR023.T.202502271249.png";

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    let result = get_req().await;
    
}

async fn get_req() -> Result<(),  Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:53.0) Gecko/20100101 Firefox/53.0".parse().unwrap());
    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

    let response = client.get(REQUEST_URL)
    .send()
    .await?
    .text()
    .await?;


    println!("body = {:?}", response);
    Ok(())
}
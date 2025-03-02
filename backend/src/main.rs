
use reqwest::{self, header::USER_AGENT};
use tokio;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;
use chrono::{naive, DateTime};


const REQUEST_URL: &str = "http://www.bom.gov.au/radar/IDR023.T.202503021144.png";

#[tokio::main]
async fn main() {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let date = chrono::DateTime::from_timestamp(time as i64, 0).unwrap();

    let naive_date = date.date_naive();
    let string_date = naive_date.format("%Y%m%d").to_string();
    let naive_time = date.time();
    let string_time = naive_time.format("%H%M").to_string();
    let combined_string = string_date + &string_time;
    println!("{}", combined_string);

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
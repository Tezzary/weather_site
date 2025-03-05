
use reqwest::{self, header::USER_AGENT};
use tokio;
use std::{fs::File};
use std::io::prelude::*;
use std::time::SystemTime;
use chrono::{naive, DateTime};


//const REQUEST_URL: &str = "http://www.bom.gov.au/radar/IDR023.T.202503021144.png";

fn get_date_string_from_timestamp(timestamp: u64) -> String {
    let date = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap();

    let naive_date = date.date_naive();
    let string_date = naive_date.format("%Y%m%d").to_string();
    let naive_time = date.time();
    let string_time = naive_time.format("%H%M").to_string();

    let formatted_date = string_date + &string_time;
    formatted_date
}
#[tokio::main]
async fn main() {
    let mut timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let formatted_date = get_date_string_from_timestamp(timestamp);

    let mut image_data: Vec<Vec<u8>> = Vec::new();

    loop {
        let url = "http://www.bom.gov.au/radar/IDR023.T.".to_owned() + &get_date_string_from_timestamp(timestamp) + ".png";
        let request_result = get_req(url).await;
        if request_result.is_err() == false {
            image_data.push(request_result.unwrap());
            break;
        }
        timestamp -= 60;
        println!("loop");
    }
    for i in 1..7 {

        let url = "http://www.bom.gov.au/radar/IDR023.T.".to_owned() + &get_date_string_from_timestamp(timestamp - 5 * i * 60) + ".png";
        println!("{}", url);
        let request_result = get_req(url.clone()).await;
        if request_result.is_err() {
            println!("failed with initial populaton at i={} with url = {}", i, url);
            return;
        }
        //image_data.insert(0, request_result.unwrap()); //oldest images first
        image_data.push(request_result.unwrap());

        //let mut file = File::create("test.png").unwrap();

        //file.write_all(&bytes).unwrap();
    }
    for i in 0..image_data.len() {
        let mut file = File::create(format!("{}.png", i)).unwrap();
        let image = &image_data[i];
        file.write_all(image).unwrap();
    }

}

fn is_failed_request(bytes: Vec<u8>) -> bool {
    if String::from_utf8(bytes.clone()).is_err() {
        return false;
    }
    true
}

async fn get_req(url: String) -> Result<Vec<u8>,  Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0".parse().unwrap());
    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

    let image_bytes = client.get(url)
    .send()
    .await?
    .bytes()
    .await?;

    
    let data: Result<Vec<_>, _> = image_bytes.bytes().collect();

    let bytes = data.unwrap();
    if is_failed_request(bytes.clone()) {
        println!("failed");
        return Err("request_failed".into());
    }
    //println!("{}", String::from_utf8(bytes.clone())?);
    //let mut file_json = File::create("testing.json")?;
    //file_json.write_all(&bytes)?;
    //println!("{:?}", String::from_utf8(bytes.clone()));
    Ok(bytes)
}
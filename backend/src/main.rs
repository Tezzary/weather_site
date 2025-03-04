
use reqwest::{self, header::USER_AGENT};
use tokio;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;
use chrono::{naive, DateTime};


//const REQUEST_URL: &str = "http://www.bom.gov.au/radar/IDR023.T.202503021144.png";

#[derive(Debug)]
struct MyError {
    message: String,
}

#[tokio::main]
async fn main() {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let date = chrono::DateTime::from_timestamp(time as i64, 0).unwrap();

    let naive_date = date.date_naive();
    let string_date = naive_date.format("%Y%m%d").to_string();
    let naive_time = date.time();
    let string_time = naive_time.format("%H%M").to_string();

    let initial_formatted_date = string_date + &string_time;
    for i in 0..7 {
        let number_date: i64 = initial_formatted_date.parse::<i64>().unwrap() - i;

        let url = "http://www.bom.gov.au/radar/IDR023.T.".to_owned() + &(number_date).to_string() + ".png";

        let bytes = get_req(url).await.unwrap();

        file.write_all(&bytes)?;
    }
    
    
}

fn is_failed_request(bytes: Vec<u8>) -> bool {
    if String::from_utf8(bytes.clone()).is_err() {
        return true;
    }
    false
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

    let mut file = File::create("test.png")?;
    let data: Result<Vec<_>, _> = image_bytes.bytes().collect();

    let bytes = data.unwrap();
    if is_failed_request(bytes.clone()) {
        println!("failed");
        let my_error = MyError {
            message: String::from("request failed"),
        };
        return Err(Box::new(my_error));
    }
    //println!("{}", String::from_utf8(bytes.clone())?);
    //let mut file_json = File::create("testing.json")?;
    //file_json.write_all(&bytes)?;
    file.write_all(&bytes)?;
    Ok(bytes)
}
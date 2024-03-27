use polars::prelude::*;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct APIKey {
    api_key: String,
}

#[derive(Debug)]
struct APODRequestQueryString {
    date: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    count: Option<u8>,
    thumbs: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APODResponse {
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    title: String,
    url: String,
}

async fn url_get(url: String) -> Result<DataFrame, Box<dyn Error>> {
    let res = reqwest::get(url)
        .await?
        .json::<APODResponse>()
        .await?;

    let json = serde_json::to_string(&res).expect("unable to convert struct to string");
    let cursor = std::io::Cursor::new(json);
    let df = JsonReader::new(cursor).finish().unwrap();

    Ok(df)
}

#[tokio::main]
async fn main() {

    let api_key = APIKey {
        api_key: String::from("DEMO_KEY"),
    };

    let url = format!(
        "https://api.nasa.gov/planetary/apod?api_key={}",
        api_key.api_key
    );

    let res = url_get(url).await.expect("Unable to get APODResponse");

    let out = res
        .clone()
        .lazy()
        .select([
            col("*").exclude(["date"]),
            col("date").cast(DataType::Date)
    ]);
    
    println!("{:#?}", out.collect());
}
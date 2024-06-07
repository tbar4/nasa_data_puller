use serde::{Serialize, Deserialize};
use polars::{lazy::dsl::StrptimeOptions, prelude::*};
use std::error::Error;
use url::Url;
use std::string::ParseError;

use chrono::{DateTime, offset::Utc};

use crate::utils::api_key::APIKey;
use super::urls::URLS;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct APODResponse {
    pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct APODRequestQueryString {
    pub url: String,
    pub api_key: APIKey,
    pub date: String,
    pub start_date: String,
    pub end_date: String,
    pub count: String,
    pub thumbs: String,
    pub response: APODResponse
}

impl Default for APODRequestQueryString {
    fn default() -> Self {
        Self {
            url: URLS["apod"].clone(),
            api_key: APIKey::get_api_key(None),
            date: String::from(""),
            start_date: String::from(""),
            end_date: String::from(""),
            count: String::from(""),
            thumbs: String::from(""),
            response: APODResponse{ ..Default::default() },
        }
    }
}

impl APODRequestQueryString {
    pub async fn parse_query_string(&self) -> Result<Url, ParseError> {
        let url = Url::parse_with_params(self.url.as_str(), 
            &[("api_key", self.api_key.api_key.clone()),
            ("date", self.date.clone()),
            ("start_date", self.start_date.clone()),
            ("end_date", self.end_date.clone()),
            ("count", self.count.clone()),
            ("thumbs", self.thumbs.clone()),
        ]).unwrap();
        Ok(url)
    }

    pub async fn url_get(&self) -> Result<DataFrame, Box<dyn Error>> { 
        let url = self.parse_query_string().await.unwrap();
    
        let res = reqwest::get(url)
            .await?
            .json::<APODResponse>()
            .await?;
    
        let json = serde_json::to_string(&res).expect("unable to convert struct to string");
        let cursor = std::io::Cursor::new(json);
        let df = JsonReader::new(cursor).finish().unwrap();

        let df_date_correct = df
            .clone()
            .lazy()
            .select([
                  col("*").exclude(["date"])
                , col("date").str().to_datetime(
                    Some(TimeUnit::Milliseconds),
                    None,
                    StrptimeOptions::default(),
                    lit("raise"),
                )
            ])
            .collect()?;
    
        Ok(df_date_correct)
    }
}
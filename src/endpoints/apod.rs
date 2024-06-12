use serde::{Serialize, Deserialize};

use std::error::Error;

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use url::Url;

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

impl APODResponse {
    pub async fn sink_to_surreal(self, db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
        let _created: Vec<APODResponse> = db
            .create("astronomy_picture_of_the_day")
            .content( Self {
                date: self.date,
                explanation: self.explanation,
                hdurl: self.hdurl,
                media_type: self.media_type,
                service_version: self.service_version,
                title: self.title,
                url: self.url,
            })
            .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct APODRequestQueryString {
    pub url: String,
    //pub api_key: APIKey,
    pub date: String,
    pub start_date: String,
    pub end_date: String,
    pub count: String,
    pub thumbs: String,
}

impl Default for APODRequestQueryString {
    fn default() -> Self {
        Self {
            url: URLS["apod"].clone(),
            date: String::from(""),
            start_date: String::from(""),
            end_date: String::from(""),
            count: String::from(""),
            thumbs: String::from(""),
        }
    }
}

impl APODRequestQueryString {
    pub async fn url_get(&self, api_key: &APIKey) -> Result<APODResponse, Box<dyn Error>> {
        let url = Url::parse_with_params(self.url.as_str(), 
            &[("api_key", api_key.api_key.clone()),
            ("date", self.date.clone()),
            ("start_date", self.start_date.clone()),
            ("end_date", self.end_date.clone()),
            ("count", self.count.clone()),
            ("thumbs", self.thumbs.clone()),
        ])?;
    
        let res = reqwest::get(url)
            .await?
            .json::<APODResponse>()
            .await?;
    
        Ok(res)
    }
}

/*
    pub async fn url_get_return_df(&self) -> Result<DataFrame, Box<dyn Error>> { 
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
    */
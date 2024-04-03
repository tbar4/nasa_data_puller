use serde::{Serialize, Deserialize};
use polars::prelude::*;
use std::{collections::HashMap, error::Error};
use url::Url;
use std::string::ParseError;
use chrono::{self, Datelike};
use serde_json::Value;

use crate::helpers::date_helpers::date_builder;

use super::{api_key::APIKey, urls::URLS};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AsteroidNeoWSResponse {
    pub links: HashMap<String, String>,
    pub element_count: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NearEarthObjects {
    pub near_earth_objects: HashMap<String, Vec<NearEarthObject>>,

    #[serde(flatten)]
    pub pagination: AsteroidNeoWSResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NearEarthObject {
    id: String,
    neo_reference_id: String,
    name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsteroidNeoWSRequestQueryString {
    pub url: String,
    pub api_key: APIKey,
    pub start_date: String,
    pub end_date: String,
}

impl Default for AsteroidNeoWSRequestQueryString {
    fn default() -> Self {
        Self { 
            url: URLS["asteroid_neo_ws"].clone(),
            api_key: APIKey::get_api_key(None), 
            start_date: format!("{}-{}-{}", chrono::Utc::now().year(), chrono::Utc::now().month(), chrono::Utc::now().day()), 
            end_date: format!("{}-{}-{}", chrono::Utc::now().year(), chrono::Utc::now().month(), chrono::Utc::now().day())
        }
    }
}

impl AsteroidNeoWSRequestQueryString {
    pub async fn parse_query_string(&self) -> Result<Url, ParseError> {
        
        let url = Url::parse_with_params(self.url.as_str(), 
            &[("api_key", self.api_key.api_key.clone()),
            ("start_date", self.start_date.clone()),
            ("end_date", self.end_date.clone()),
        ]).unwrap();
        Ok(url)
    }

    pub async fn url_get(&self) -> Result<DataFrame, Box<dyn Error>> { 
        let url = self.parse_query_string().await.unwrap();
        let res = reqwest::get(url)
            .await?
            .json::<NearEarthObjects>()
            .await?;
    
        let json = serde_json::to_string(&res).expect("unable to convert struct to string");
        let cursor = std::io::Cursor::new(json);
        let asteroid = JsonReader::new(cursor).finish().unwrap();

        let df = asteroid
            .clone()
            .lazy()
            .select([
                col("near_earth_objects")
            ])
            .unnest(["near_earth_objects"])
            .explode([date_builder().as_str()])
            .unnest([date_builder().as_str()])
            .collect()
            .unwrap();
    
        Ok(df)
    }
}
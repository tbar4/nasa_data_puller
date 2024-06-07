use serde::{Serialize, Deserialize};
use polars::prelude::*;
use std::{collections::HashMap, error::Error};
use url::Url;
use std::string::ParseError;
use chrono::{self, Datelike};

use crate::utils::date_helpers::date_builder;
use crate::utils::api_key::APIKey;

use super::urls::URLS;

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
    pub id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub nasa_jpl_url: String,
    pub absolute_magnitude_h: f32,
    pub estimated_diameter: EstimatedDiameter,
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_data: Vec<CloseApproachData>,
    pub is_sentry_object: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EstimatedDiameter {
    pub kilometers: EstKilometers,
    pub meters:EstMeters,
    pub miles: EstMiles,
    pub feet: EstFeet,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CloseApproachData {
    pub close_approach_date: String,
    pub close_approach_date_full: String,
    pub epoch_date_close_approach: usize,
    pub relative_velocity: RelativeVelocity,
    pub miss_distance: MissDistance,
    pub orbiting_body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RelativeVelocity {
    pub kilometers_per_second: String,
    pub kilometers_per_hour: String,
    pub miles_per_hour: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MissDistance {
    #[serde(rename(serialize = "miss_distance_astronomical"))]
    pub astronomical: String,
    #[serde(rename(serialize = "miss_distance_lunar"))]
    pub lunar: String,
    #[serde(rename(serialize = "miss_distance_kilometers"))]
    pub kilometers: String,
    #[serde(rename(serialize = "miss_distance_miles"))]
    pub miles: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EstKilometers {
    #[serde(rename(serialize = "estimated_diameter_min_kilometers"))]
    estimated_diameter_min: f32,
    #[serde(rename(serialize = "estimated_diameter_max_kilometers"))]
    estimated_diameter_max: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EstMeters {
    #[serde(rename(serialize = "estimated_diameter_min_meters"))]
    estimated_diameter_min: f32,
    #[serde(rename(serialize = "estimated_diameter_max_meters"))]
    estimated_diameter_max: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EstMiles {
    #[serde(rename(serialize = "estimated_diameter_min_miles"))]
    estimated_diameter_min: f32,
    #[serde(rename(serialize = "estimated_diameter_max_miles"))]
    estimated_diameter_max: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EstFeet {
    #[serde(rename(serialize = "estimated_diameter_min_feet"))]
    estimated_diameter_min: f32,
    #[serde(rename(serialize = "estimated_diameter_max_feet"))]
    estimated_diameter_max: f32,
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
            .explode(["close_approach_data"])
            .unnest(["estimated_diameter", "close_approach_data"])
            .unnest(["kilometers", "meters", "miles", "feet"])
            .unnest(["relative_velocity", "miss_distance"])
            //.unnest(["meters"])
            .collect()
            .unwrap();
    
        Ok(df)
    }
}

/*
            .select([
                (col("estimated_diameter_min")).alias("estimated_diameter_min_meters"),
                (col("estimated_diameter_max")).alias("estimated_diameter_max_meters")
            ])
            .unnest(["miles"])
            .select([
                (col("estimated_diameter_min")).alias("estimated_diameter_min_miles"),
                (col("estimated_diameter_max")).alias("estimated_diameter_max_miles")
            ])
            .unnest(["feet"])
            .select([
                (col("estimated_diameter_min")).alias("estimated_diameter_min_feet"),
                (col("estimated_diameter_max")).alias("estimated_diameter_max_feet")
            ])

*/
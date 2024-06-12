use std::{env, error::Error};

use anyhow::Result;
use dotenv::dotenv;

mod endpoints;
use endpoints::{apod, asteroid_neows_feed};

mod utils;
use utils::{api_key, surreal_credentials::SurrealConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db = SurrealConnection::init().await?;
    
    let api_key_env = dotenv::var("NASA_API_KEY").unwrap();
    let api_key = api_key::APIKey::get_api_key(Some(api_key_env.as_str()));

    let _asteroid = asteroid_neows_feed::AsteroidNeoWSRequestQueryString::default()
        .url_get(&api_key)
        .await?;
        //.near_earth_objects
        //.sink_to_surreal(&db)
        //.await?;

    let _apod = apod::APODRequestQueryString::default()
        .url_get(&api_key)
        .await?
        .sink_to_surreal(&db)
        .await?;

    Ok(())
    
}
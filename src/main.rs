use std::env;

mod endpoints;
use endpoints::{api_key, apod, asteroid_neows_feed};

mod helpers;
use helpers::date_helpers::date_builder;

const API_KEY: Option<&str> = None;

#[tokio::main]
async fn main() {
    env::set_var("POLARS_FMT_MAX_COLS", "50");
    let api_key = api_key::APIKey::get_api_key(API_KEY);

    /* 
    let apod_query = apod::APODRequestQueryString { 
        api_key: api_key,
        ..Default::default()
    }.url_get().await.unwrap();
    */

    let asteroid = asteroid_neows_feed::AsteroidNeoWSRequestQueryString { 
        api_key: api_key,
        ..Default::default()
    }
    .url_get()
    .await
    .unwrap();

    println!("{:#?}", asteroid);
    
}
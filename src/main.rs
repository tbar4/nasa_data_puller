use std::env;

mod endpoints;
use endpoints::{apod, asteroid_neows_feed};

mod utils;
use utils::{api_key, date_helpers::date_builder};

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

    //let asteroid = asteroid_neows_feed::AsteroidNeoWSRequestQueryString { 
    let asteroid = apod::APODRequestQueryString {
        api_key: api_key,
        ..Default::default()
    }
    .url_get()
    .await
    .unwrap();

    println!("{:#?}", asteroid);
    
}
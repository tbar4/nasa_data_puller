use polars::{lazy::dsl::col, prelude::*};

mod endpoints;
use endpoints::{api_key, apod, asteroid_neows_feed};

mod helpers;
use helpers::date_helpers::date_builder;

const API_KEY: Option<&str> = Some("9hlwQ6xZEzNnPaIwTmROODd1r1F57D0mkLY5Qar8");

#[tokio::main]
async fn main() {

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
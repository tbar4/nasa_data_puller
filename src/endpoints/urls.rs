use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref URLS: HashMap<String, String> =
    HashMap::from([
        (String::from("apod"), String::from("https://api.nasa.gov/planetary/apod")),
        (String::from("asteroid_neo_ws"), String::from("https://api.nasa.gov/neo/rest/v1/feed"))
    ]);
}
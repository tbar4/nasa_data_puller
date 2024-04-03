use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIKey {
    pub api_key: String,
}

impl Default for APIKey {
    fn default() -> Self {
        Self {
            api_key: String::from("DEMO_KEY")
        }
    }
}

impl APIKey {
    pub fn get_api_key(api_key: Option<&str>) -> Self {
        match api_key {
            Some(api_key) => Self { api_key: String::from(api_key) },
            None => Self { ..Default::default() }
        }
    }
}
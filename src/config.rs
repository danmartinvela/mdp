use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub date_from: String,
    pub date_to: String,
    pub directory: String,
    pub symbols: Vec<String>,
    pub limit: Option<u32>,
}

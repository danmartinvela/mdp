use reqwest::Error;
use serde_json::Value;

pub struct Downloader {
    api_key: String,
}

impl Downloader {
    pub fn new(api_key: &String) -> Self {
        Downloader {
            api_key: api_key.to_string(),
        }
    }

    pub async fn download_data(
        &self,
        symbol: &str,
        date_from: &str,
        date_to: &str,
        limit: &Option<u32>,
    ) -> Result<Value, Error> {
        let limit = limit.unwrap_or(1000);
        let url = format!(
            "http://api.marketstack.com/v1/eod?access_key={}&symbols={}&date_from={}&date_to={}&limit={}",
            self.api_key, symbol, date_from, date_to, limit
        );

        let response = reqwest::get(&url).await?;
        let data: Value = response.json().await?;
        // println!("{}", data);
        Ok(data)
    }
}

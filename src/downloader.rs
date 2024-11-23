use reqwest::Error;
use serde_json::Value;

pub struct Downloader {
    api_key: String,
}

impl Downloader {
    pub fn new(api_key: String) -> Self {
        Downloader { api_key }
    }

    pub async fn download_data(
        &self,
        symbol: &str,
        date_from: &str,
        date_to: &str,
    ) -> Result<Value, Error> {
        let url = format!(
            "http://api.marketstack.com/v1/eod?access_key={}&symbols={}&date_from={}&date_to={}",
            self.api_key, symbol, date_from, date_to
        );

        let response = reqwest::get(&url).await?;

        let data: Value = response.json().await?;
        // println!("{}", data);
        Ok(data)
    }
}

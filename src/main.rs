mod config;
mod downloader;
mod processor;

use config::Config;
use downloader::Downloader;
use processor::Processor;
use serde_json::from_str;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Example: {} /Users/danmartinvela/Desktop/mdp/config.txt",
            args[0]
        );
        std::process::exit(1);
    }

    let config_path = &args[1];
    let config_content =
        fs::read_to_string(config_path).expect("Error: failed to read the config file");
    let config: Config = from_str(&config_content).expect("Error: failed to parse the config file");

    let downloader = Downloader::new(config.api_key);
    let processor = Processor::new();

    for symbol in config.symbols {
        match downloader
            .download_data(&symbol, &config.date_from, &config.date_to)
            .await
        {
            Ok(data) => {
                //println!("Data downloaded for {}: {:?}", symbol, data);
                processor.process_data(&data);
            }
            Err(e) => eprintln!("Error downloading data for {}: {}", symbol, e),
        }
    }
}

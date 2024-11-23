mod config;
mod date_formatter;
mod downloader;
mod entry;
mod output_formatter;
mod processor;
mod writer;

use config::Config;
use downloader::Downloader;
use entry::Entry;
use output_formatter::MetastockOutputWriterFormatter;
use processor::Processor;
use serde_json::from_str;
use std::env;
use std::fs;
use writer::Writer;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[mdp - example] mdp /Users/danmartinvela/Desktop/mdp/config.txt",);
        std::process::exit(1);
    }

    let config_path = &args[1];
    let config_content =
        fs::read_to_string(config_path).expect("Error: failed to read the config file");
    let config: Config = from_str(&config_content).expect("Error: failed to parse the config file");

    let downloader = Downloader::new(config.api_key);
    // TODO processor new DYN INPUT DATE
    let processor = Processor::new();
    let writer = Writer::new(Box::new(MetastockOutputWriterFormatter));

    for symbol in config.symbols {
        match downloader
            .download_data(&symbol, &config.date_from, &config.date_to, &config.limit)
            .await
        {
            Ok(data) => {
                let all_entries: Vec<Entry> = processor.process_data(&data);
                writer.write_file(all_entries);
            }
            Err(e) => eprintln!("Error downloading data for {}: {}", symbol, e),
        }
    }
}

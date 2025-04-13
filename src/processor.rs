use crate::{date_formatter::MarketstackMetastockDateFormatter, entry::Entry};
use serde_json::Value;

pub struct Processor;

impl Processor {
    pub fn new() -> Self {
        Processor
    }

    pub fn process_data(&self, data: &Value) -> Vec<Entry> {
        if let Some(array) = data.get("data").and_then(|d| d.as_array()) {
            let mut entries: Vec<Entry> = vec![];
            for item in array {
                let symbol = item
                    .get("symbol")
                    .and_then(|s| s.as_str())
                    .unwrap_or("N/A")
                    .to_string();
                let date = item
                    .get("date")
                    .and_then(|d| d.as_str())
                    .unwrap_or("N/A")
                    .to_string();
                let open = item.get("open").and_then(|o| o.as_f64()).unwrap_or(0.0);
                let close = item.get("close").and_then(|c| c.as_f64()).unwrap_or(0.0);
                let high = item.get("high").and_then(|h| h.as_f64()).unwrap_or(0.0);
                let low = item.get("low").and_then(|l| l.as_f64()).unwrap_or(0.0);
                let volume = item.get("volume").and_then(|v| v.as_f64()).unwrap_or(0.0);

                let entry = Entry::new(
                    symbol,
                    date,
                    open,
                    close,
                    high,
                    low,
                    volume,
                    Box::new(MarketstackMetastockDateFormatter),
                );
                println!("{}", entry);
                entries.push(entry);
            }
            entries
        } else {
            println!("[Warning] no data found in the response");
            vec![]
        }
    }
}

use crate::entry::Entry;

pub trait OutputWriterFormatter {
    fn format(&self, entry: &Entry) -> String;
}

pub struct MetastockOutputWriterFormatter;

impl OutputWriterFormatter for MetastockOutputWriterFormatter {
    fn format(&self, entry: &Entry) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            entry.symbol, entry.date, entry.open, entry.high, entry.low, entry.close, entry.volume
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Entry, MetastockOutputWriterFormatter, OutputWriterFormatter};

    #[test]
    fn test_metastock_output_writer_formatter() {
        let formatter = MetastockOutputWriterFormatter;
        let entry_input = Entry {
            symbol: "TSLA".to_string(),
            date: "12/01/2024".to_string(),
            open: 230.50,
            close: 250.10,
            high: 251.10,
            low: 230.00,
            volume: 123456789,
        };
        let expected_output = "TSLA,12/01/2024,230.5,251.1,230,250.1,123456789";

        let actual_output = formatter.format(&entry_input);

        assert_eq!(expected_output, actual_output);
    }
}

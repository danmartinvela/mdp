use crate::entry::Entry;

pub trait OutputWriterFormatter {
    fn format(&self, entry: &Entry) -> String;
}

pub struct MetastockOutputWritterFormatter;

impl OutputWriterFormatter for MetastockOutputWritterFormatter {
    fn format(&self, entry: &Entry) -> String {
        // TICKER,DATE,   HOUR?     OPEN HIGH LOW  CLOSE VOLUMEN
        // NTNX,1/05/2017,15:30:00,13.5,13.94,10.7,11.1,11297800
        format!(
            "{},{},{},{},{},{},{}",
            entry.symbol, entry.date, entry.open, entry.high, entry.low, entry.low, entry.volume
        )
    }
}

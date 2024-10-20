use crate::date_formatter::InputDateFormatter;
use core::fmt;

pub struct Entry {
    pub symbol: String,
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: i64,
}

impl Entry {
    pub fn new(
        symbol: String,
        date: String,
        open: f64,
        close: f64,
        high: f64,
        low: f64,
        volume: i64,
        formatter: Box<dyn InputDateFormatter>,
    ) -> Self {
        let formatted_date = formatter.format(&date);
        Entry {
            symbol,
            date: formatted_date,
            open,
            close,
            high,
            low,
            volume,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIX THIS; I WANT A TAB INSTEAD OF SPACES; MAKE IT WORK
        write!(
            f,
            "{}  {}  {}  {}  {:.2}  {:.2}  {:.2}",
            self.symbol, self.date, self.open, self.close, self.high, self.low, self.volume
        )
    }
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::Entry;
    use crate::date_formatter::MarketstackInputFormatter;

    #[test]
    fn test_entry_not_formatted() {
        let formatter = Box::new(MarketstackInputFormatter);
        let entry = Entry::new(
            "ASRT".to_string(),
            "2024-10-01T00:00:00+0000".to_string(),
            1.10,
            1.50,
            1.50,
            1.10,
            123456789,
            formatter,
        );
        println!("{}", entry);
        println!("{}", entry);
        println!("{}", entry);

        assert_eq!(entry.symbol, "ASRT");
        assert_ne!(entry.date, "2024-10-01T00:00:00+0000");
        assert_eq!(entry.open, 1.10);
    }

    #[test]
    fn test_entry_formatted() {
        let formatter = Box::new(MarketstackInputFormatter);
        let entry = Entry::new(
            "ASRT".to_string(),
            "2024-10-01T00:00:00+0000".to_string(),
            1.10,
            1.50,
            1.50,
            1.10,
            123456789,
            formatter,
        );

        assert_eq!(entry.symbol, "ASRT");
        assert_eq!(entry.date, "01-10-2024");
        assert_eq!(entry.open, 1.10);
    }
}

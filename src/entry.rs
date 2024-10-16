// use core::fmt;

use core::fmt;

pub struct Entry {
    symbol: String,
    date: String,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: i64,
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
    ) -> Self {
        Entry {
            symbol,
            date,
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

    #[test]
    fn test_new() {
        let entry = Entry::new(
            "ASRT".to_string(),
            "2024-10-01T00:00:00+0000".to_string(),
            1.10,
            1.50,
            1.50,
            1.10,
            123456789,
        );
        println!("{}", entry);
        println!("{}", entry);
        println!("{}", entry);

        assert_eq!(entry.symbol, "ASRT");
        assert_eq!(entry.date, "2024-10-01T00:00:00+0000");
        assert_eq!(entry.open, 1.10);
    }
}

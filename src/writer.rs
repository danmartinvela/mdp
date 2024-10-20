use crate::{entry::Entry, output_formatter::OutputWriterFormatter};

pub struct Writer {
    formatter: Box<dyn OutputWriterFormatter>,
}

impl Writer {
    pub fn new(formatter: Box<dyn OutputWriterFormatter>) -> Self {
        Writer { formatter }
    }

    pub fn write_file(&self, entries: Vec<Entry>) {
        for entry in entries {
            let formatted_entry = self.formatter.format(&entry);

            writeln!(writer, "{}", formatted_entry).expect("Could not write to file");
        }
    }
}

// // Escribe en el archivo
// writeln!(
//     file,
//     "Date: {}, Open: {}, Close: {}, High: {}, Low: {}, Volume: {}",
//     date, open, close, high, low, volume
// )?;

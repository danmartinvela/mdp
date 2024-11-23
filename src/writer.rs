use crate::{entry::Entry, output_formatter::OutputWriterFormatter};
use std::fs::File;
use std::io::Write;

pub struct Writer {
    formatter: Box<dyn OutputWriterFormatter>,
}

impl Writer {
    pub fn new(formatter: Box<dyn OutputWriterFormatter>) -> Self {
        Writer { formatter }
    }

    pub fn write_file(&self, entries: Vec<Entry>) {
        let mut file =
            File::create("/Users/danmartinvela/Desktop/IDT.txt").expect("Failed to create file");

        for entry in entries {
            let formatted_entry = self.formatter.format(&entry);
            writeln!(file, "{}", formatted_entry).expect("Failed to write to file");
        }
    }
}

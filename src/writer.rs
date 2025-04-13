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

    pub fn write_file(&self, entries: &Vec<Entry>, directory: &String) {
        //or .config folder default
        let file_name = format!("{}/{}.txt", directory, entries[0].symbol);
        let mut file_path = File::create(&file_name).expect("[Error] failed to create file");

        for entry in entries {
            let formatted_entry = self.formatter.format(&entry);
            writeln!(file_path, "{}", formatted_entry).expect("[Error] failed to write to file");
        }
    }
}

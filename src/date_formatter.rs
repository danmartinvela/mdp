pub trait DateFormatter {
    fn format(&self, date: &str) -> String;
}

pub struct MarketstackMetastockDateFormatter;

impl DateFormatter for MarketstackMetastockDateFormatter {
    // "yyyy-mm-ddT00:00:00+0000" -> mm/dd/yyyy
    fn format(&self, date: &str) -> String {
        let parts: Vec<&str> = date.split('T').collect();
        let date_parts: &str = parts[0];
        let date_components: Vec<&str> = date_parts.split('-').collect();
        format!(
            "{}/{}/{}",
            date_components[1], date_components[2], date_components[0]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{DateFormatter, MarketstackMetastockDateFormatter};

    #[test]
    fn test_marketstack_metastock_date_formatter() {
        let formatter = MarketstackMetastockDateFormatter;
        let date_input = "2024-10-01T00:00:00+0000";
        let expected_output = "10/01/2024";

        let actual_output = formatter.format(date_input);

        assert_eq!(expected_output, actual_output);
    }
}

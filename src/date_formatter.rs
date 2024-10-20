pub trait InputDateFormatter {
    fn format(&self, date: &str) -> String;
}

pub trait OutputDateFormatter {
    fn format(&self, date: &str) -> String;
}

pub struct MarketstackInputFormatter;

impl InputDateFormatter for MarketstackInputFormatter {
    // "yyyy-mm-ddT00:00:00+0000" -> dd/mm/yyyy
    fn format(&self, date: &str) -> String {
        let parts: Vec<&str> = date.split('T').collect();
        let date_parts: &str = parts[0];
        let date_components: Vec<&str> = date_parts.split('-').collect();
        format!(
            "{}-{}-{}",
            date_components[2], date_components[1], date_components[0]
        )
    }
}

pub struct MetastockOutputFormatter;

impl OutputDateFormatter for MetastockOutputFormatter {
    // dd/mm/yy -> mm/dd/yyyy
    fn format(&self, date: &str) -> String {
        let parts: Vec<&str> = date.split('-').collect();
        format!("{}/{}/{}", parts[1], parts[0], parts[2])
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InputDateFormatter, MarketstackInputFormatter, MetastockOutputFormatter,
        OutputDateFormatter,
    };

    #[test]
    fn test_marketstack_input_formatter() {
        let formatter = MarketstackInputFormatter;
        let date_input = "2024-10-01T00:00:00+0000";
        let expected_output = "01-10-2024";

        let formatted_date = formatter.format(date_input);

        assert_eq!(formatted_date, expected_output);
    }

    #[test]
    fn test_metastock_output_formatter() {
        let formatter = MetastockOutputFormatter;
        let date_input = "01-05-2017";
        let expected_output = "05/01/2017";

        let formatted_date = formatter.format(date_input);

        assert_eq!(formatted_date, expected_output);
    }
}

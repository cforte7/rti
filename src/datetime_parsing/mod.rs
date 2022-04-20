// If datetime -> NaiveDateTime::parse_from_string(arg, fmt).timestamp
// if date -> NaiveDate::parse_from_string(arg, fmt).and_hms(0,0,0).timestamp
// if time ->
//
//
//
//
//
//

pub mod datetime_parsing {
    use chrono::prelude::{DateTime, Local, NaiveDate, NaiveTime};
    use itertools::iproduct;

    pub const INVALID_ARG: &str = "Invalid Pattern"; // public for tests

    fn parse_time(time: NaiveTime) -> String {
        // Add time to Date object of today to get DateTime
        Local::today()
            .and_time(time)
            .unwrap()
            .timestamp()
            .to_string()
    }

    fn parse_date(date: NaiveDate) -> String {
        // need to add zero seconds to get full DateTime before converting to timestamp
        date.and_hms(0, 0, 0).timestamp().to_string()
    }

    pub fn parse_datetime(arg: &str) -> String {
        // Take an arg from the command line and try to match it to known date/time patterns

        let time_patterns = [
            "%I:%M %P", // 01:23 PM
            "%I:%M %p", // 01:23 pm
            "%l:%M %P", // 1:23 PM
            "%l:%M %p", // 1:23 pm
            "%H:%M",    // 13:55
        ];
        for pattern in time_patterns {
            if let Ok(time) = NaiveTime::parse_from_str(arg, pattern) {
                return parse_time(time);
            }
        }

        // order matters! Some will wrongly catch if not in correct order.
        let date_patterns = [
            "%m-%d-%y", // 5-24-93
            "%m-%d-%Y", // 5-24-1993
            "%D",       // 05/24/93, 5/24/93
            "%m/%d/%Y", // 5/24/1993
            "%F",       // 1993-05-01
            "%v",       // 1-May-1993
        ];
        for pattern in date_patterns {
            if let Ok(date) = NaiveDate::parse_from_str(arg, pattern) {
                println!("{:?}", pattern);
                return parse_date(date);
            }
        }

        // for full datetime, allow any combination of the known date/time patterns
        let datetime_patterns = iproduct!(date_patterns, time_patterns);
        println!("{:?}", datetime_patterns);
        for pattern in datetime_patterns {
            let parse_str = format!("{} {}", pattern.0, pattern.1);
            if let Ok(datetime) = DateTime::parse_from_str(arg, &*parse_str) {
                // if we have a full datetime, we can go straight to timestamp
                return datetime.timestamp().to_string();
            }
        }

        // if no matches found notify user we can't parse string
        return INVALID_ARG.to_string();
    }
}

#[cfg(test)]
mod time_tests {
    // Test that the known time string formats are being parsed.
    // Since the output of these functions depends on the day you
    // run them, we can only test that it does not return as INVALID_ARG
    // This is not ideal as we cannot assert accuracy of how it is
    // being parsed, only that it is being parsed into something.
    // TODO: refactor to fix this.
    use super::datetime_parsing::{parse_datetime, INVALID_ARG};
    #[test]
    fn test_24_hour_time() {
        assert_ne!(parse_datetime("13:55"), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_uppercase() {
        assert_ne!(parse_datetime("01:23 PM"), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_lowercase() {
        assert_ne!(parse_datetime("01:23 pm"), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_lowercase() {
        assert_ne!(parse_datetime("1:23 pm"), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_uppercase() {
        assert_ne!(parse_datetime("1:23 PM"), INVALID_ARG);
    }
}

#[cfg(test)]
mod date_tests {
    use super::datetime_parsing::parse_datetime;
    // let date_patterns = [
    //     "%m-%d-%y", // 5-24-93
    //     "%m-%d-%Y", // 5-24-1993
    //     "%m/%d/%Y",
    //     "%D",
    //     "%F",
    //     "%v",
    // ];

    const MAY_ONE_1993: &str = "736214400";
    #[test]
    fn test_dashes_long_year_no_pad() {
        // "%m-%d-%Y"
        assert_eq!(parse_datetime("5-1-1993"), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_long_year_padded() {
        // "%m-%d-%Y"
        assert_eq!(parse_datetime("05-01-1993"), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_no_pad() {
        // "%m-%d-%y"
        assert_eq!(parse_datetime("5-1-93"), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_padded() {
        // "%m-%d-%y"
        assert_eq!(parse_datetime("05-01-93"), MAY_ONE_1993);
    }

    #[test]
    fn test_slashes_short_year() {
        // %D
        assert_eq!(parse_datetime("5/1/93"), MAY_ONE_1993)
    }
    #[test]
    fn test_slashes_long_year() {
        // "%m/%d/%Y"
        assert_eq!(parse_datetime("5/1/1993"), MAY_ONE_1993)
    }

    #[test]
    fn test_dashes_year_padded_month_day() {
        // "%F"
        assert_eq!(parse_datetime("1993-05-01"), MAY_ONE_1993)
    }

    #[test]
    fn test_dashes_year_month_day() {
        // "%F"
        assert_eq!(parse_datetime("1993-5-1"), MAY_ONE_1993)
    }

    #[test]
    fn test_dashes_day_word_month_year() {
        // "%F"
        assert_eq!(parse_datetime("1-May-1993"), MAY_ONE_1993)
    }
}

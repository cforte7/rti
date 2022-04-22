pub mod datetime_parsing {
    use super::date_time_patterns::{DATE_PATTERNS, TIME_PATTERNS};
    use chrono::prelude::{FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime};
    use chrono::Duration;

    use itertools::iproduct;

    pub const INVALID_ARG: &str = "Invalid Pattern"; // public for tests

    fn time_to_string(time: NaiveTime) -> String {
        // Get tz aware datetime and offset with provided time.
        Local::today()
            .and_time(time)
            .unwrap()
            .timestamp()
            .to_string()
    }

    fn date_to_string(date: NaiveDate) -> String {
        // Create datetime at midnight from date, offset with local timezone
        let datetime = date.and_hms(0, 0, 0);
        let local_time = Local::now();
        let timezone_offset: &FixedOffset = local_time.offset();
        (datetime - *timezone_offset).timestamp().to_string()
    }

    fn datetime_to_string(datetime: NaiveDateTime) -> String {
        datetime.timestamp().to_string()
    }

    pub fn parse_arg(arg: &str) -> String {
        // Take an arg from the command line and try to match it to known date/time patterns

        for pattern in TIME_PATTERNS {
            if let Ok(time) = NaiveTime::parse_from_str(arg, pattern) {
                return time_to_string(time);
            }
        }

        for pattern in DATE_PATTERNS {
            if let Ok(date) = NaiveDate::parse_from_str(arg, pattern) {
                return date_to_string(date);
            }
        }

        // for full datetime, allow any combination of the known date/time patterns
        let datetime_patterns =
            iproduct!(DATE_PATTERNS, TIME_PATTERNS).map(|(x, y)| format!("{} {}", x, y));
        for pattern in datetime_patterns {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(arg, &pattern) {
                return datetime_to_string(datetime);
            }
        }

        let timedate_patterns =
            iproduct!(TIME_PATTERNS, DATE_PATTERNS).map(|(x, y)| format!("{} {}", x, y));
        for pattern in timedate_patterns {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(arg, &pattern) {
                return datetime_to_string(datetime);
            }
        }

        return match arg {
            "yesterday" => (Local::now() + Duration::days(-1)).timestamp().to_string(),
            "now" => Local::now().timestamp().to_string(),
            "tomorrow" => (Local::now() + Duration::days(1)).timestamp().to_string(),
            _ => INVALID_ARG.to_string(),
        };

        // if no matches found notify user we can't parse string
        // return INVALID_ARG.to_string();
    }
}

mod date_time_patterns {
    // order matters! Some will wrongly catch if not in correct order.
    pub static DATE_PATTERNS: [&str; 6] = [
        "%m-%d-%y", // 5-24-93
        "%m-%d-%Y", // 5-24-1993
        "%D",       // 05/24/93, 5/24/93
        "%m/%d/%Y", // 5/24/1993
        "%F",       // 1993-05-01
        "%v",       // 1-May-1993
    ];

    pub static TIME_PATTERNS: [&str; 10] = [
        "%I:%M %P",    // 01:23 PM
        "%I:%M %p",    // 01:23 pm
        "%l:%M %P",    // 1:23 PM
        "%l:%M %p",    // 1:23 pm
        "%H:%M",       // 13:55
        "%I:%M:%S %P", // 01:23:01 PM
        "%I:%M:%S %p", // 01:23:01 pm
        "%l:%M:%S %P", // 1:23:01 PM
        "%l:%M:%S %p", // 1:23:01 pm
        "%H:%M:%S",    // 13:55:01
    ];
}

#[cfg(test)]
mod time_tests {
    // Test that the known time string formats are being parsed.
    // Since the output of these functions depends on the day you
    // run them, we can only test that it does not return as INVALID_ARG
    // This is not ideal as we cannot assert accuracy of how it is
    // being parsed, only that it is being parsed into something.
    // TODO: refactor to fix this.
    use super::datetime_parsing::{parse_arg, INVALID_ARG};
    #[test]
    fn test_24_hour_time() {
        assert_ne!(parse_arg("13:55"), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_uppercase() {
        assert_ne!(parse_arg("01:23 PM"), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_lowercase() {
        assert_ne!(parse_arg("01:23 pm"), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_lowercase() {
        assert_ne!(parse_arg("1:23 pm"), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_uppercase() {
        assert_ne!(parse_arg("1:23 PM"), INVALID_ARG);
    }
}

#[cfg(test)]
mod date_tests {
    use super::datetime_parsing::parse_arg;
    const MAY_ONE_1993: &str = "736232400";
    #[test]
    fn test_dashes_long_year_no_pad() {
        // "%m-%d-%Y"
        assert_eq!(parse_arg("5-1-1993"), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_long_year_padded() {
        // "%m-%d-%Y"
        assert_eq!(parse_arg("05-01-1993"), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_no_pad() {
        // "%m-%d-%y"
        assert_eq!(parse_arg("5-1-93"), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_padded() {
        // "%m-%d-%y"
        assert_eq!(parse_arg("05-01-93"), MAY_ONE_1993);
    }

    #[test]
    fn test_slashes_short_year() {
        // %D
        assert_eq!(parse_arg("5/1/93"), MAY_ONE_1993)
    }
    #[test]
    fn test_slashes_long_year() {
        // "%m/%d/%Y"
        assert_eq!(parse_arg("5/1/1993"), MAY_ONE_1993)
    }

    #[test]
    fn test_dashes_year_padded_month_day() {
        // "%F"
        assert_eq!(parse_arg("1993-05-01"), MAY_ONE_1993)
    }

    #[test]
    fn test_dashes_year_month_day() {
        // "%F"
        assert_eq!(parse_arg("1993-5-1"), MAY_ONE_1993)
    }

    #[test]
    fn test_dashes_day_word_month_year() {
        // "%F"
        assert_eq!(parse_arg("1-May-1993"), MAY_ONE_1993)
    }
}

#[cfg(test)]
mod datetime_tests {
    // Only going to test a few since the tests above are comprehensive and
    // these are all functions of the above working
    use super::datetime_parsing::parse_arg;
    const MAY_ONE_1993_FOUR_FIFTY: &str = "736231800";

    #[test]
    fn test_multi_part() {
        assert_eq!(parse_arg("1-May-1993 4:50 AM"), MAY_ONE_1993_FOUR_FIFTY);
    }

    #[test]
    fn test_slashes_date_lowercase_am() {
        assert_eq!(parse_arg("5/1/93 4:50 am"), MAY_ONE_1993_FOUR_FIFTY);
    }

    #[test]
    fn test_dashes_then_24hour_time() {
        assert_eq!(parse_arg("2022-04-22 11:40:09"), "1650627609");
    }
}

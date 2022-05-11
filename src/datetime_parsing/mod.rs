//https://blog.logrocket.com/timezone-handling-in-rust-with-chrono-tz/

pub mod datetime_parsing {
    use super::date_time_patterns::{DATE_PATTERNS, TIME_PATTERNS};
    use chrono::prelude::{
        Local, NaiveDateTime, NaiveTime, TimeZone, NaiveDate,
    };

    use chrono::format::Parsed;
    use chrono::{Duration, Utc};
    use chrono_tz::{Tz};
    use itertools::iproduct;

    pub const INVALID_ARG: &str = "Invalid Pattern"; // public for tests

    fn time_to_epoch(time: NaiveTime, tz: &Tz) -> String {
            let utctoday = Utc::today();
            let naive_with_time = utctoday.and_time(time).unwrap().naive_utc();
            let tz_aware = (*tz).from_local_datetime(&naive_with_time).unwrap();
            tz_aware.timestamp().to_string()
    }


    fn date_to_epoch(date: NaiveDate, tz: &Tz) -> String {
        // Create datetime at midnight from date, offset with timezone
        let with_time = date.and_hms(0,0,0);
        let tz_aware = (*tz).from_local_datetime(&with_time).unwrap();
        tz_aware.timestamp().to_string()
    }

    fn datetime_to_epoch(datetime: NaiveDateTime, tz: &Tz) -> String {
        let tz_aware = (*tz).from_local_datetime(&datetime).unwrap();
        return tz_aware.timestamp().to_string();
    }

    pub fn parse_arg(arg: &str, tz: &Tz, custom_tokens: &Vec<String>) -> String {
        // Take an arg from the command line and try to match it to known date/time patterns

        for pattern in custom_tokens {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(arg, &pattern) {
                return datetime_to_epoch(datetime, tz);
            }
        }

        for pattern in TIME_PATTERNS {
            if let Ok(time) = NaiveTime::parse_from_str(arg, pattern) {
                return time_to_epoch(time, tz);
            }
        }

        for pattern in DATE_PATTERNS {
            if let Ok(date) = NaiveDate::parse_from_str(arg, pattern) {
                return date_to_epoch(date, tz);
            }
        }

        // for full datetime, allow any combination of the known date/time patterns
        let datetime_patterns =
            iproduct!(DATE_PATTERNS, TIME_PATTERNS).map(|(x, y)| format!("{} {}", x, y));
        for pattern in datetime_patterns {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(arg, &pattern) {
                return datetime_to_epoch(datetime, tz);
            }
        }
        let timedate_patterns =
            iproduct!(TIME_PATTERNS, DATE_PATTERNS).map(|(x, y)| format!("{} {}", x, y));
        for pattern in timedate_patterns {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(arg, &pattern) {
                return datetime_to_epoch(datetime, tz);
            }
        }

        return match arg {
            "yesterday" => (Local::now() + Duration::days(-1)).timestamp().to_string(),
            "now" => Local::now().timestamp().to_string(),
            "tomorrow" => (Local::now() + Duration::days(1)).timestamp().to_string(),
            _ => INVALID_ARG.to_string(),
        };
    }

    const DATETIME_PARSE_FORMAT: &str = "%m-%d-%Y %H:%M:%S";
    pub fn epoch_to_datetime(epoch: i64, tz: &Tz) -> String {
        // take in epoch time and return datetime as timezone adjusted string.
        let mut parsed = Parsed::new();
        parsed.set_timestamp(epoch).unwrap();
        parsed
            .to_datetime_with_timezone(tz)
            .unwrap()
            .format(DATETIME_PARSE_FORMAT)
            .to_string()
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
mod utc_time_tests {
    // Test that the known time string formats are being parsed.
    // Since the output of these functions depends on the day you
    // run them, we can only test that it does not return as INVALID_ARG
    // This is not ideal as we cannot assert accuracy of how it is
    // being parsed, only that it is being parsed into something.
    // TODO: refactor to fix this.
    use super::datetime_parsing::{parse_arg, INVALID_ARG};
    use chrono_tz::UTC;
    #[test]
    fn test_24_hour_time() {
        assert_ne!(parse_arg("13:55", &UTC), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_uppercase() {
        assert_ne!(parse_arg("01:23 PM", &UTC), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_lowercase() {
        assert_ne!(parse_arg("01:23 pm", &UTC), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_lowercase() {
        assert_ne!(parse_arg("1:23 pm", &UTC), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_uppercase() {
        assert_ne!(parse_arg("1:23 PM", &UTC), INVALID_ARG);
    }
}

#[cfg(test)]
mod with_tz_time_tests {
    // Test that the known time string formats are being parsed.
    // Since the output of these functions depends on the day you
    // run them, we can only test that it does not return as INVALID_ARG
    // This is not ideal as we cannot assert accuracy of how it is
    // being parsed, only that it is being parsed into something.
    // TODO: refactor to fix this.
    use super::datetime_parsing::{parse_arg, INVALID_ARG};
    use chrono_tz::US::Central;

    #[test]
    fn test_24_hour_time() {
        assert_ne!(parse_arg("13:55", &Central), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_uppercase() {
        assert_ne!(parse_arg("01:23 PM", &Central), INVALID_ARG);
    }

    #[test]
    fn test_padded_hour_lowercase() {
        assert_ne!(parse_arg("01:23 pm", &Central), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_lowercase() {
        assert_ne!(parse_arg("1:23 pm", &Central), INVALID_ARG);
    }

    #[test]
    fn test_not_padded_hour_uppercase() {
        assert_ne!(parse_arg("1:23 PM", &Central), INVALID_ARG);
    }
}

#[cfg(test)]
mod utc_date_tests {
    use super::datetime_parsing::parse_arg;
    use chrono_tz::UTC;
    const MAY_ONE_1993: &str = "736214400";
    #[test]
    fn test_dashes_long_year_no_pad() {
        // "%m-%d-%Y"
        assert_eq!(parse_arg("5-1-1993", &UTC), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_long_year_padded() {
        // "%m-%d-%Y"
        assert_eq!(parse_arg("05-01-1993", &UTC), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_no_pad() {
        // "%m-%d-%y"
        assert_eq!(parse_arg("5-1-93", &UTC), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_padded() {
        // "%m-%d-%y"
        assert_eq!(parse_arg("05-01-93", &UTC), MAY_ONE_1993);
    }

    #[test]
    fn test_slashes_short_year() {
        // %D
        assert_eq!(parse_arg("5/1/93", &UTC), MAY_ONE_1993)
    }
    #[test]
    fn test_slashes_long_year() {
        // "%m/%d/%Y"
        assert_eq!(parse_arg("5/1/1993", &UTC), MAY_ONE_1993)
    }

    // also test in January before daylight savings time
    const JAN_THIRD_NINETY_THREE: &str = "726019200";
    #[test]
    fn test_dashes_year_padded_month_day() {
        // "%F"
        assert_eq!(parse_arg("1993-01-03", &UTC), JAN_THIRD_NINETY_THREE)
    }

    #[test]
    fn test_dashes_year_month_day() {
        // "%F"
        assert_eq!(parse_arg("1993-1-3", &UTC), JAN_THIRD_NINETY_THREE)
    }

    #[test]
    fn test_dashes_day_word_month_year() {
        // "%F"
        assert_eq!(parse_arg("3-Jan-1993", &UTC), JAN_THIRD_NINETY_THREE)
    }
}

#[cfg(test)]
mod with_tz_date_tests {
    use super::datetime_parsing::parse_arg;
    use chrono_tz::US::Central;

    const MAY_ONE_1993: &str = "736232400";
    #[test]
    fn test_dashes_long_year_no_pad() {
        // "%m-%d-%Y"
        assert_eq!(parse_arg("5-1-1993", &Central), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_long_year_padded() {
        // "%m-%d-%Y"
        assert_eq!(parse_arg("05-01-1993", &Central), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_no_pad() {
        // "%m-%d-%y"
        assert_eq!(parse_arg("5-1-93", &Central), MAY_ONE_1993);
    }

    #[test]
    fn test_dashes_short_year_padded() {
        // "%m-%d-%y"
        assert_eq!(parse_arg("05-01-93", &Central), MAY_ONE_1993);
    }

    #[test]
    fn test_slashes_short_year() {
        // %D
        assert_eq!(parse_arg("5/1/93", &Central), MAY_ONE_1993)
    }
    #[test]
    fn test_slashes_long_year() {
        // "%m/%d/%Y"
        assert_eq!(parse_arg("5/1/1993", &Central), MAY_ONE_1993)
    }

    // also test in January before daylight savings time
    const JAN_THIRD_NINETY_THREE: &str = "726040800";
    #[test]
    fn test_dashes_year_padded_month_day() {
        // "%F"
        assert_eq!(parse_arg("1993-01-03", &Central), JAN_THIRD_NINETY_THREE)
    }

    #[test]
    fn test_dashes_year_month_day() {
        // "%F"
        assert_eq!(parse_arg("1993-1-3", &Central), JAN_THIRD_NINETY_THREE)
    }

    #[test]
    fn test_dashes_day_word_month_year() {
        // "%F"
        assert_eq!(parse_arg("3-Jan-1993", &Central), JAN_THIRD_NINETY_THREE)
    }
}

#[cfg(test)]
mod utc_datetime_tests {
    // Only going to test a few since the tests above are comprehensive and
    // these are all functions of the above working
    use super::datetime_parsing::parse_arg;
    const MAY_ONE_1993_FOUR_FIFTY: &str = "736231800";
    use chrono_tz::UTC;
    #[test]
    fn test_multi_part() {
        assert_eq!(parse_arg("1-May-1993 4:50 AM", &UTC), MAY_ONE_1993_FOUR_FIFTY);
    }

    #[test]
    fn test_slashes_date_lowercase_am() {
        assert_eq!(parse_arg("5/1/93 4:50 am", &UTC), MAY_ONE_1993_FOUR_FIFTY);
    }

    #[test]
    fn test_dashes_then_24hour_time() {
        assert_eq!(parse_arg("2022-04-22 13:40:09", &UTC), "1650634809");
    }
}


#[cfg(test)]
mod with_tz_datetime_tests {
    // Only going to test a few since the tests above are comprehensive and
    // these are all functions of the above working
    use super::datetime_parsing::parse_arg;
    const MAY_ONE_1993_FOUR_FIFTY: &str = "736249800";
    use chrono_tz::US::Central;


    #[test]
    fn test_multi_part() {
        assert_eq!(parse_arg("1-May-1993 4:50 AM", &Central), MAY_ONE_1993_FOUR_FIFTY);
    }

    #[test]
    fn test_slashes_date_lowercase_am() {
        assert_eq!(parse_arg("5/1/93 4:50 am", &Central), MAY_ONE_1993_FOUR_FIFTY);
    }

    #[test]
    fn test_dashes_then_24hour_time() {
        assert_eq!(parse_arg("2022-04-22 13:40:09", &Central), "1650652809");
    }
}



#[cfg(test)]
mod with_tz_epoch_to_datetime {
    // Only going to test a few since the tests above are comprehensive and
    // these are all functions of the above working
    use super::datetime_parsing::epoch_to_datetime;
    use chrono_tz::US::Central;

    #[test]
    fn test_epoch_before_ds_time() {
        const JAN_TEN_TWENTY_TWO: i64 = 1641794400;
        assert_eq!(epoch_to_datetime(JAN_TEN_TWENTY_TWO, &Central),
                   "01-10-2022 00:00:00");
    }

    #[test]
    fn test_epoch_during_ds_time() {
        const MAY_ONE_1993_FOUR_FIFTY: i64 = 736249800;
        assert_eq!(epoch_to_datetime(MAY_ONE_1993_FOUR_FIFTY, &Central),
                   "05-01-1993 04:50:00");
    }


    #[test]
    fn test_epoch_after_ds_time() {
        const OCT_TEN_TWENTY_TWO: i64 = 1665378000;
        assert_eq!(epoch_to_datetime(OCT_TEN_TWENTY_TWO, &Central),
                   "10-10-2022 00:00:00");
    }
}

#[cfg(test)]
mod utc_epoch_to_datetime {
    // Only going to test a few since the tests above are comprehensive and
    // these are all functions of the above working
    use super::datetime_parsing::epoch_to_datetime;
    use chrono_tz::UTC;
    #[test]
    fn test_epoch_before_ds_time() {
        const JAN_TEN_TWENTY_TWO: i64 = 1641794400;
        assert_eq!(epoch_to_datetime(JAN_TEN_TWENTY_TWO, &UTC),
                   "01-10-2022 06:00:00");
    }

    #[test]
    fn test_epoch_during_ds_time() {
        const MAY_ONE_1993_FOUR_FIFTY: i64 = 736249800;
        assert_eq!(epoch_to_datetime(MAY_ONE_1993_FOUR_FIFTY, &UTC),
                   "05-01-1993 09:50:00");
    }


    #[test]
    fn test_epoch_after_ds_time() {
        const NOV_TEN_TWENTY_TWO: i64 = 1668060000;
        assert_eq!(epoch_to_datetime(NOV_TEN_TWENTY_TWO, &UTC),
                   "11-10-2022 06:00:00");
    }
}
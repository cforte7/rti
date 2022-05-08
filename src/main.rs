extern crate chrono;
use chrono::format::Parsed;
use chrono::prelude::Local;
use chrono_tz::Tz;
mod config;
use config::config::{clear_tz_config, load_config, set_tz_config, MyConfig};
use std::env;

mod datetime_parsing;
use datetime_parsing::datetime_parsing::parse_arg;

const DATETIME_PARSE_FORMAT: &str = "%m-%d-%Y %H:%M:%S";

fn epoch_to_datetime(epoch: i64, config: &MyConfig) -> String {
    // take in epoch time and return datetime as timezone adjusted string.
    let mut parsed = Parsed::new();
    parsed.set_timestamp(epoch).unwrap();
    // This would be much cleaner as a Trait object but you can't do this
    // with TimeZone (chrono limitation)
    // https://github.com/chronotope/chrono/issues/432
    match &config.default_timezone {
        Some(tz) => {
            let tz: Tz = tz.parse().unwrap();
            let timezone_offset = tz;
            parsed
                .to_datetime_with_timezone(&timezone_offset)
                .unwrap()
                .format(DATETIME_PARSE_FORMAT)
                .to_string()
        }
        None => {
            let local_offset = Local::now();
            parsed
                .to_datetime_with_timezone(&local_offset.timezone())
                .unwrap()
                .format(DATETIME_PARSE_FORMAT)
                .to_string()
        }
    }
}

fn fmt_and_print(arg: String, config: &MyConfig) {
    let maybe_int_parse = arg.parse::<i64>();
    let parsed_value = match maybe_int_parse {
        Ok(val) => epoch_to_datetime(val, config),
        Err(_) => parse_arg(&arg, config),
    };

    println!("{}", format!("{} => {}", arg, parsed_value));
}

fn main() {
    let input: Vec<String> = env::args().collect();
    let maybe_keyword: String = input[1].parse().unwrap();
    match maybe_keyword.as_str() {
        "set-tz" => set_tz_config(input[2].parse().unwrap()),
        "clear-tz" => clear_tz_config(),
        _ => {
            let config = load_config();
            for elem in input[1..].iter() {
                fmt_and_print(elem.to_string(), &config);
            }
            println!("Timezone: {}", config.get_timezone_str());
        }
    }
}

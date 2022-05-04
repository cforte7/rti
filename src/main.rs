extern crate chrono;
use chrono::format::Parsed;
use chrono::prelude::{DateTime, Local, NaiveDateTime};
use chrono_tz::Tz;
mod config;
use config::config::{clear_tz_config, load_config, set_tz_config, MyConfig};

use std::env;

mod datetime_parsing;
use datetime_parsing::datetime_parsing::parse_arg;

const DATETIME_PARSE_FORMAT: &str = "%m-%d-%Y %H:%M:%S";

fn epoch_to_datetime(epoch: i64, config: &MyConfig) -> String {
    // take in epoch time and return datetime as string formatted for system timezone.
    // todo: allow env var to specify timezone
    let mut parsed = Parsed::new();
    parsed.set_timestamp(epoch).unwrap();
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
        } // parsed.to_datetime_with_timezone(&timezone_offset).unwrap().format(DATETIME_PARSE_FORMAT).to_string()
    }
}

fn fmt_and_print(arg: String, config: &MyConfig) {
    let maybe_int_parse = arg.parse::<i64>();
    let parsed_value = match maybe_int_parse {
        Ok(val) => epoch_to_datetime(val, config),
        Err(_) => parse_arg(&arg),
    };

    println!("{}", format!("{} => {}", arg, parsed_value));
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input[1] == "set-tz" {
        let tz_arg: String = input[2].parse().unwrap();
        match tz_arg.as_str() {
            "clear" => clear_tz_config(),
            _ => set_tz_config(tz_arg.to_string()),
        };
        return;
    }

    let config = load_config();
    println!("{:?}", config);
    for elem in input[1..].iter() {
        fmt_and_print(elem.to_string(), &config);
    }
}

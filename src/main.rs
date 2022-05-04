extern crate chrono;
use chrono::format::Parsed;
use chrono::prelude::Local;
use chrono_tz::TZ_VARIANTS;

mod config;
use config::config::load_config;

use std::env;

mod datetime_parsing;
use datetime_parsing::datetime_parsing::parse_arg;

const DATETIME_PARSE_FORMAT: &str = "%m-%d-%Y %H:%M:%S";

fn epoch_to_datetime(epoch: i64) -> String {
    // take in epoch time and return datetime as string formatted for system timezone.
    // todo: allow env var to specify timezone
    let mut parsed = Parsed::new();
    parsed.set_timestamp(epoch).unwrap();
    let local_time = Local::now();
    let timezone_offset = local_time.offset();
    parsed
        .to_datetime_with_timezone(timezone_offset)
        .unwrap()
        .format(DATETIME_PARSE_FORMAT)
        .to_string()
}

fn fmt_and_print(arg: String) {
    let maybe_int_parse = arg.parse::<i64>();
    let parsed_value = match maybe_int_parse {
        Ok(val) => epoch_to_datetime(val),
        Err(_) => parse_arg(&arg),
    };

    println!("{}", format!("{} => {}", arg, parsed_value));
}

fn main() {
    for tz in TZ_VARIANTS {
        println!("{:?}", tz);
    }
    let config = load_config();
    println!("{:?}", config);
    let input: Vec<String> = env::args().collect();
    for elem in input[1..].iter() {
        fmt_and_print(elem.to_string());
    }
}

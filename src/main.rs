extern crate chrono;
use chrono::prelude::NaiveDateTime;

use std::env;

mod datetime_parsing;
use datetime_parsing::datetime_parsing::parse_arg;

const DATETIME_PARSE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn epoch_to_datetime(epoch: i64) -> String {
    // take in epoch time and return datetime as string
    let datetime = NaiveDateTime::from_timestamp(epoch, 0);
    datetime.format(DATETIME_PARSE_FORMAT).to_string()
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
    let input: Vec<String> = env::args().collect();
    for elem in input[1..].iter() {
        fmt_and_print(elem.to_string());
    }
}

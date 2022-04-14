extern crate chrono;
use chrono::prelude::{Local, NaiveDateTime};

use std::env;
struct EpochArg {
    arg: String,
}
const DATETIME_PARSE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn epoch_to_datetime(epoch: i64) -> String {
    // take in epoch time and return datetime as string
    let datetime = NaiveDateTime::from_timestamp(epoch, 0);
    datetime.format(DATETIME_PARSE_FORMAT).to_string()
}

fn parse_string(arg: &String) -> String {
    // take in datetime string and return epoch as string
    match arg.to_lowercase().as_str() {
        "now" => Local::now().timestamp().to_string(),
        _ => "Unsupported keyword".to_string(),
    }
}

impl EpochArg {
    fn new(arg: String) -> EpochArg {
        EpochArg { arg }
    }

    fn fmt_for_user(&self) -> String {
        let maybe_int_parse = self.arg.parse::<i64>();
        let parsed_value = match maybe_int_parse {
            Ok(val) => epoch_to_datetime(val),
            Err(_) => parse_string(&self.arg),
        };

        format!("{} => {}", self.arg, parsed_value).to_string()
    }
}

fn parse_inputs(input_args: Vec<String>) -> Vec<EpochArg> {
    // Take in arbitrary number of inputs and convert them to EpochArg
    let mut parsed_input = vec![];

    // first arg is always path to file which we do not want
    for elem in input_args[1..input_args.len()].iter() {
        parsed_input.push(EpochArg::new(elem.to_string()));
    }
    parsed_input
}

fn main() {
    let input: Vec<String> = env::args().collect();
    let parsed_inputs = parse_inputs(input);

    for i in parsed_inputs {
        println!("{}", i.fmt_for_user());
    }
}

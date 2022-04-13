// use clap::Parser;
extern crate chrono;
use std::env;

use chrono::prelude::{NaiveDateTime};

struct EpochArg {
    epoch: i64,
    parsed_value: NaiveDateTime
}

impl EpochArg {
    fn new(epoch: i64) -> EpochArg {
        EpochArg{epoch, parsed_value: NaiveDateTime::from_timestamp(epoch , 0)}
    }

    fn fmt_for_user(&self) -> String {
        format!("{} => {}", self.epoch, self.parsed_value.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}



fn main() {
    let input: Vec<String> = env::args().collect();
    let mut parsed_input = vec![];

    // first arg is always path to file which we do not want
    for elem in input[1..input.len()].iter() {
        let parsed_elem = elem.parse::<i64>().unwrap();
        parsed_input.push(EpochArg::new(parsed_elem))
    }

    for i in parsed_input {
        println!("{}", i.fmt_for_user())
    }

}

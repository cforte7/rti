// use clap::Parser;
extern crate chrono;
use std::env;

use chrono::prelude::{NaiveDateTime};

enum InputArgTypes {
    EpochInput,
    StringInput,
}

struct EpochArg {
    original_input: String,
}
const DATETIME_PARSE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

impl EpochArg {
    fn new(original_input: String) -> EpochArg {
        EpochArg{original_input}
    }

    fn fmt_for_user(&self) -> String {
        format!("{} => {}", self.original_input, self.parsed_value.format("%Y-%m-%d %H:%M:%S")).to_string()
    }
}



fn parse_inputs(input_args: Vec<String>) -> Vec<EpochArg> {
    // Take in arbitrary number of inputs and convert them to EpochArg struct
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

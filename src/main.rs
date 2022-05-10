extern crate chrono;
use chrono_tz::Tz;
mod config;
use config::config::{clear_tz_config, load_config, set_tz_config, get_timezone};
use std::env;

mod datetime_parsing;
use datetime_parsing::datetime_parsing::{parse_arg, epoch_to_datetime};


fn fmt_and_print(arg: String) {
    let maybe_int_parse = arg.parse::<i64>();
    let tz: Option<Tz> = get_timezone();
    let parsed_value = match maybe_int_parse {
        Ok(val) => epoch_to_datetime(val, tz),
        Err(_) => parse_arg(&arg, tz),
    };

    println!("{}", format!("{} => {}", arg, parsed_value));
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() == 1 {
        println!("Must include at least one argument!");
        return;
    }
    let maybe_keyword: String = input[1].parse().unwrap();
    match maybe_keyword.as_str() {
        "set-tz" => set_tz_config(input[2].parse().unwrap()),
        "clear-tz" => clear_tz_config(),
        _ => {
            for elem in input[1..].iter() {
                fmt_and_print(elem.to_string());
            }
            println!("Timezone: {}", load_config().get_timezone_str());
        }
    }
}

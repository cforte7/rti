extern crate chrono;
use chrono_tz::Tz;
mod config;
use config::config::{
    clear_tz_config,
    set_tz_config,
    get_timezone,
    get_custom_tokens,
    add_custom_token,
    remove_custom_token,
    view_tokens
};
use std::env;

mod datetime_parsing;
use datetime_parsing::datetime_parsing::{parse_arg, epoch_to_datetime};



fn fmt_and_print(arg: String, tz: &Tz, custom_tokens: &Vec<String>) {
    let maybe_int_parse = arg.parse::<i64>();
    let parsed_value = match maybe_int_parse {
        Ok(val) => epoch_to_datetime(val, tz),
        Err(_) => parse_arg(&arg, tz, custom_tokens),
    };

    println!("{}", format!("{} => {}", arg, parsed_value));
}


fn help() {
    println!("RTI converts Unix epoch time to a human readable format and vice versa.");
    println!("Enter values to convert separated by a space.\n");
    println!("Additional commands:");
    println!("    help -  View this message.");
    println!("    set-tz - Set a configured timezone. Uses first argument after set-tz.");
    println!("    clear-tz - Clear timezone config.");
    println!("    add-token - Add a custom parsing token. Uses first argument after add-token. See https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html for syntax.");
    println!("    remove-token - Remove a custom parsing token. No changes made if the token doesn't exist.");
    println!("    view-tokens - See a list of stored custom parsing tokens.");
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() == 1 {
        println!("Must include at least one argument!");
        return;
    }
    let maybe_keyword: String = input[1].parse().unwrap();
    match maybe_keyword.as_str() {
        "help" => help(),
        "set-tz" => set_tz_config(input[2].parse().unwrap()),
        "clear-tz" => clear_tz_config(),
        "add-token" => add_custom_token(input[2].parse().unwrap()),
        "remove-token" => remove_custom_token(input[2].parse().unwrap()),
        "view-tokens" => view_tokens(),
        _ => {
            let tz: Tz = get_timezone();
            let custom_tokens: Vec<String> = get_custom_tokens();
            for elem in input[1..].iter() {
                fmt_and_print(elem.to_string(), &tz, &custom_tokens);
            }
            println!("Timezone: {}", tz);
        }
    }
}

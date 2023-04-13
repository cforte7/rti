extern crate chrono;
use chrono_tz::Tz;
mod config;
use config::{
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

mod cli;
use cli::{parse_input, help, Action, ParsedInput};

pub type OkOrStringError = Result<Option<String>, String>;

fn fmt_and_print(arg: String, tz: &Tz, custom_tokens: &Vec<String>) {
    let maybe_int_parse = arg.parse::<i64>();
    let parsed_value = match maybe_int_parse {
        Ok(val) => epoch_to_datetime(val, tz),
        Err(_) => parse_arg(&arg, tz, custom_tokens),
    };

    println!("{}", format!("{} => {}", arg, parsed_value));
}




fn execute_action(input: ParsedInput) -> OkOrStringError {
    match input.action {
        Action::Help => help(),
        Action::SetTz => set_tz_config(input.second_arg),
        Action::ClearTz => clear_tz_config(),
        Action::AddToken => add_custom_token(input.second_arg),
        Action::RemoveToken => remove_custom_token(input.second_arg),
        Action::ViewTokens => view_tokens(),
        _ => {
            let tz: Tz = get_timezone();
            let custom_tokens: Vec<String> = get_custom_tokens();
            let combined_args = input.get_non_action_args();
            for elem in combined_args {
                fmt_and_print(elem.to_string(), &tz, &custom_tokens);
            }
            println!("Timezone: {}", tz);
            Ok(None)
        }
    }

}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let input = match parse_input(args) {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            return Err("".to_string());
        }
    };

    execute_action(input)?;
    Ok(())
}

use crate::OkOrStringError;

pub fn help() -> OkOrStringError {
    println!("RTI converts Unix epoch time to a human readable format and vice versa.");
    println!("Enter values to convert separated by a space.\n");
    println!("Additional commands:");
    println!("    help -  View this message.");
    println!("    set-tz - Set a configured timezone. Uses first argument after set-tz.");
    println!("    clear-tz - Clear timezone config.");
    println!("    add-token - Add a custom parsing token. Uses first argument after add-token. See https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html for syntax.");
    println!("    remove-token - Remove a custom parsing token. No changes made if the token doesn't exist.");
    println!("    view-tokens - See a list of stored custom parsing tokens.");
    Ok(None)
}

pub struct ParsedInput {
    pub action: Option<Action>,
    pub second_arg: Option<String>,
    pub date_args: Vec<String>
}


pub fn parse_input(input: Vec<String>) -> Result<ParsedInput, String> {
    let input_len = input.len();
    if input_len == 1 {
        return Err("Must include at least one argument!".to_string());
    }
    let maybe_action = match input[1].parse() {
        Ok(val) =>  {
            Action::from_string(val)
        },
        Err(_) => return Err("Error parsing arg.".to_string())
    };

    let second_arg : Option<String> = match input.get(2) {
        Some(val) => Some(val.clone()),
        None => None,
    };

    // if we have an action, we aren't doing any datetime parsing
    // so we just give empty list, otherwise all of our args
    // are datetimes to parse
    let date_args = match maybe_action {
        Some(_) => vec![],
        None => input[1..].iter().map(|x| x.to_owned()).collect(),
    };

    return Ok(ParsedInput {
        action: maybe_action,
        second_arg,
        date_args
    });
}

pub enum Action {
    Help,
    SetTz,
    ClearTz,
    AddToken,
    RemoveToken,
    ViewTokens
}


impl Action {
    fn from_string(input: String) -> Option<Action> {
        match input.as_str() {
            "help"  => Some(Action::Help),
            "set-tz"  => Some(Action::SetTz),
            "clear-tz"  => Some(Action::ClearTz),
            "add-token" => Some(Action::AddToken),
            "remove-token" => Some(Action::RemoveToken),
            "view-tokens" => Some(Action::ViewTokens),
            _  => None,
        }
    }
}
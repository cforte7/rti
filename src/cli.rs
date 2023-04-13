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
    pub action: Action,
    pub second_arg: Option<String>,
    pub other_args: Vec<String>
}

impl ParsedInput {
    pub fn get_non_action_args(&self) -> Vec<String> {
        // Turn second_arg and other_args into a single Vec.
        let mut args = match &self.second_arg {
            Some(arg) => vec![arg.to_owned()],
            None => vec![]
        };
        args.append(&mut self.other_args.clone());
        args
    }
}

pub fn parse_input(input: Vec<String>) -> Result<ParsedInput, &'static str> {
    let action: Action = match input[1].parse() {
        Ok(val) =>  {
            let a = Action::from_string(val)?;
            a
        },
        Err(_) => return Err("Must include at least one argument!")
    };

    let second_arg: Option<String> = match input[2].parse() {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    let other_args;
    if input.len() > 3 {
        other_args = input[2..].iter().map(|x| x.to_owned()).collect();
    } else{
        other_args = vec![];
    };
    return Ok(ParsedInput {
        action,
        second_arg,
        other_args
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
    fn from_string(input: String) -> Result<Action, &'static str> {
        match input.as_str() {
            "help"  => Ok(Action::Help),
            "set-tz"  => Ok(Action::SetTz),
            "clear-tz"  => Ok(Action::ClearTz),
            "add-token" => Ok(Action::AddToken),
            "remove-token" => Ok(Action::RemoveToken),
            "view-tokens" => Ok(Action::ViewTokens),
            _  => Err("Invalid action"),
        }
    }
}
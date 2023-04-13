
use chrono_tz::{ParseError, Tz, UTC};
use confy;
use serde::{Deserialize, Serialize};
use std::{fmt, env};
use crate::OkOrStringError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub default_timezone: Option<String>,
    pub custom_parsing_tokens : Option<Vec<String>>
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            default_timezone: None,
            custom_parsing_tokens: None
        }
    }
}

impl fmt::Display for MyConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timezone: {:?}", self.default_timezone)
    }
}

pub fn load_config() -> MyConfig {
    confy::load("rti").expect("Unable to load config.")
}

pub fn set_tz_config(tz_input: Option<String>) -> OkOrStringError {
    // Verify that this can be parsed as a timezone. Notify and exit if it can't.
    let timezone: Tz = match tz_input {
        Some(val) => match val.parse() {
            Ok(v) => v,
            Err(_) => return Err("Invalid timezone provided.".to_string())
        },
        None => return Err("Must provide timezone argument.".to_string())
    };
    let existing_config = load_config();
    let new_config = MyConfig {
        default_timezone: Some(timezone.to_string()),
        ..existing_config
    };
    match confy::store("rti", new_config) {
        Ok(_) => Ok(Some(format!("Timezone updated to {}", timezone))),
        Err(e) => {
            Err(format!("Error storing timezone: {}", e.to_string()))
        }
    }
}

pub fn clear_tz_config() -> OkOrStringError {
    // change default timezone to None and save.
    let existing_config = load_config();
    let new_config = MyConfig {
        default_timezone: None,
        ..existing_config
    };

    match confy::store("rti", new_config) {
        Ok(_) => Ok(Some("Timezone cleared.".to_string())),
        Err(e) => {
            Err(format!("Error storing timezone: {}", e.to_string()))
        }
    }
}

fn get_env_timezone() -> Option<Tz> {
    // Check for TIMEZONE env variable
    let env_timezone = env::var("TIMEZONE");
    if let Ok(env_tz) = env_timezone {
        // If TIMEZONE env var is found, try to parse into timezone
        // and panic if it cant be parsed
        let parsed_env_tz: Result<Tz, ParseError> = env_tz.parse();
        match parsed_env_tz {
            Ok(tz) => return Some(tz),
            _ => (panic!("Unable to parse TIMEZONE env variable.")),
        }
    };
    None
}

pub fn get_timezone() -> Tz {
    // Get Timezone, returning the first found in the following order:
    // 1. env
    // 2. config
    // 3. default to UTC

    let env_timezone = get_env_timezone();
    if let Some(tz) = env_timezone {
        return tz;
    }

    let existing_config = load_config();
    return if let Some(val) = existing_config.default_timezone {
        let tz: Tz = val.parse().unwrap();
        tz
    } else {
        UTC
    }
}

pub fn get_custom_tokens() -> Vec<String> {
    // Return vec of custom tokens and return an empty vec if none exist
    let existing_config = load_config();
    match existing_config.custom_parsing_tokens {
        Some(tokens) => tokens,
        None=>Vec::new()
    }
}

pub fn add_custom_token(new_token: Option<String>) -> OkOrStringError {
    let token: String = match new_token {
        Some(val) => val,
        None => return Err("Must provide timezone argument.".to_string())
    };

    let mut existing_tokens = match load_config().custom_parsing_tokens {
        Some(tokens) => tokens,
        None=>Vec::new()
    };

    existing_tokens.push(token);
    let existing_config = load_config();
    let new_config = MyConfig {
        custom_parsing_tokens: Some(existing_tokens),
        ..existing_config
    };
    match confy::store("rti", new_config) {
        Ok(_) => Ok(Some("Custom Token successfully added.".to_string())),
        Err(e) => {
            Err(format!("Error storing custom token: {}", e.to_string()))
        }
    }
    
}

pub fn remove_custom_token(to_remove: Option<String>) -> OkOrStringError {
    let token: String = match to_remove {
        Some(val) => val,
        None => return Err("Must provide token to remove.".to_string())
    };
    let existing_tokens: Vec<String> = match load_config().custom_parsing_tokens {
        Some(tokens) => tokens,
        None=>{
            return Ok(Some("No tokens to remove.".to_string()));
        }
    };
    let filtered_tokens: Vec<String> = existing_tokens
        .into_iter()
        .filter(|val| val!= &token)
        .collect();
    if filtered_tokens.len() == 0 {
        return Ok(Some("No matching token found.".to_string()));
    }

    let tokens_to_store = match filtered_tokens.len() {
        0 => None,
        _ => Some(filtered_tokens)
    };

    let new_config = MyConfig {
        custom_parsing_tokens: tokens_to_store,
        ..load_config()
    };

    match confy::store("rti", new_config) {
        Ok(_) => Ok(Some("Custom token removed.".to_string())),
        Err(e) => {
            Err(format!("Error removing custom token: {}", e.to_string()))
        }
    }
}

pub fn view_tokens() -> OkOrStringError {
    // get tokens from config and print them out
    let mut existing_tokens: Vec<String> = vec!["Custom datetime tokens:".to_string()];
    match load_config().custom_parsing_tokens {
        Some(mut tokens) => existing_tokens.append(&mut tokens),
        None => return Err("No custom tokens exist!".to_string())
    };
    Ok(Some(existing_tokens.join("\n").to_string()))
}

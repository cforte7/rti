pub mod config {
    use chrono_tz::{ParseError, Tz, UTC};
    use confy;
    use serde::{Deserialize, Serialize};
    use std::{fmt, env};

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

    pub fn set_tz_config(tz_input: String) {
        // Verify that this can be parsed as a timezone. Notify and exit if it can't.
        let parse_result: Result<Tz, ParseError> = tz_input.parse();
        if let Err(_) = parse_result {
            println!("Invalid Timezone! Settings not updated.");
            return;
        }
        let existing_config = load_config();
        let new_config = MyConfig {
            default_timezone: Some(tz_input.to_string()),
            ..existing_config
        };
        if let Ok(_) = confy::store("rti", new_config) {
            println!("Timezone updated to {}", tz_input)
        }
    }

    pub fn clear_tz_config() {
        // change default timezone to None and save.
        let existing_config = load_config();
        let new_config = MyConfig {
            default_timezone: None,
            ..existing_config
        };
        if let Ok(_) = confy::store("rti", new_config) {
            println!("Timezone cleared.")
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

    pub fn add_custom_token(new_token: String) {
        let mut existing_tokens = match load_config().custom_parsing_tokens {
            Some(tokens) => tokens,
            None=>Vec::new()
        };
        existing_tokens.push(new_token);
        let existing_config = load_config();
        let new_config = MyConfig {
            custom_parsing_tokens: Some(existing_tokens),
            ..existing_config
        };
        if let Ok(_) = confy::store("rti", new_config) {
            println!("Custom Token added");
        }
    }

    pub fn remove_custom_token(to_remove: String) {
        let existing_tokens: Vec<String> = match load_config().custom_parsing_tokens {
            Some(tokens) => tokens,
            None=>{
                println!("No tokens to remove.");
                return;
            }
        };
        let filtered_tokens: Vec<String> = existing_tokens
            .into_iter()
            .filter(|val| val!= &to_remove)
            .collect();

        let tokens_to_store = match filtered_tokens.len() {
            0 => None,
            _ => Some(filtered_tokens)
        };

        let new_config = MyConfig {
            custom_parsing_tokens: tokens_to_store,
            ..load_config()
        };


        if let Ok(_) = confy::store("rti", new_config) {
            println!("Token removed if it existed: {}", to_remove);
        }
    }

    pub fn view_tokens() {
        // get tokens from config and print them out
        let existing_tokens: Vec<String> = match load_config().custom_parsing_tokens {
            Some(tokens) => tokens,
            None=>{
                println!("No custom tokens exist!");
                return;
            }
        };
        println!("Custom datetime tokens:");
        for val in existing_tokens {
            println!("{}", val);
        }
    }
}

pub mod config {
    use chrono_tz::{ParseError, Tz, UTC};
    use confy;
    use serde::{Deserialize, Serialize};
    use std::{fmt, env};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MyConfig {
        pub default_timezone: Option<String>,
    }

    impl ::std::default::Default for MyConfig {
        fn default() -> Self {
            Self {
                default_timezone: None,
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

        // first check for tz set by env var
        // If one is found, return it
        let env_timezone = get_env_timezone();
        if let Some(tz) = env_timezone {
            return tz;
        }

        // Check if config exists and if it does, return that
        // else default to UTC
        let existing_config = load_config();
        return if let Some(val) = existing_config.default_timezone {
            let tz: Tz = val.parse().unwrap();
            tz
        } else {
            UTC
        }
    }
}

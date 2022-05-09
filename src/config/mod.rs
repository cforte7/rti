pub mod config {
    use chrono_tz::{ParseError, Tz};
    use confy;
    use serde::{Deserialize, Serialize};
    use std::fmt;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MyConfig {
        pub default_timezone: Option<String>,
    }
    impl MyConfig {
        pub fn get_timezone_str(self) -> String {
            if let Some(val) = load_config().default_timezone {
                val
            } else {
                String::from("Local")
            }
        }
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
}

pub mod config {
    use confy;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MyConfig {
        default_timezone: Option<String>,
    }

    impl ::std::default::Default for MyConfig {
        fn default() -> Self {
            Self {
                default_timezone: None,
            }
        }
    }
    pub fn load_config() -> MyConfig {
        confy::load("rti").expect("Unable to load config.")
    }
}

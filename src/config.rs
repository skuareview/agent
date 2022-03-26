use serde_derive::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Default, Deserialize)]
pub struct Config {
    token: String,
}

impl Config {
    pub fn get_config(&mut self) {
        let path_config_file = "/etc/phantom-agent/config.toml";

        // Read the contents of the file using a `match` block
        // to return the `data: Ok(c)` as a `String`
        // or handle any `errors: Err(_)`.
        let contents = match fs::read_to_string(path_config_file) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", path_config_file);
                exit(1);
            }
        };

        // Use a `match` block to return the
        // file `contents` as a `Data struct: Ok(d)`
        // or handle any `errors: Err(_)`.
        let data: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", path_config_file);
                exit(1);
            }
        };
        // Verify if the token is already set
        if (data.token) == "" {
            println!(
                "The token is empty, please fill the key 'token' here {}",
                path_config_file
            );
            exit(1);
        } else {
            self.set_token(data.token)
        }
    }
    // Setters
    fn set_token(&mut self, token: String) {
        self.token = token;
    }
    // Getters
    pub fn get_token(self) -> String {
        return self.token;
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBookShelfConfig {
    pub server: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub audio_book_shelf: AudioBookShelfConfig,
}

impl Config {
    pub fn load_config() -> Self {
        let config_path: String = String::from("./config.toml");

        let toml_content: String = match std::fs::read_to_string(config_path) {
            Ok(toml_content) => toml_content,
            Err(error) => panic!("{}", error), 
        };

        match toml::from_str(&toml_content) {
            Ok(config) => config,
            Err(error) => panic!("{}", error),
        }
    }
}
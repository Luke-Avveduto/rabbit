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
    fn load_config() -> Self {
        let config_path: String = String::from("./config.toml");
        let toml_content = std::fs::read_to_string(config_path).unwrap();

        let config: Config = toml::from_str(&toml_content).unwrap();
        config
    }
}


fn main() {
    println!("Hello, world!");

    let config = Config::load_config();

    println!("{:?}", config)
}

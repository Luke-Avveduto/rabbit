mod config;

use config::Config;

fn main() {
    let config: Config = config::Config::load_config();

    println!("{:?}", config)
}

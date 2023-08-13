mod config;

use config::*;

fn main() {
    let config: Config = read_config("./App.toml");
    println!("Path: {0}", config.log_path);
}

mod config;

use config::*;
use log::*;

fn main() {
    env_logger::init();
    let config: Config = read_config("./App.toml");

    debug!("config.log_path: {0}", config.log_path);
}

mod config;

use config::*;

fn main() {
    let config: Config = read_config();
    println!("Path: {0}", config.log_path);
}

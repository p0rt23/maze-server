mod config;

use config::*;
use log::*;
use std::io::{self, Write};

fn main() {
    init_logger();
    run();
}

fn init_logger() {
    env_logger::init();
    let config: Config = read_config("./App.toml");
    debug!("config.log_path: {0}", config.log_path);
}

fn run() {
    loop {
        let reader = io::stdin();
        let mut buffer: String = String::new();

        print!("Command: ");
        io::stdout().flush().unwrap();
        reader.read_line(&mut buffer).expect("Failed to read line"); // TODO

        buffer = buffer.trim().to_string();

        if buffer == "quit" {
            debug!("Execute: {}", buffer);
            break;
        }
    }
}

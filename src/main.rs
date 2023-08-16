mod config;

use std::error::Error;

use config::*;
use log::*;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();

    let config_path = "./App.toml";
    let config: Config = init_config(config_path);

    debug!("Binding to interface.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;

    debug!("Listening on: {}", config.port);
    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("Connection from: {}", addr.to_string());

        process(stream);
    }
}

fn init_config(config_path: &str) -> Config {
    debug!("Reading config file: {}", config_path);
    read_config(&config_path)
}

fn init_logger() {
    env_logger::init();
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    stream.write_all(b"hello world!").await?;
    stream.shutdown().await?;
    Ok(())
}

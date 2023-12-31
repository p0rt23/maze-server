mod config;

use config::*;
use log::*;
use std::error::Error;
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
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    debug!("Listening on: {}", config.port);
    loop {
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            debug!("Connection from: {}", addr.to_string());
            if let Err(e) = process(stream).await {
                error!("Error: {:?}", e);
            }
        });
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

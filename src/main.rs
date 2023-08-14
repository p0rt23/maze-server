mod config;

use config::*;
use log::*;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    init_logger();

    let config_path = "./App.toml";
    let config: Config = init_config(config_path);

    debug!("Binding to interface.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();

    debug!("Listening on: {}", config.port);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

fn init_config(config_path: &str) -> Config {
    debug!("Reading config file: {}", config_path);
    read_config(&config_path)
}

fn init_logger() {
    env_logger::init();
}

async fn process(socket: TcpStream) {
    let std_tcp_stream = socket.into_std().unwrap();
    std_tcp_stream.set_nonblocking(false).unwrap();
}

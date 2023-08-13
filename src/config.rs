use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub log_path: String,
}

pub fn read_config() -> Config {
    let config: Config = Figment::new()
        .merge(Toml::file("App.toml"))
        .extract()
        .unwrap();
    config
}

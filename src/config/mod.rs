use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub log_path: String,
}

pub fn read_config(config_path: &str) -> Config {
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .extract()
        .unwrap(); // TODO: Handle exception
    config
}

#[cfg(test)]
mod tests {
    use crate::config::*;

    #[test]
    fn can_read_log_path() {
        let log_path = "/var/log";
        let config = read_config("src/config/config_test.toml");
        assert_eq!(log_path, config.log_path);
    }
}

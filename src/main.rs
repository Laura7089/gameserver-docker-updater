#[macro_use]
extern crate log;
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;

mod container;
mod steam;

const DEFAULT_CONFIG_PATH: &'static str = "/config.yml";

#[derive(Deserialize, Debug)]
struct Config {
    steam_api_key: String,
    #[serde(with = "humantime_serde")]
    check_interval: Duration,
    containers: Vec<container::Container>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting gameserver updater");

    let mut args = std::env::args();
    args.next();
    let config_path = args
        .next()
        .unwrap_or(std::env::var("UPDATER_CONFIG_PATH").unwrap_or(DEFAULT_CONFIG_PATH.into()));
    info!("Config file path is {}", config_path);
    let config_path = Path::new(&config_path);
    if !config_path.exists() {
        error!("Path {} doesn't exist", config_path.display());
        return Err("No config file found!".into());
    }
    let config: Config = serde_yaml::from_str(&std::fs::read_to_string(&config_path)?)?;

    loop {
        std::thread::sleep(config.check_interval);
    }
}

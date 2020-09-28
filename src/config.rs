use serde::Deserialize;
use std::env;
use std::path::Path;
use std::time::Duration;

const DEFAULT_CONFIG_PATH: &'static str = "/config.yml";
const DEFAULT_STATE_PATH: &'static str = "/updater_state";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub steam_api_key: String,
    #[serde(with = "humantime_serde")]
    pub check_interval: Duration,
    pub containers: Vec<crate::container::Container>,
    #[serde(default)]
    pub state_directory: String,
    #[serde(default)]
    pub connect_mode: Option<DockerConnectMode>,
}

#[derive(Deserialize, Debug)]
pub enum DockerConnectMode {
    UnixSocket,
    Http,
    SSL,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next();
    let config_path_raw: String;
    let config_path = if let Some(path_raw) = args.next() {
        info!("Got config file path {} from arguments", path_raw);
        config_path_raw = path_raw;
        Path::new(&config_path_raw)
    } else if let Ok(path_raw) = env::var("UPDATER_CONFIG_PATH") {
        info!("Got config file path {} from environment", path_raw);
        config_path_raw = path_raw;
        Path::new(&config_path_raw)
    } else {
        info!("Default config path {} selected", DEFAULT_CONFIG_PATH);
        Path::new(DEFAULT_CONFIG_PATH)
    };

    if !config_path.exists() {
        return Err(format!("Config file {} not found!", config_path.display()).into());
    }
    let mut config_basic: Config = serde_yaml::from_str(&std::fs::read_to_string(&config_path)?)?;

    if config_basic.state_directory == "" {
        match env::var("UPDATER_STATE_PATH") {
            Ok(p) => {
                info!("Got state directory {} from environment", p);
                config_basic.state_directory = p;
            }
            Err(_) => {
                info!("State directory defaulting to {}", DEFAULT_STATE_PATH);
                config_basic.state_directory = DEFAULT_STATE_PATH.to_owned();
            }
        }
    }

    if let None = config_basic.connect_mode {
        info!("Docker daemon connection method defaulting to unix socket");
        config_basic.connect_mode = Some(DockerConnectMode::UnixSocket);
    }

    Ok(config_basic)
}

use serde::Deserialize;
use std::env;
use std::path::Path;
use std::time::Duration;

const DEFAULT_CONFIG_PATH: &'static str = "./config.yml";
const DEFAULT_STATE_PATH: &'static str = "./state";

/// Global application configuration
///
/// Call get_config() to get this from the various places the application can be configured
/// through.
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub steam_api_key: String,
    #[serde(with = "humantime_serde")]
    pub check_interval: Duration,
    pub containers: Vec<crate::container::Container>,
    #[serde(default)]
    pub state_directory: String,
    #[serde(default)]
    pub connect_mode: Option<DockerConnectMode>,
}

/// The different methods of connecting to the Docker daemon
///
/// Currently, support for only these 3 is planned. TLS/SSL may prove to be problematic to support,
/// in which case I will most likely drop it.
#[derive(Deserialize, Debug)]
pub enum DockerConnectMode {
    UnixSocket,
    Http,
    SSL,
}

/// Get the global config for the current program instance
///
/// This will read from disk, args and environment so unfortunately the contents are messy.
pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Get args and consume the first one to remove the program invocation string
    let mut args = env::args();
    args.next();

    // Get the config path from the various sources, log where we got it from
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

    // Deserialise
    let mut config_basic: Config = serde_yaml::from_str(&std::fs::read_to_string(&config_path)?)?;

    // TODO: Should the environment override the config file?
    // Get the API key from the environnment if it's not in the config
    if config_basic.steam_api_key == "" {
        match env::var("UPDATER_STEAM_API_KEY") {
            Ok(k) => {
                info!("Got steam API key from environment");
                config_basic.steam_api_key = k;
            }
            Err(_) => {
                return Err("Steam API key not found in configuration file or environment".into());
            }
        }
    }

    // Get the state directory from the environment if it's not in the config
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

    // Default to connecting through the Docker Unix Socket
    if let None = config_basic.connect_mode {
        info!("Docker daemon connection method defaulting to unix socket");
        config_basic.connect_mode = Some(DockerConnectMode::UnixSocket);
    }

    Ok(config_basic)
}

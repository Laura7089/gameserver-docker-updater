use serde::Deserialize;
use std::env;
use std::path::PathBuf;
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
    steam_api_key: String,
    #[serde(with = "humantime_serde")]
    check_interval: Duration,
    containers: Vec<crate::container::Container>,
    #[serde(default)]
    state_directory: PathBuf,
    #[serde(default)]
    connect_mode: DockerConnectMode,
}

/// The different methods of connecting to the Docker daemon
///
/// Currently, support for only these 3 is planned. TLS/SSL may prove to be problematic to support,
/// in which case I will most likely drop it.
#[derive(Deserialize, Debug)]
pub enum DockerConnectMode {
    #[serde(rename(deserialize = "unix_socket"))]
    UnixSocket,
    #[serde(rename(deserialize = "windows_pipe"))]
    WindowsPipe,
    #[serde(rename(deserialize = "http"))]
    Http,
    #[serde(rename(deserialize = "ssl"))]
    SSL,
}

impl Default for DockerConnectMode {
    #[cfg(unix)]
    fn default() -> Self {
        DockerConnectMode::UnixSocket
    }

    #[cfg(windows)]
    fn default() -> Self {
        DockerConnectMode::WindowsPipe
    }
}

impl Config {
    /// Get the global config for the current program instance
    ///
    /// This will read from disk, args and environment so unfortunately the contents are messy.
    pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
        // Get args and consume the first one to remove the program invocation string
        let mut args = env::args();
        args.next();

        // Get the config path from the various sources, log where we got it from
        let config_path = PathBuf::from(if let Some(path_raw) = args.next() {
            info!("Got config file path {} from arguments", path_raw);
            path_raw
        } else if let Ok(path_raw) = env::var("UPDATER_CONFIG_PATH") {
            info!("Got config file path {} from environment", path_raw);
            path_raw
        } else {
            info!("Default config path {} selected", DEFAULT_CONFIG_PATH);
            DEFAULT_CONFIG_PATH.to_owned()
        });
        if !config_path.exists() {
            return Err(format!("Config file {} not found!", config_path.display()).into());
        }

        // Deserialise
        let mut ret: Config = serde_yaml::from_str(&std::fs::read_to_string(&config_path)?)?;

        // Get the API key from the environnment and override the config file
        match env::var("UPDATER_STEAM_API_KEY") {
            Ok(k) => {
                info!("Got steam API key from environment");
                ret.steam_api_key = k;
            }
            Err(_) => {
                if ret.steam_api_key == "" {
                    return Err(
                        "Steam API key not found in configuration file or environment".into(),
                    );
                } else {
                    info!("Got steam API key from config file");
                }
            }
        }

        // Get the state directory from the environment if it's not in the config
        if &ret.state_directory.to_str().unwrap() == &"" {
            match env::var("UPDATER_STATE_PATH") {
                Ok(p) => {
                    info!("Got state directory {} from environment", p);
                    ret.state_directory = PathBuf::from(p);
                }
                Err(_) => {
                    info!("State directory defaulting to {}", DEFAULT_STATE_PATH);
                    ret.state_directory = PathBuf::from(DEFAULT_STATE_PATH);
                }
            }
        }

        Ok(ret)
    }

    pub fn consume(
        self,
    ) -> (
        Vec<crate::container::Container>,
        String,
        Duration,
        PathBuf,
        DockerConnectMode,
    ) {
        (
            self.containers,
            self.steam_api_key,
            self.check_interval,
            self.state_directory,
            self.connect_mode,
        )
    }
}

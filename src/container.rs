use crate::steam::{get_game_version, SteamVersion};
use bollard::Docker;
use futures::executor;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub appid: u64,
    #[serde(default)]
    current_version: SteamVersion,
    action: UpdateAction,
    #[serde(default)]
    options: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum UpdateAction {
    #[serde(rename = "build")]
    DockerBuild,
    #[serde(rename = "restart")]
    DockerRestart,
    #[serde(rename = "custom")]
    Custom,
}

impl Container {
    pub fn init(&mut self, key: &str, state_dir: &str) {
        debug!(
            "Initialising container {} (appid {})",
            self.name, self.appid
        );
        let save_path_raw = self.save_file(state_dir);
        let save_path = Path::new(&save_path_raw);

        if save_path.exists() {
            info!(
                "Saved state for {} found at {}",
                self.name,
                save_path.display()
            );
            let content = match std::fs::read_to_string(save_path) {
                Ok(c) => c,
                Err(e) => panic!("FAILED to read state file {}: {}", save_path.display(), e),
            };
            let saved_version: Self = match serde_json::from_str(&content) {
                Ok(s) => s,
                Err(e) => panic!(
                    "FAILED to deserialise state from file {}: {}",
                    save_path.display(),
                    e
                ),
            };
            self.current_version = saved_version.current_version;
        } else {
            match get_game_version(key, self.appid) {
                Ok(v) => {
                    info!(
                        "Initialised container {} (appid {}): version {} found",
                        self.name, self.appid, v
                    );
                    self.current_version = v;
                }
                Err(e) => error!(
                    "Container {} (appid {}) Initialisation FAILED: {}",
                    self.name, self.appid, e
                ),
            }
        }
    }

    pub fn update(&mut self, api_key: &str, docker_client: &Docker) {
        debug!("Checking version of {}", self.name);
        let new_version = match get_game_version(&api_key, self.appid) {
            Ok(v) => {
                debug!(
                    "Got new version for container {} (appid {}): {}",
                    self.name, self.appid, v
                );
                v
            }
            Err(e) => {
                error!(
                    "Container {} (appid {}) version check FAILED: {}",
                    self.name, self.appid, e
                );
                return;
            }
        };

        if self.current_version == new_version {
            info!(
                "{} is UP-TO-DATE at version {}",
                self.name, self.current_version
            );
            return;
        }

        match self.action {
            UpdateAction::DockerRestart => {
                if let Ok(_) = self.restart(docker_client) {
                    self.current_version = new_version;
                }
            }
            _ => todo!(),
        }
    }

    pub fn restart(&self, docker_client: &Docker) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Restarting container {}", self.name);
        match executor::block_on(docker_client.restart_container(&self.name, None)) {
            Err(e) => {
                error!("FAILED to restart container {}: {}", self.name, &e);
                Err(Box::new(e))
            }
            _ => {
                info!("Container {} successfully updated via restart", self.name);
                Ok(())
            }
        }
    }

    pub fn save_state(&self, dir: &Path) {
        debug!("Saving container {}'s state to disk", self.name);
        let serial = match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(e) => panic!("FAILED to serialise container {}: {}", self.name, e),
        };
        let save_path_raw = self.save_file(dir.to_str().unwrap());
        let save_path = Path::new(&save_path_raw);
        match std::fs::write(save_path, serial) {
            Err(e) => panic!("FAILED saving container {} state to disk: {}", self.name, e),
            _ => (),
        }
    }

    fn save_file(&self, dir: &str) -> String {
        Path::new(dir)
            .join(Path::new(&format!("{}.json", self.name)))
            .to_str()
            .unwrap()
            .to_owned()
    }
}

use crate::steam::{get_game_version, SteamVersion};
use bollard::Docker;
use futures::executor;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub appid: u64,
    #[serde(default)]
    current_version: SteamVersion,
    action: UpdateAction,
    #[serde(default)]
    options: BTreeMap<String, String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum UpdateAction {
    #[serde(rename(deserialize = "build"))]
    DockerBuild,
    #[serde(rename(deserialize = "restart"))]
    DockerRestart,
    #[serde(rename(deserialize = "custom"))]
    Custom,
}

impl Container {
    pub fn init(&mut self, key: &str) {
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

    pub fn update(&mut self, api_key: &str, docker_client: &Docker) {
        info!("Checking version of {}", self.name);
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
            info!("{} is UP-TO-DATE", self.name);
            return;
        }

        match self.action {
            UpdateAction::DockerRestart => {
                if let Ok(_) = self.restart(docker_client) {
                    info!("Container {} successfully restarted", self.name);
                    self.current_version = new_version;
                }
            }
            _ => todo!(),
        }
    }

    pub fn restart(&self, docker_client: &Docker) -> Result<(), Box<dyn std::error::Error>> {
        match executor::block_on(docker_client.restart_container(&self.name, None)) {
            Err(e) => {
                error!("FAILED to restart container {}: {}", self.name, &e);
                Err(Box::new(e))
            }
            _ => Ok(()),
        }
    }
}

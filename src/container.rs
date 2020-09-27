use crate::steam::SteamVersion;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub appid: u64,
    #[serde(default)]
    current_version: Option<SteamVersion>,
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

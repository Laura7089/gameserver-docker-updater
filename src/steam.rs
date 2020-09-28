use serde::Deserialize;
use std::collections::BTreeMap;

const GAME_MANIFEST_BASE_URL: &'static str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2";

pub type SteamVersion = u64;

#[derive(Deserialize, Debug)]
struct SteamGame {
    #[serde(rename(deserialize = "gameName"))]
    name: String,
    #[serde(rename(deserialize = "gameVersion"))]
    version: String,
}

pub fn get_game_version(key: &str, appid: u64) -> Result<SteamVersion, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/?key={}&appid={}&format=json",
        GAME_MANIFEST_BASE_URL, key, appid
    );
    debug!("Making request to {}", url);

    let body = reqwest::blocking::get(&url)?.text()?;
    let game_schema: BTreeMap<String, SteamGame> = serde_json::from_str(&body)?;

    Ok(game_schema.get("game").unwrap().version.parse()?)
}

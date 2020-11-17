use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Naked endpoint for getting steam store manifests
///
/// From [here](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetSchemaForGame_.28v2.29).
const GAME_MANIFEST_BASE_URL: &'static str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2";

/// Type alias to allow universal use across application
pub type SteamVersion = u64;

/// Helper struct for steam deserialisation
#[derive(Deserialize, Debug)]
struct SteamGame {
    #[serde(rename(deserialize = "gameName"))]
    name: String,
    #[serde(rename(deserialize = "gameVersion"))]
    version: String,
}

/// Get the version integer of a game from steam
///
/// Will try to log helpful messages to hint at avenues for fixes on failure.
pub async fn get_game_version(key: &str, appid: u64) -> Result<SteamVersion, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/?key={}&appid={}&format=json",
        GAME_MANIFEST_BASE_URL, key, appid
    );
    debug!("Making request to {}", url);

    let result = reqwest::get(&url).await?;
    match result.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
            let body = result.text()?;
            let game_schema: BTreeMap<String, SteamGame> = serde_json::from_str(&body)?;
            Ok(game_schema.get("game").unwrap().version.parse()?)
        }
        StatusCode::FORBIDDEN => Err("Steam API returned 403 - check your API key".into()),
        s @ _ => Err(format!("Steam API returned {}", s.as_u16()).into()),
    }
}

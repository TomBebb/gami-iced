use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_key: String,
    pub steam_id: String,
}

impl Config {
    pub async fn load() -> Config {
        if !CONF_PATH.exists() {
            return Config::default();
        }
        tokio::task::spawn_blocking(|| {
            serde_json::from_reader(std::fs::File::open(CONF_PATH.as_path()).unwrap()).unwrap()
        })
        .await
        .unwrap()
    }
}
const CONF_PATH: Lazy<PathBuf> = Lazy::new(|| gami_sdk::resolve_addon_config_json_path("steam"));

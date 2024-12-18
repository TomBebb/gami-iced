use crate::models::MyTheme;
use bitcode::{Decode, Encode};
use gami_sdk::BASE_DATA_DIR;
use std::cell::LazyCell;
use std::path::PathBuf;

const SETTINGS_PATH: LazyCell<PathBuf> =
    LazyCell::new(|| BASE_DATA_DIR.join("desktop_settings.bin"));
#[derive(Encode, Decode, Default, Debug, PartialEq, Clone)]
pub struct Settings {
    pub appearance: AppearanceSettings,
}
#[derive(Encode, Decode, Default, Debug, PartialEq, Clone)]
pub struct AppearanceSettings {
    pub theme: MyTheme,
}

pub async fn save_async(settings: &Settings) -> std::io::Result<()> {
    let encoded: Vec<u8> = bitcode::encode(settings);
    tokio::fs::write(&*SETTINGS_PATH, &encoded).await?;
    Ok(())
}

pub fn save(settings: &Settings) -> std::io::Result<()> {
    let encoded: Vec<u8> = bitcode::encode(settings);
    std::fs::write(&*SETTINGS_PATH, &encoded)?;
    Ok(())
}

pub async fn load_async() -> std::io::Result<Settings> {
    let bytes = tokio::fs::read(&*SETTINGS_PATH).await?;
    let decoded: Settings = bitcode::decode(&bytes).unwrap();
    Ok(decoded)
}

pub fn load() -> std::io::Result<Settings> {
    let bytes = std::fs::read(&*SETTINGS_PATH)?;
    let decoded: Settings = bitcode::decode(&bytes).unwrap();
    Ok(decoded)
}

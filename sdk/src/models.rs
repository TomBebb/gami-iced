use ::safer_ffi::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, SystemTime};
use url::Url;
pub trait IsGameLibraryRef {
    fn get_name(&self) -> &str;
    fn get_library_type(&self) -> &str;
    fn get_library_id(&self) -> &str;
}
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive_ReprC]
#[repr(opaque)]
pub struct GameLibraryRef {
    pub name: String,
    pub library_type: String,
    pub library_id: String,
}
impl IsGameLibraryRef for GameLibraryRef {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_library_type(&self) -> &str {
        &self.library_type
    }

    fn get_library_id(&self) -> &str {
        &self.library_id
    }
}
impl fmt::Display for GameLibraryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}: {})",
            self.name, self.library_type, self.library_id
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
#[derive_ReprC] // <- `::safer_ffi`'s attribute
pub enum GameInstallStatus {
    Installed,
    Installing,
    InLibrary,
    Queued,
}
impl Default for GameInstallStatus {
    fn default() -> Self {
        GameInstallStatus::InLibrary
    }
}
#[derive_ReprC]
#[repr(opaque)]
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ScannedGameLibraryMetadata {
    pub name: String,
    pub library_type: String,
    pub library_id: String,

    pub last_played: Option<SystemTime>,
    pub install_status: GameInstallStatus,
    pub playtime: Duration,
    pub icon_url: Option<Url>,
}
impl IsGameLibraryRef for ScannedGameLibraryMetadata {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_library_type(&self) -> &str {
        &self.library_type
    }

    fn get_library_id(&self) -> &str {
        &self.library_id
    }
}

#[derive_ReprC]
#[repr(opaque)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GameData {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub play_time: Duration,
    pub install_status: GameInstallStatus,
    pub release_date: Option<SystemTime>,
    pub last_played: Option<SystemTime>,
    pub icon_url: Option<String>,
    pub header_url: Option<String>,
    pub logo_url: Option<String>,
    pub hero_url: Option<String>,
    pub library_type: String,
    pub library_id: String,
}

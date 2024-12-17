use ::safer_ffi::prelude::*;
use chrono::{DateTime, Duration, Utc};
use safer_ffi::option::TaggedOption;
use safer_ffi::String;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::string::String as RString;
pub trait IsGameLibraryRef {
    fn get_name(&self) -> &str;
    fn get_library_type(&self) -> &str;
    fn get_library_id(&self) -> &str;
}
#[derive(Debug, Clone)]
#[derive_ReprC]
#[repr(C)]
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
#[derive_ReprC]
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
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScannedGameLibraryMetadata {
    pub name: String,
    pub library_type: String,
    pub library_id: String,

    pub last_played_epoch: TaggedOption<u64>,
    pub install_status: GameInstallStatus,
    pub playtime_secs: u64,
    pub icon_url: TaggedOption<String>,
}
impl Default for ScannedGameLibraryMetadata {
    fn default() -> Self {
        Self {
            name: "".into(),
            library_type: "".into(),
            library_id: "".into(),
            last_played_epoch: TaggedOption::None,
            install_status: GameInstallStatus::InLibrary,
            playtime_secs: 0,
            icon_url: TaggedOption::None,
        }
    }
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

#[derive(Clone, Debug, Default)]
pub struct GameData {
    pub id: i32,
    pub name: RString,
    pub description: RString,
    pub play_time: Duration,
    pub install_status: GameInstallStatus,
    pub release_date: Option<DateTime<Utc>>,
    pub last_played: Option<DateTime<Utc>>,
    pub icon_url: Option<RString>,
    pub header_url: Option<RString>,
    pub logo_url: Option<RString>,
    pub hero_url: Option<RString>,
    pub library_type: RString,
    pub library_id: RString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSchemaMetadata {
    pub hint: RString,
    pub name: RString,
    pub kind: ConfigSchemaKind,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ConfigSchemaKind {
    String,
    Int,
    Boolean,
}

use crate::GameCommon;
use ::safer_ffi::prelude::*;
use chrono::{DateTime, Duration, NaiveDate, TimeDelta, Utc};
use safer_ffi::option::TaggedOption;
use safer_ffi::string::str_ref;
use safer_ffi::{String, Vec};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::string::String as RString;
use std::vec::Vec as RVec;

pub trait IsGameLibraryRef {
    fn get_name(&self) -> &str;
    fn get_library_type(&self) -> &str;
    fn get_library_id(&self) -> &str;
}
#[derive(Debug, Copy, Clone)]
#[derive_ReprC]
#[repr(C)]
pub struct GameLibraryRef<'a> {
    pub name: str_ref<'a>,
    pub library_type: str_ref<'a>,
    pub library_id: str_ref<'a>,
}

impl<'a> GameLibraryRef<'a> {
    pub fn to_owned(self) -> GameLibraryRefOwned {
        GameLibraryRefOwned {
            library_id: self.library_id.into(),
            library_type: self.library_type.into(),
            name: self.name.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameLibraryRefOwned {
    pub name: RString,
    pub library_type: RString,
    pub library_id: RString,
}

impl From<GameLibraryRef<'_>> for GameLibraryRefOwned {
    fn from(game_library: GameLibraryRef<'_>) -> Self {
        Self {
            name: game_library.name.into(),
            library_type: game_library.library_type.into(),
            library_id: game_library.library_id.into(),
        }
    }
}
impl GameLibraryRefOwned {
    pub fn as_ref(&self) -> GameLibraryRef {
        GameLibraryRef {
            name: self.name.as_str().into(),
            library_type: self.library_type.as_str().into(),
            library_id: self.library_id.as_str().into(),
        }
    }
}

impl<'a> PartialEq for GameLibraryRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        &*self.name == &*other.name
            && &*self.library_type == &*other.library_type
            && &*self.library_id == &*other.library_id
    }

    fn ne(&self, other: &Self) -> bool {
        !Self::eq(self, other)
    }
}
impl<'a> Eq for GameLibraryRef<'a> {}
impl<'a> Hash for GameLibraryRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.library_type.hash(state);
        self.library_id.hash(state);
    }
}
impl<'a> fmt::Display for GameLibraryRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}: {})",
            self.name.as_str(),
            self.library_type.as_str(),
            self.library_id.as_str()
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

impl fmt::Display for GameInstallStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            GameInstallStatus::Installed => "Installed",
            GameInstallStatus::Installing => "Installing",
            GameInstallStatus::InLibrary => "In library",
            GameInstallStatus::Queued => "Queued",
        })
    }
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

impl Into<GameData> for ScannedGameLibraryMetadata {
    fn into(self) -> GameData {
        GameData {
            name: self.name.into(),
            library_type: self.library_type.into(),
            library_id: self.library_id.into(),
            last_played: self
                .last_played_epoch
                .into_rust()
                .and_then(|v| DateTime::from_timestamp(v as i64, 0)),
            install_status: self.install_status,
            play_time: Duration::from(TimeDelta::seconds(self.playtime_secs as i64)),
            icon_url: self.icon_url.into_rust().map(RString::from),
            ..GameData::default()
        }
    }
}
impl GameCommon for ScannedGameLibraryMetadata {
    fn get_ref(&self) -> GameLibraryRef {
        let id_str: &str = &self.library_id;
        let ty_str: &str = &self.library_type;
        let name_str: &str = &self.name;
        GameLibraryRef {
            library_id: id_str.into(),
            library_type: ty_str.into(),
            name: name_str.into(),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GenreData {
    pub name: String,
    pub library_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct RGenreData {
    pub name: RString,
    pub library_id: RString,
}
impl From<GenreData> for RGenreData {
    fn from(genre_data: GenreData) -> Self {
        Self {
            name: genre_data.name.into(),
            library_id: genre_data.library_id.into(),
        }
    }
}
impl From<RGenreData> for GenreData {
    fn from(genre_data: RGenreData) -> Self {
        Self {
            name: genre_data.name.into(),
            library_id: genre_data.library_id.into(),
        }
    }
}
impl PartialEq for GenreData {
    fn eq(&self, other: &Self) -> bool {
        self.name.trim_end() == other.name.trim_end()
            && self.library_id.trim_end() == other.library_id.trim_end()
    }
}
impl Eq for GenreData {}
impl Hash for GenreData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.library_id.hash(state);
    }
}
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GameMetadata {
    pub description: TaggedOption<String>,
    pub developers: Vec<String>,
    pub genres: Vec<GenreData>,
    pub platforms: Vec<String>,
    pub publishers: Vec<String>,
    pub series: Vec<String>,
    pub tags: Vec<String>,
    pub release_date_timestamp: TaggedOption<u32>,
    pub last_played_timestamp: TaggedOption<u32>,
    pub icon_url: TaggedOption<String>,
    pub cover_url: TaggedOption<String>,
    pub header_url: TaggedOption<String>,
}

impl Default for GameMetadata {
    fn default() -> Self {
        Self {
            description: TaggedOption::None,
            developers: Vec::EMPTY,
            genres: Vec::EMPTY,
            tags: Vec::EMPTY,
            platforms: Vec::EMPTY,
            publishers: Vec::EMPTY,
            series: Vec::EMPTY,
            release_date_timestamp: TaggedOption::None,
            last_played_timestamp: TaggedOption::None,
            icon_url: TaggedOption::None,
            cover_url: TaggedOption::None,
            header_url: TaggedOption::None,
        }
    }
}
pub trait EditableEnum: fmt::Display + Sized + PartialEq + 'static {
    const ALL: &'static [Self];
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CompletionStatus {
    #[default]
    Backlog,
    Playing,
    Played,
    OnHold,
}

impl EditableEnum for CompletionStatus {
    const ALL: &'static [Self] = &[Self::Backlog, Self::Playing, Self::Played, Self::OnHold];
}
impl fmt::Display for CompletionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CompletionStatus::Backlog => "Backlog",
            CompletionStatus::Playing => "Playing",
            CompletionStatus::Played => "Played",
            CompletionStatus::OnHold => "On Hold",
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameData {
    pub id: i32,
    pub name: RString,
    pub description: RString,
    pub genres: RVec<GenreData>,
    pub play_time: Duration,
    pub install_status: GameInstallStatus,
    pub release_date: Option<NaiveDate>,
    pub last_played: Option<DateTime<Utc>>,
    pub icon_url: Option<RString>,
    pub header_url: Option<RString>,
    pub cover_url: Option<RString>,
    pub library_type: RString,
    pub library_id: RString,
    pub completion_status: CompletionStatus,
}
impl GameData {
    pub fn extend(&mut self, metadata: GameMetadata) {
        self.genres = metadata
            .genres
            .into_iter()
            .map(|v| v.clone().into())
            .collect();
        if let TaggedOption::Some(description) = metadata.description {
            self.description = description.into();
        }
        if let TaggedOption::Some(icon_url) = metadata.icon_url {
            self.icon_url = Some(icon_url.into());
        }
        if let TaggedOption::Some(header_url) = metadata.header_url {
            self.header_url = Some(header_url.into());
        }
        if let TaggedOption::Some(cover_url) = metadata.cover_url {
            self.cover_url = Some(cover_url.into());
        }
        if let TaggedOption::Some(release_date) = metadata.release_date_timestamp {
            self.release_date = Some(
                DateTime::from_timestamp(release_date as i64, 0)
                    .unwrap()
                    .date_naive(),
            );
        }
    }
}
impl GameCommon for GameData {
    fn get_ref(&self) -> GameLibraryRef {
        GameLibraryRef {
            library_id: self.library_id.as_str().into(),
            library_type: self.library_type.as_str().into(),
            name: self.name.as_str().into(),
        }
    }
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

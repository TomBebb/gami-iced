use chrono::NaiveDateTime;
use gami_sdk::{GameData, GameInstallStatus, IsGameLibraryRef};
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, DeriveEntityModel, EnumIter};
use std::time::{Duration, SystemTime};
#[derive(EnumIter, DeriveActiveEnum, Copy, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum DbGameInstallStatus {
    Installed = 0,
    Installing = 1,
    InLibrary = 2,
    Queued = 3,
}
impl From<GameInstallStatus> for DbGameInstallStatus {
    fn from(value: GameInstallStatus) -> Self {
        match value {
            GameInstallStatus::Installed => Self::Installed,
            GameInstallStatus::Installing => Self::Installing,
            GameInstallStatus::InLibrary => Self::InLibrary,
            GameInstallStatus::Queued => Self::Queued,
        }
    }
}
impl Into<GameInstallStatus> for DbGameInstallStatus {
    fn into(self) -> GameInstallStatus {
        match self {
            Self::Installed => GameInstallStatus::Installed,
            Self::Installing => GameInstallStatus::Installing,
            Self::InLibrary => GameInstallStatus::InLibrary,
            Self::Queued => GameInstallStatus::Queued,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub play_time_secs: i64,
    pub install_status: DbGameInstallStatus,
    pub release_date: Option<NaiveDateTime>,
    pub last_played: Option<NaiveDateTime>,
    pub icon_url: Option<String>,
    pub header_url: Option<String>,
    pub logo_url: Option<String>,
    pub hero_url: Option<String>,
    pub library_type: String,
    pub library_id: String,
}
fn naive_to_system(naive: NaiveDateTime) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(naive.and_utc().timestamp() as u64)
}
impl Into<GameData> for Model {
    fn into(self) -> GameData {
        GameData {
            id: self.id,
            name: self.name,
            library_type: self.library_type,
            library_id: self.library_id,
            install_status: self.install_status.into(),
            header_url: self.header_url,
            description: self.description,
            hero_url: self.hero_url,
            icon_url: self.icon_url,
            last_played: self.last_played.map(naive_to_system),
            logo_url: self.logo_url,
            release_date: self.release_date.map(naive_to_system),
            play_time: Duration::from_secs(self.play_time_secs as u64),
        }
    }
}
impl Default for Game {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            description: String::new(),
            play_time_secs: 0,
            install_status: DbGameInstallStatus::InLibrary,
            last_played: None,
            release_date: None,
            icon_url: None,
            header_url: None,
            logo_url: None,
            hero_url: None,
            library_type: String::new(),
            library_id: String::new(),
        }
    }
}

pub type Game = Model;
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::game_genres::Entity")]
    GameGenres,
}

impl Related<super::game_genres::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameGenres.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}

impl IsGameLibraryRef for Game {
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

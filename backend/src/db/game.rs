use chrono::{DateTime, Duration, NaiveDate, Utc};
use gami_sdk::{CompletionStatus, GameData, GameInstallStatus, IsGameLibraryRef};
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, DeriveEntityModel, EnumIter};
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

#[derive(EnumIter, DeriveActiveEnum, Copy, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum DbGameCompletionStatus {
    Backlog = 0,
    Playing = 1,
    Played = 2,
    OnHold = 3,
}
impl From<CompletionStatus> for DbGameCompletionStatus {
    fn from(value: CompletionStatus) -> Self {
        match value {
            CompletionStatus::Backlog => Self::Backlog,
            CompletionStatus::Playing => Self::Playing,
            CompletionStatus::Played => Self::Played,
            CompletionStatus::OnHold => Self::OnHold,
        }
    }
}
impl Into<CompletionStatus> for DbGameCompletionStatus {
    fn into(self) -> CompletionStatus {
        match self {
            Self::Backlog => CompletionStatus::Backlog,
            Self::Playing => CompletionStatus::Playing,
            Self::Played => CompletionStatus::Played,
            Self::OnHold => CompletionStatus::OnHold,
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
    pub release_date: Option<NaiveDate>,
    pub last_played: Option<DateTime<Utc>>,
    pub icon_url: Option<String>,
    pub header_url: Option<String>,
    pub cover_url: Option<String>,
    pub library_type: String,
    pub library_id: String,
    pub completion_status: DbGameCompletionStatus,
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
            cover_url: self.cover_url,
            icon_url: self.icon_url,
            last_played: self.last_played,
            release_date: self.release_date,
            play_time: Duration::seconds(self.play_time_secs),
            completion_status: self.completion_status.into(),
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
            cover_url: None,
            library_type: String::new(),
            library_id: String::new(),
            completion_status: CompletionStatus::Backlog.into(),
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

use rquickjs::class::{Trace, Tracer};
use rquickjs::{Ctx, FromJs, IntoJs, JsLifetime, Value};
use std::fmt;
use std::time::{Duration, SystemTime};

pub trait IsGameLibraryRef {
    fn get_name(&self) -> &str;
    fn get_library_type(&self) -> &str;
    fn get_library_id(&self) -> &str;
}
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum GameInstallStatus {
    Installed,
    Installing,
    InLibrary,
    Queued,
    Uninstalling,
}

impl<'js> IntoJs<'js> for GameInstallStatus {
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        rquickjs::String::from_str(
            ctx.clone(),
            match self {
                Self::Installed => "Installed",
                Self::Installing => "Installing",
                Self::InLibrary => "InLibrary",
                Self::Queued => "Queued",
                Self::Uninstalling => "Uninstalling",
            },
        )
        .map(Value::from_string)
    }
}
impl<'a> FromJs<'a> for GameInstallStatus {
    fn from_js(_ctx: &Ctx<'a>, value: Value<'a>) -> rquickjs::Result<Self> {
        let v = value
            .as_string()
            .expect("GameInstallStatus::from_js expected a string");
        Ok(match v.to_string()?.as_str() {
            "Installed" => Self::Installed,
            "Installing" => Self::Installing,
            "InLibrary" => Self::InLibrary,
            "Queued" => Self::Queued,
            "Uninstalling" => Self::Uninstalling,
            _ => {
                unimplemented!()
            }
        })
    }
}
impl Default for GameInstallStatus {
    fn default() -> Self {
        GameInstallStatus::InLibrary
    }
}
#[rquickjs::class(rename_all = "camelCase")]
#[derive(Default, Debug, Clone, Eq, PartialEq, JsLifetime)]
pub struct ScannedGameLibraryMetadata {
    pub name: String,
    pub library_type: String,
    pub library_id: String,
    pub last_played: Option<SystemTime>,
    pub install_status: GameInstallStatus,
    pub playtime: Duration,
    pub icon_url: Option<String>,
}
impl<'js> Trace<'js> for ScannedGameLibraryMetadata {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
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

#[rquickjs::class(rename_all = "camelCase")]
#[derive(Clone, Debug, PartialEq, Eq, Default, JsLifetime)]
pub struct GameData {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub play_time_secs: i64,
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

impl<'js> Trace<'js> for GameData {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::class(rename_all = "camelCase")]
#[derive(Clone, Debug, JsLifetime, Trace)]
pub struct GameLibrary {
    pub name: String,
    pub id: String,
}

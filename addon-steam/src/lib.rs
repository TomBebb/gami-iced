mod conf;
mod curl;
mod kv;
mod local_scanner;
mod models;
mod store;
mod store_models;

use crate::conf::Config;
use crate::curl::Collector;
use crate::store::StoreMetadataScanner;
use ::curl::easy::Easy2;
use gami_sdk::{
    register_plugin, ConfigSchemaKind, ConfigSchemaMetadata, GameLibrary, PluginRegistrar,
};
use gami_sdk::{GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use log::*;
pub use models::*;
use once_cell::sync::Lazy;
use safer_ffi::option::TaggedOption;
use std::collections::{BTreeMap, HashMap};
use std::ffi::{OsStr, OsString};
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::runtime::{self, Runtime};
use tokio::sync::Mutex;
use tokio::task;
use url::Url;

#[derive(Default)]
pub struct SteamLibrary {
    user_id: Mutex<Option<String>>,
}

const RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap()
});

fn wrap_in_quotes<T: AsRef<OsStr>>(path: T) -> OsString {
    let mut result = OsString::from("\"");
    result.push(path);
    result.push("\"");

    result
}
#[cfg(target_os = "windows")]
fn map_open_url_command(url: &str) -> Command {
    use std::os::windows::process::CommandExt;
    let mut cmd = Command::new("cmd");
    cmd.arg("/c")
        .arg("start")
        .raw_arg("\"\"")
        .raw_arg(wrap_in_quotes(url))
        .creation_flags(CREATE_NO_WINDOW);
    cmd
}
#[cfg(target_os = "linux")]
fn map_open_url_command(url: &str) -> Command {
    let mut cmd = Command::new("xdg-open");
    cmd.arg(url);
    cmd
}
#[cfg(target_os = "macos")]
fn map_open_url_command(url: &str) -> Command {
    let mut cmd = Command::new("/usr/bin/open");
    cmd.arg(url);
    cmd
}
const CREATE_NO_WINDOW: u32 = 0x08000000;
fn run_cmd(cmd: &'static str, id: &str) {
    let raw = format!("steam://{}/{}", cmd, id);
    let mut cmd = map_open_url_command(&raw);
    debug!("run : {:?} raw={}", cmd, raw);
    cmd.spawn().unwrap().wait().unwrap();
}
fn run_cmd_ref(cmd: &'static str, my_ref: GameLibraryRef) {
    run_cmd(cmd, &my_ref.library_id)
}
fn from_epoch(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}

pub fn auto_cache_map(id: &str, postfix: &str) -> TaggedOption<safer_ffi::string::String> {
    let full = crate::local_scanner::LIB_CACHE_PATH.join(format!("{}{}", id, postfix));
    if full.exists() {
        let url = Url::from_file_path(full).unwrap();
        TaggedOption::Some(url.to_string().into())
    } else {
        TaggedOption::None
    }
}

const ID: &str = "steam";
impl SteamLibrary {
    async fn auto_get_id(&self) -> String {
        let my_id = self.user_id.lock().await.clone();

        if let Some(id) = my_id {
            id
        } else {
            let id = local_scanner::get_steam_id().await;

            *self.user_id.lock().await = Some(id.clone());
            id
        }
    }
    async fn get_owned_games(&self, conf: Config) -> models::OwnedGamesResponse {
        let steam_id = self.auto_get_id().await;
        task::spawn_blocking(move || {
            let mut url =
                Url::parse("https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001")
                    .unwrap();
            url.query_pairs_mut()
                .append_pair("key", conf.api_key.as_str())
                .append_pair("steamid", steam_id.as_str())
                .append_pair("include_appinfo", "1")
                .append_pair("format", "json");

            let mut req = Easy2::new(Collector::default());
            req.url(url.as_str()).unwrap();
            req.perform().unwrap();
            let text: &str = std::str::from_utf8(req.get_ref().as_ref()).unwrap();
            serde_json::from_str(text).unwrap()
        })
        .await
        .unwrap()
    }
}
impl GameLibrary for SteamLibrary {
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata> {
        RUNTIME.block_on(async move {
            let conf = Config::load().await;
            let local_games = local_scanner::scan_local_dir_auto().await;
            if conf.api_key.is_empty() || (self.auto_get_id().await).is_empty() {
                return local_games;
            }
            let local_by_id: BTreeMap<String, ScannedGameLibraryMetadata> = BTreeMap::from_iter(
                local_games
                    .into_iter()
                    .map(|g| (g.library_id.to_string(), g)),
            );
            self.get_owned_games(conf)
                .await
                .response
                .games
                .into_iter()
                .map(|g: OwnedGame| {
                    let id_str = g.appid.to_string();
                    ScannedGameLibraryMetadata {
                        library_type: ID.into(),
                        library_id: id_str.clone().into(),
                        name: g.name.into(),
                        icon_url: auto_cache_map(&id_str, "_icon.jpg").into(),
                        last_played_epoch: TaggedOption::Some(g.rtime_last_played),
                        playtime_secs: g.playtime_forever,
                        install_status: local_by_id
                            .get(&g.appid.to_string())
                            .map(|v| v.install_status)
                            .unwrap_or(GameInstallStatus::InLibrary),
                    }
                })
                .collect()
        })
    }
    fn launch(&self, game: GameLibraryRef) {
        run_cmd_ref("rungameid", game)
    }
    fn install(&self, game: GameLibraryRef) {
        run_cmd_ref("install", game)
    }
    fn uninstall(&self, game: GameLibraryRef) {
        run_cmd_ref("uninstall", game)
    }
    fn check_install_status(&self, _game: GameLibraryRef) -> GameInstallStatus {
        GameInstallStatus::Installing
    }
}
register_plugin!(register, ID, "Steam");
#[no_mangle]
extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_library("steam", Arc::new(SteamLibrary::default()));
    registrar.register_metadata_scanner("steam", Arc::new(StoreMetadataScanner));

    let mut conf: HashMap<String, ConfigSchemaMetadata> = HashMap::with_capacity(2);
    conf.insert(
        "steamId".into(),
        ConfigSchemaMetadata {
            name: "Steam ID".into(),
            hint: "TODo".into(),
            kind: ConfigSchemaKind::Int,
        },
    );
    conf.insert(
        "apiKey".into(),
        ConfigSchemaMetadata {
            name: "API Key".into(),
            hint: "TODo".into(),
            kind: ConfigSchemaKind::Int,
        },
    );
    registrar.register_config("steam", conf);
}

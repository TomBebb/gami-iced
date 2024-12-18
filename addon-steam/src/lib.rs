mod conf;
mod kv;
mod local_scanner;
mod models;

use crate::conf::Config;
use gami_sdk::{
    register_plugin, ConfigSchemaKind, ConfigSchemaMetadata, GameLibrary, PluginRegistrar,
};
use gami_sdk::{GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use log::*;
pub use models::*;
use once_cell::sync::Lazy;
use safer_ffi::option::TaggedOption;
use std::collections::{BTreeMap, HashMap};
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::runtime::{self, Runtime};
use url::Url;

pub struct SteamLibrary;

const RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .unwrap()
});
fn run_cmd(cmd: &'static str, id: &str) {
    let raw = format!("steam://{}/{}", cmd, id);
    let mut cmd = Command::new(if cfg!(target_os = "linux") {
        "xdg-open"
    } else {
        "open"
    });
    cmd.arg(&raw);
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
    async fn get_owned_games(&self, conf: &Config) -> models::OwnedGamesResponse {
        let mut url =
            Url::parse("https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/").unwrap();
        url.query_pairs_mut()
            .append_pair("key", conf.api_key.as_str())
            .append_pair("steamid", conf.steam_id.as_str())
            .append_pair("include_appinfo", "1")
            .append_pair("format", "json");
        reqwest::get(url)
            .await
            .unwrap()
            .json::<models::OwnedGamesResponse>()
            .await
            .unwrap()
    }
}
impl GameLibrary for SteamLibrary {
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata> {
        RUNTIME.block_on(async move {
            let conf = Config::load().await;
            let local_games = local_scanner::scan_local_dir_auto().await;
            if conf.api_key.is_empty() || conf.steam_id.is_empty() {
                return local_games;
            }
            let local_by_id: BTreeMap<String, ScannedGameLibraryMetadata> = BTreeMap::from_iter(
                local_games
                    .into_iter()
                    .map(|g| (g.library_id.to_string(), g)),
            );
            self.get_owned_games(&conf)
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
    registrar.register_library("steam", Arc::new(SteamLibrary {}));

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

mod conf;
mod kv;
mod models;

use crate::conf::Config;
use crate::kv::ast::KvValue;
use crate::kv::parser::full_parse;
use gami_sdk::GameInstallStatus::Queued;
use gami_sdk::{register_plugin, BaseAddon, GameLibrary, PluginRegistrar};
use gami_sdk::{GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use log::*;
use once_cell::sync::Lazy;
use safer_ffi::option::TaggedOption;
use safer_ffi::string::str_ref;
use std::env;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use tokio::{fs, runtime};
use url::Url;

pub struct SteamLibrary;

const BASE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(windows) {
        "C:/Program Files (x86)/Steam".into()
    } else {
        let home = PathBuf::from(env::var("HOME").expect("HOME not found"));
        home.join(if cfg!(target_os = "linux") {
            ".local/share/Steam/"
        } else if cfg!(target_os = "macos") {
            "Library/Application Support/Steam"
        } else {
            unimplemented!()
        })
    }
});
const APPS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("steamapps"));
const LIB_CACHE_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("appcache/librarycache"));
fn auto_cache_map(id: &str, postfix: &str) -> TaggedOption<safer_ffi::string::String> {
    let full = LIB_CACHE_PATH.join(format!("{}{}", id, postfix));
    if full.exists() {
        let url = Url::from_file_path(full).unwrap();
        TaggedOption::Some(url.to_string().into())
    } else {
        TaggedOption::None
    }
}
const RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .unwrap()
});
fn run_cmd(cmd: &'static str, id: &str) {
    let raw = format!("steam://{}//{}", cmd, id);
    debug!("steam cmd: {}", raw);
    open::that_in_background(&raw);
}
fn run_cmd_ref(cmd: &'static str, my_ref: &GameLibraryRef) {
    run_cmd(cmd, &my_ref.library_id)
}
fn from_epoch(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}
const ID: &str = "steam";
impl BaseAddon for SteamLibrary {
    fn get_id(&self) -> str_ref<'static> {
        ID.into()
    }
}
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
    fn launch(&self, game: &GameLibraryRef) {
        run_cmd_ref("launch", game)
    }
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata> {
        RUNTIME.block_on(async move {
            let conf = Config::load().await;
            let owned_games = self.get_owned_games(&conf).await;
            println!("Owned games: {:?}", owned_games);
            let mut res = Vec::with_capacity(8);
            let mut reader = fs::read_dir(APPS_PATH.as_path()).await.unwrap();
            while let Some(entry) = reader.next_entry().await.unwrap() {
                let path: PathBuf = entry.path();
                debug!("Checking file: {:?}", path);
                let name = path.file_name().unwrap().to_str().unwrap();
                if !name.starts_with("appmanifest") {
                    continue;
                }
                debug!("Reading file: {:?}", path);
                let reader = fs::File::open(&path).await.unwrap();
                debug!("Parsing file: {:?}", path);
                let parsed = full_parse(reader).await.unwrap();
                debug!("Parsed file: {:?}", parsed);
                let obj = if let KvValue::Object(v) = parsed.value {
                    v
                } else {
                    error!("Steam KSV: Expected an object");
                    continue;
                };

                let get_obj_text = |key: &str| match obj[key] {
                    KvValue::String(ref s) => s.as_str(),
                    _ => panic!("Steam KSV: Expected an string at key: {}", key),
                };
                let get_obj_text_opt = |key: &str| match obj.get(key) {
                    Some(KvValue::String(ref s)) => Some(s.as_str()),
                    Some(_) => panic!("Steam KSV: Expected an string at key: {}", key),
                    None => None,
                };

                let get_obj_unix_opt = |key: &str| {
                    if let Some(raw) = get_obj_text_opt(key) {
                        let parsed: u64 = raw.parse().unwrap();
                        Some(from_epoch(parsed))
                    } else {
                        None
                    }
                };
                let bytes_to_dl = get_obj_text_opt("BytesToDownload");
                let bytes_dl = get_obj_text_opt("BytesDownloaded");
                let app_id = get_obj_text("appid");
                res.push(ScannedGameLibraryMetadata {
                    library_id: app_id.into(),
                    name: get_obj_text("name").into(),
                    icon_url: auto_cache_map(app_id, "_icon.jpg").into(),
                    last_played_epoch: get_obj_unix_opt("LastPlayed")
                        .map(|time| time.duration_since(UNIX_EPOCH).unwrap().as_secs())
                        .into(),
                    library_type: ID.into(),
                    install_status: if bytes_dl == None {
                        Queued
                    } else if bytes_dl == bytes_to_dl {
                        GameInstallStatus::Installed
                    } else {
                        GameInstallStatus::Installing
                    },
                    ..Default::default()
                })
            }
            res
        })
    }
    fn install(&self, game: &GameLibraryRef) {
        run_cmd_ref("install", game)
    }
    fn uninstall(&self, game: &GameLibraryRef) {
        run_cmd_ref("uninstall", game)
    }
    fn check_install_status(&self, _game: &GameLibraryRef) -> GameInstallStatus {
        GameInstallStatus::Installing
    }
}
// random/src/lib.rs

register_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_library("steam", Box::new(SteamLibrary {}));
}

mod kv;

use crate::kv::ast::KvValue;
use crate::kv::parser::full_parse;
use gami_sdk::GameInstallStatus::Queued;
use gami_sdk::{register_plugin, BaseAddon, BoxFuture, GameLibrary, PluginRegistrar};
use gami_sdk::{GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use log::*;
use once_cell::sync::Lazy;
use safer_ffi::string::str_ref;
use std::env;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs;

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
fn run_cmd(cmd: &'static str, id: &str) {
    let raw = format!("steam://{}//{}", cmd, id);
    debug!("steam cmd: {}", raw);
    open::that_in_background(&raw);
}
fn run_cmd_ref(cmd: &'static str, my_ref: GameLibraryRef) {
    run_cmd(cmd, &my_ref.library_id)
}
fn wrapped_run_cmd(cmd: &'static str, game: &GameLibraryRef) -> BoxFuture<'static> {
    let game_c = game.clone();
    Box::pin(async move {
        run_cmd_ref(cmd, game_c);
    })
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
impl GameLibrary for SteamLibrary {
    fn scan(&self) -> BoxFuture<'static, Vec<ScannedGameLibraryMetadata>> {
        debug!("APPS_PATH: {:?}", &*APPS_PATH);
        let mut res = Vec::with_capacity(8);
        Box::pin(async move {
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
                res.push(ScannedGameLibraryMetadata {
                    library_id: get_obj_text("appid").into(),
                    name: get_obj_text("name").into(),
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
    fn launch(&self, game: &GameLibraryRef) -> BoxFuture<'static> {
        wrapped_run_cmd("launch", game)
    }
    fn install(&self, game: &GameLibraryRef) -> BoxFuture<'static> {
        wrapped_run_cmd("install", game)
    }
    fn uninstall(&self, game: &GameLibraryRef) -> BoxFuture<'static> {
        wrapped_run_cmd("uninstall", game)
    }
    fn check_install_status(&self, game: &GameLibraryRef) -> BoxFuture<'static, GameInstallStatus> {
        // TODO
        Box::pin(async { GameInstallStatus::Installing })
    }
}
// random/src/lib.rs

register_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_library("steam", Box::new(SteamLibrary {}));
}

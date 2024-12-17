use crate::kv::ast::KvValue;
use crate::kv::parser::full_parse;
use crate::{auto_cache_map, from_epoch};
use gami_sdk::GameInstallStatus::Queued;
use gami_sdk::{GameInstallStatus, ScannedGameLibraryMetadata};
use log::{debug, error};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::{env, io};
use tokio::fs;

const BASE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(windows) {
        "C:/Program Files (x86)/Steam".into()
    } else {
        let home = PathBuf::from(env::var("HOME").expect("HOME not found"));

        if cfg!(target_os = "linux") {
            let non_debian = home.join(".steam/steam");
            if non_debian.exists() {
                non_debian
            } else {
                home.join(".steam/debian-installation")
            }
        } else if cfg!(target_os = "macos") {
            home.join("Library/Application Support/Steam")
        } else {
            unimplemented!()
        }
    }
});
const APPS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("steamapps"));
const USERS_CONF_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_PATH.join("config/loginusers.vdf"));
pub(crate) const LIB_CACHE_PATH: Lazy<PathBuf> =
    Lazy::new(|| BASE_PATH.join("appcache/librarycache"));

pub async fn scan_local_dir_auto() -> Vec<ScannedGameLibraryMetadata> {
    println!("Scanning local folders: {}", APPS_PATH.display());
    let mut reader = fs::read_dir(APPS_PATH.as_path()).await.unwrap();

    let mut items = Vec::with_capacity(8);
    while let Some(entry) = reader.next_entry().await.unwrap() {
        let path: PathBuf = entry.path();
        debug!("Checking file: {:?}", path);
        let name = path.file_name().unwrap().to_str().unwrap();
        if !name.starts_with("appmanifest") {
            continue;
        }
        items.push(scan_local(&path).await.unwrap());
    }
    items
}
pub async fn scan_local(path: &Path) -> io::Result<ScannedGameLibraryMetadata> {
    let reader = fs::File::open(&path).await?;
    debug!("Parsing file: {:?}", path);
    let parsed = full_parse(reader).await.unwrap();
    debug!("Parsed file: {:?}", parsed);
    let obj = if let KvValue::Object(v) = parsed.value {
        v
    } else {
        error!("Steam KSV: Expected an object");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected an object",
        ));
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
    Ok(ScannedGameLibraryMetadata {
        library_id: app_id.into(),
        name: get_obj_text("name").into(),
        icon_url: auto_cache_map(app_id, "_icon.jpg").into(),
        last_played_epoch: get_obj_unix_opt("LastPlayed")
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap().as_secs())
            .into(),
        library_type: "steam".into(),
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

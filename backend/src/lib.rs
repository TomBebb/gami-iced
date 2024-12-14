use crate::plugins::ExternalAddons;
use std::cell::LazyCell;
use std::path::PathBuf;

pub mod db;
pub mod models;
mod modules;
pub mod plugins;

pub const BASE_DATA_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    dirs::data_dir()
        .expect("No data directory set!")
        .join("gami")
});
pub const ADDONS: LazyCell<ExternalAddons> = LazyCell::new(|| unsafe {
    let mut addons = ExternalAddons::new();
    addons.auto_load_addons().unwrap();
    addons
});

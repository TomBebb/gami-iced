use crate::plugins::PluginsRuntime;
use rquickjs::Runtime;
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
pub const RUNTIME: LazyCell<Runtime> = LazyCell::new(|| Runtime::new().unwrap());
pub const PLUGINS: LazyCell<PluginsRuntime> = LazyCell::new(|| PluginsRuntime::new(&*RUNTIME));
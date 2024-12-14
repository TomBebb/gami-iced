use crate::modules::{js_fs, js_sdk};
use crate::plugins::PluginsRuntime;
use rquickjs::loader::{BuiltinResolver, FileResolver, ModuleLoader, ScriptLoader};
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
pub const RUNTIME: LazyCell<Runtime> = LazyCell::new(|| {
    let plugin_dir = BASE_DATA_DIR.join("plugins");
    let resolver = (
        BuiltinResolver::default().with_module(plugin_dir.to_string_lossy()),
        FileResolver::default().with_path("./").with_native(),
    );
    let loader = (
        ModuleLoader::default()
        .with_module("@gami/sdk", js_sdk)
        .with_module("fs/promises", js_fs),
        ScriptLoader::default(),
    );
    let rt = Runtime::new().unwrap();
    rt.set_loader(resolver, loader);
    rt
});
pub const PLUGINS: LazyCell<PluginsRuntime> = LazyCell::new(|| PluginsRuntime::new(&*RUNTIME));

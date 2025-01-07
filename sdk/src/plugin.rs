use crate::{
    models::ConfigSchemaMetadata, BoxFuture, GameInstallStatus, GameLibraryRef, GameLibraryRefOwned,
    GameMetadata, ScannedGameLibraryMetadata, BASE_DATA_DIR,
};
use safer_ffi::string::String;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

#[derive(Clone, Debug)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
}

pub const ADDONS_DIR: LazyCell<PathBuf> = LazyCell::new(|| BASE_DATA_DIR.join("addons"));
pub type ConfigsSchema = HashMap<std::string::String, ConfigSchemaMetadata>;

pub fn load_schema(id: &str) -> ConfigsSchema {
    let path = ADDONS_DIR.join(format!("{}/schema.json", id));
    if !path.exists() {
        return HashMap::new();
    }
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}
pub trait PluginRegistrar {
    fn register_config(
        &mut self,
        file_name: &str,
        schema: HashMap<std::string::String, ConfigSchemaMetadata>,
    );
    fn register_library(&mut self, name: &str, function: Arc<dyn GameLibrary + Send + Sync>);
    fn register_metadata_scanner(
        &mut self,
        name: &str,
        function: Arc<dyn GameMetadataScanner + Send + Sync>,
    );
}

pub trait GameMetadataScanner: Send {
    fn get_metadata(&self, game: GameLibraryRef) -> Option<GameMetadata>;
    fn get_metadatas<'a>(
        &self,
        games: &[GameLibraryRef<'a>],
        on_process_one: Box<dyn Fn() -> BoxFuture<'a, ()>>,
    ) -> HashMap<GameLibraryRefOwned, GameMetadata>;
}
pub trait GameLibrary: Send {
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata>;
    fn launch(&self, game: GameLibraryRef);
    fn install(&self, game: GameLibraryRef);
    fn uninstall(&self, game: GameLibraryRef);
    fn check_install_status(&self, game: GameLibraryRef) -> GameInstallStatus;
}
pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

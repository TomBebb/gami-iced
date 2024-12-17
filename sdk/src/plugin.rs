use crate::{
    models::ConfigSchemaMetadata, GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata,
    BASE_DATA_DIR, BASE__DIR,
};
use safer_ffi::string::String;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::path::PathBuf;

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
    fn register_library(&mut self, name: &str, function: Box<dyn GameLibrary>);
}

pub trait GameLibrary: Send {
    fn launch(&self, game: &GameLibraryRef);
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata>;
    fn install(&self, game: &GameLibraryRef);
    fn uninstall(&self, game: &GameLibraryRef);
    fn check_install_status(&self, game: &GameLibraryRef) -> GameInstallStatus;
}
pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

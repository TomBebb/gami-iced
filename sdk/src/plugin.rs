use crate::{GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use safer_ffi::string::str_ref;
use std::collections::HashMap;

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}
#[derive(Debug, Clone, Copy)]
pub struct ConfigSchemaMetadata {
    pub key: &'static str,
    pub hint: &'static str,
    pub name: &'static str,
    pub kind: ConfigSchemaKind,
}
#[derive(Debug, Copy, Clone)]
pub enum ConfigSchemaKind {
    String,
    Int,
    Boolean,
}
pub trait PluginRegistrar {
    fn register_config(&mut self, file_name: &str, schema: HashMap<String, ConfigSchemaMetadata>);
    fn register_library(&mut self, name: &str, function: Box<dyn GameLibrary>);
}

pub trait BaseAddon {
    fn get_id(&self) -> str_ref<'static>;
}

pub trait GameLibrary: BaseAddon + Send {
    fn launch(&self, game: &GameLibraryRef);
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata>;
    fn install(&self, game: &GameLibraryRef);
    fn uninstall(&self, game: &GameLibraryRef);
    fn check_install_status(&self, game: &GameLibraryRef) -> GameInstallStatus;
}

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

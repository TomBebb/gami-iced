use crate::{BoxFuture, GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use safer_ffi::string::str_ref;

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register_library(&mut self, name: &str, function: Box<dyn GameLibrary>);
}

pub trait BaseAddon {
    fn get_id(&self) -> str_ref<'static>;
    //   const TYPE: &'static str;
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

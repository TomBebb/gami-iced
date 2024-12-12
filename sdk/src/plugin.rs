use crate::{BoxFuture, GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register_library(&mut self, name: &str, function: Box<dyn GameLibrary>);
}

pub trait BaseAddon {
    fn get_id(&self) -> &'static str;
    //   const TYPE: &'static str;
}

pub trait GameLibrary: BaseAddon + Send {
    fn launch(&self, game: &GameLibraryRef) -> BoxFuture<'static>;
    fn scan(&self) -> BoxFuture<'static, Vec<ScannedGameLibraryMetadata>>;
    fn install(&self, game: &GameLibraryRef) -> BoxFuture<'static>;
    fn uninstall(&self, game: &GameLibraryRef) -> BoxFuture<'static>;
    fn check_install_status(&self, game: &GameLibraryRef) -> BoxFuture<'static, GameInstallStatus>;
}

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

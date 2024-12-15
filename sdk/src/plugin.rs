use crate::{BoxFuture, GameInstallStatus, GameLibraryRef, ScannedGameLibraryMetadata};
use ::safer_ffi::prelude::*;
use safer_ffi::derive_ReprC;
use safer_ffi::string::str_ref;

#[derive_ReprC(dyn)]
pub trait BaseAddon {
    fn get_id(&self) -> str_ref<'static>;
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

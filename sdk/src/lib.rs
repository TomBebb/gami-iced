use std::cell::LazyCell;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio_stream::Stream;

mod models;
mod plugin;

pub use plugin::*;

pub use models::*;

pub const BASE_DATA_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    dirs::data_dir()
        .expect("No data directory set!")
        .join("gami")
});
pub fn resolve_addon_config_json_path(key: &str) -> PathBuf {
    let parent = ADDONS_DIR.join(key);
    std::fs::create_dir_all(&parent).unwrap();
    parent.join("config.json")
}
pub const BASE__DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    dirs::data_dir()
        .expect("No data directory set!")
        .join("gami")
});
pub type BoxFuture<'a, T = ()> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;
#[macro_export]
macro_rules! register_plugin {
    ($register:expr, $id:expr, $name:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        unsafe extern "C" fn get_metadata() -> $crate::PluginMetadata {
            $crate::PluginMetadata {
                id: $id.into(),
                name: $name.into(),
            }
        }
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}

pub trait GameCommon {
    fn get_ref(&self) -> GameLibraryRef;
    fn get_owned_ref(&self) -> GameLibraryRefOwned {
        self.get_ref().to_owned()
    }
}

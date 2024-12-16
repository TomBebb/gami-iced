use gami_sdk::{
    BaseAddon, BoxFuture, BoxStream, ConfigSchemaMetadata, GameInstallStatus, GameLibrary,
    GameLibraryRef, PluginDeclaration, ScannedGameLibraryMetadata, BASE_DATA_DIR,
};
use libloading::Library;
use safer_ffi::string::str_ref;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
pub struct GameLibraryProxy {
    pub inner: Box<dyn GameLibrary>,
    pub _lib: Arc<Library>,
}
impl BaseAddon for GameLibraryProxy {
    fn get_id(&self) -> str_ref<'static> {
        self.inner.get_id()
    }
}
impl GameLibrary for GameLibraryProxy {
    fn launch(&self, game: &GameLibraryRef) {
        self.inner.launch(game)
    }

    fn scan(&self) -> Vec<ScannedGameLibraryMetadata> {
        self.inner.scan()
    }

    fn install(&self, game: &GameLibraryRef) {
        self.inner.install(game)
    }

    fn uninstall(&self, game: &GameLibraryRef) {
        self.inner.uninstall(game)
    }

    fn check_install_status(&self, game: &GameLibraryRef) -> GameInstallStatus {
        self.inner.check_install_status(game)
    }
}

#[derive(Default)]
pub struct ExternalAddons {
    game_libs: HashMap<String, GameLibraryProxy>,
    libraries: Vec<Arc<Library>>,
}

impl ExternalAddons {
    pub fn new() -> ExternalAddons {
        ExternalAddons::default()
    }

    pub fn get_keys(&self) -> Vec<&str> {
        self.game_libs.keys().map(String::as_str).collect()
    }
    pub fn get_game_library(&self, name: &str) -> Option<&GameLibraryProxy> {
        self.game_libs.get(name)
    }

    pub unsafe fn auto_load_addons(&mut self) -> io::Result<()> {
        log::info!("Automatically loading addons");
        for res in std::fs::read_dir(&*ADDONS_DIR)? {
            let path = res?.path();
            println!("Loading {}", path.display());
            self.load(&path)?;
            println!("Loaded {}", path.display());
        }
        Ok(())
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        // load the library into memory
        let library = Arc::new(Library::new(library_path).unwrap());

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")
            .unwrap()
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.rustc_version != gami_sdk::RUSTC_VERSION
            || decl.core_version != gami_sdk::CORE_VERSION
        {
            return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
        }

        let mut registrar = PluginRegistrar::new(Arc::clone(&library));

        (decl.register)(&mut registrar);

        // add all loaded plugins to the functions map
        self.game_libs.extend(registrar.game_libs);
        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);

        Ok(())
    }
}
struct PluginRegistrar {
    game_libs: HashMap<String, GameLibraryProxy>,
    lib: Arc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Arc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            game_libs: HashMap::default(),
        }
    }
}

impl gami_sdk::PluginRegistrar for PluginRegistrar {
    fn register_config(&mut self, file_name: &str, schema: HashMap<String, ConfigSchemaMetadata>) {}

    fn register_library(&mut self, name: &str, lib: Box<dyn GameLibrary>) {
        let proxy = GameLibraryProxy {
            inner: lib,
            _lib: Arc::clone(&self.lib),
        };
        self.game_libs.insert(name.to_string(), proxy);
    }
}
pub const ADDONS_DIR: LazyCell<PathBuf> = LazyCell::new(|| BASE_DATA_DIR.join("addons"));

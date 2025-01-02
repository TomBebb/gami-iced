use gami_sdk::{
    ConfigSchemaMetadata, GameInstallStatus, GameLibrary, GameLibraryRef, GameMetadataScanner,
    PluginDeclaration, PluginMetadata, ScannedGameLibraryMetadata, ADDONS_DIR,
};
use libloading::Library;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::Arc;
use std::{fs, io};

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
///
#[derive(Clone)]
pub struct GameLibraryProxy {
    pub inner: Arc<dyn GameLibrary + Send + Sync>,
    pub _lib: Arc<Library>,
}
#[derive(Clone)]
pub struct GameMetadataScannerProxy {
    pub inner: Arc<dyn GameMetadataScanner + Send + Sync>,
    pub _lib: Arc<Library>,
}
impl GameLibrary for GameLibraryProxy {
    fn scan(&self) -> Vec<ScannedGameLibraryMetadata> {
        self.inner.scan()
    }
    fn launch(&self, game: GameLibraryRef) {
        self.inner.launch(game)
    }

    fn install(&self, game: GameLibraryRef) {
        self.inner.install(game)
    }

    fn uninstall(&self, game: GameLibraryRef) {
        self.inner.uninstall(game)
    }

    fn check_install_status(&self, game: GameLibraryRef) -> GameInstallStatus {
        self.inner.check_install_status(game)
    }
}

#[derive(Default)]
pub struct ExternalAddons {
    game_libs: HashMap<String, GameLibraryProxy>,
    meta_scanners: HashMap<String, GameMetadataScannerProxy>,
    metas: Vec<PluginMetadata>,
    libraries: Vec<Arc<Library>>,
}

impl ExternalAddons {
    pub fn new() -> ExternalAddons {
        ExternalAddons::default()
    }

    pub fn get_addon_metadatas(&self) -> &[PluginMetadata] {
        &self.metas
    }

    pub fn get_keys(&self) -> Vec<&str> {
        self.game_libs.keys().map(String::as_str).collect()
    }
    pub fn get_game_library(&self, name: &str) -> Option<&GameLibraryProxy> {
        self.game_libs.get(name)
    }

    pub fn get_game_metadata(&self, name: &str) -> Option<&GameMetadataScannerProxy> {
        self.meta_scanners.get(name)
    }

    pub unsafe fn auto_load_addons(&mut self) -> io::Result<()> {
        log::info!("Automatically loading addons");

        fs::create_dir_all(&*ADDONS_DIR)?;
        for dir in fs::read_dir(&*ADDONS_DIR)? {
            for sub in fs::read_dir(dir?.path())? {
                let path = sub?.path();

                if path.extension().unwrap_or_default() == "json" {
                    continue;
                }

                println!("Loading {}", path.display());
                self.load(&path)?;
                println!("Loaded {}", path.display());
            }
        }
        log::info!(
            "loaded addons; library: {:?}; metadata: {:?}",
            self.game_libs.keys(),
            self.meta_scanners.keys()
        );
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
        let metadata = library
            .get::<unsafe extern "C" fn() -> PluginMetadata>(b"get_metadata\0")
            .unwrap()();

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
        println!("configs: {:?}", registrar.configs);

        // add all loaded plugins to the functions map
        self.game_libs.extend(registrar.game_libs);
        self.meta_scanners.extend(registrar.game_meta_scanners);
        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);
        self.metas.push(metadata);

        Ok(())
    }
}
struct PluginRegistrar {
    game_libs: HashMap<String, GameLibraryProxy>,
    game_meta_scanners: HashMap<String, GameMetadataScannerProxy>,
    configs: HashMap<String, HashMap<String, ConfigSchemaMetadata>>,
    lib: Arc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Arc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            configs: HashMap::default(),
            game_libs: HashMap::default(),
            game_meta_scanners: HashMap::default(),
        }
    }
}

impl gami_sdk::PluginRegistrar for PluginRegistrar {
    fn register_config(&mut self, file_name: &str, schema: HashMap<String, ConfigSchemaMetadata>) {
        self.configs.insert(file_name.to_string(), schema);
    }

    fn register_library(&mut self, name: &str, lib: Arc<dyn GameLibrary + Send + Sync>) {
        let proxy = GameLibraryProxy {
            inner: lib,
            _lib: Arc::clone(&self.lib),
        };
        self.game_libs.insert(name.to_string(), proxy);
    }
    fn register_metadata_scanner(
        &mut self,
        name: &str,
        lib: Arc<dyn GameMetadataScanner + Send + Sync>,
    ) {
        let proxy = GameMetadataScannerProxy {
            inner: lib,
            _lib: Arc::clone(&self.lib),
        };
        self.game_meta_scanners.insert(name.to_string(), proxy);
    }
}

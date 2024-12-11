use gami_sdk::{
    BaseAddon, BoxFuture, BoxStream, GameInstallStatus, GameLibrary, GameLibraryRef,
    PluginDeclaration, ScannedGameLibraryMetadata,
};
use libloading::Library;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::rc::Rc;

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
pub struct GameLibraryProxy {
    pub inner: Box<dyn GameLibrary>,
    pub _lib: Rc<Library>,
}
impl BaseAddon for GameLibraryProxy {
    fn get_id(&self) -> &'static str {
        self.inner.get_id()
    }
}
impl GameLibrary for GameLibraryProxy {
    fn launch(&self, game: &GameLibraryRef) -> BoxFuture<'static> {
        self.inner.launch(game)
    }

    fn scan(&self) -> BoxStream<'static, ScannedGameLibraryMetadata> {
        self.inner.scan()
    }

    fn install(&self, game: &GameLibraryRef) -> BoxFuture<'static> {
        self.inner.install(game)
    }

    fn uninstall(&self, game: &GameLibraryRef) -> BoxFuture<'static> {
        self.inner.uninstall(game)
    }

    fn check_install_status(&self, game: &GameLibraryRef) -> BoxFuture<'static, GameInstallStatus> {
        self.inner.check_install_status(game)
    }
}

#[derive(Default)]
pub struct ExternalAddons {
    game_libs: HashMap<String, GameLibraryProxy>,
    libraries: Vec<Rc<Library>>,
}

impl ExternalAddons {
    pub fn new() -> ExternalAddons {
        ExternalAddons::default()
    }

    pub fn get_keys(&self) -> Vec<&str> {
        self.game_libs.keys().map(String::as_str).collect()
    }
    pub fn get_game_library(&self, name: &str) -> Result<&GameLibraryProxy, String> {
        self.game_libs
            .get(name)
            .ok_or_else(|| format!("\"{}\" not found", name))
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
        let library = Rc::new(Library::new(library_path).unwrap());

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

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

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
    lib: Rc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            game_libs: HashMap::default(),
        }
    }
}

impl gami_sdk::PluginRegistrar for PluginRegistrar {
    fn register_library(&mut self, name: &str, lib: Box<dyn GameLibrary>) {
        let proxy = GameLibraryProxy {
            inner: lib,
            _lib: Rc::clone(&self.lib),
        };
        self.game_libs.insert(name.to_string(), proxy);
    }
}

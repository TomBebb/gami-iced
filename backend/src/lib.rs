use crate::plugin::ExternalAddons;
use std::cell::LazyCell;

pub mod db;
pub mod plugin;

pub const ADDONS: LazyCell<ExternalAddons> = LazyCell::new(|| unsafe {
    let mut addons = ExternalAddons::new();
    addons.auto_load_addons().unwrap();
    addons
});

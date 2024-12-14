use rquickjs::{Ctx, Module};

pub mod fs;

pub fn setup(ctx:Ctx) {

    Module::declare_def::<fs::js_fs, _>(ctx, "fs/promises").unwrap();
}
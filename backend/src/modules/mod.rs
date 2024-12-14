use rquickjs::{Ctx, Module};

mod fs;
mod sdk;

pub fn setup(ctx: Ctx) {
    Module::declare_def::<sdk::js_sdk, _>(ctx.clone(), "@gami/sdk").unwrap();
    Module::declare_def::<fs::js_fs, _>(ctx, "fs/promises").unwrap();
}

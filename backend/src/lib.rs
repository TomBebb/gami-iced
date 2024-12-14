use crate::plugins::PluginsRuntime;
use async_once_cell::Lazy;
use rquickjs::AsyncRuntime;
use std::cell::LazyCell;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

pub mod db;
pub mod models;
mod modules;
pub mod plugins;

pub const BASE_DATA_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    dirs::data_dir()
        .expect("No data directory set!")
        .join("gami")
});

pub const RUNTIME: LazyCell<AsyncRuntime> = LazyCell::new(|| AsyncRuntime::new().unwrap());
struct CF<F>(F)
where
    F: Future<Output = PluginsRuntime> + 'static;
impl<F> Future for CF<F>
where
    F: Future<Output = PluginsRuntime> + 'static,
{
    type Output = PluginsRuntime;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.poll(cx)
    }
}

static PLUGINS: Lazy<PluginsRuntime, _> = Lazy::new(CF(None));

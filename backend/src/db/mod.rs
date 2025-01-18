pub mod game;
pub mod game_genres;
pub mod genre;
pub mod ops;

pub use game::Game;
use gami_sdk::BASE_DATA_DIR;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use std::cell::LazyCell;
use std::fs;
use std::path::PathBuf;
const DB_FILE: LazyCell<PathBuf> = LazyCell::new(|| BASE_DATA_DIR.join("data.db"));
pub(crate) const DB_OPTIONS: LazyCell<ConnectOptions> = LazyCell::new(|| {
    let mut opt = ConnectOptions::new(format!("sqlite:{}?mode=rwc", DB_FILE.to_string_lossy()));
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    opt
});

async fn connect() -> DatabaseConnection {
    Database::connect(DB_OPTIONS.clone()).await.unwrap()
}
pub async fn init() {
    log::info!("initializing database");

    fs::create_dir_all(&*BASE_DATA_DIR).unwrap();
    let conn = connect().await;

    conn.execute_unprepared(include_str!("init.sql"))
        .await
        .unwrap();
    log::info!("initialized game database");
}

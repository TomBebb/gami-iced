use crate::{db, ADDONS};
use db::game::Entity as GameEntity;
use gami_sdk::GameData;
use gami_sdk::GameLibrary;
use sea_orm::EntityTrait;
pub async fn sync_library() {
    for key in ADDONS.get_keys() {
        if let Some(lib) = ADDONS.get_game_library(key) {
            for item in lib.scan().await {
                println!("got item: {:?}", item);
            }
        }
    }
}
pub async fn get_games() -> Vec<GameData> {
    println!("Getting games");
    let conn = crate::db::connect().await;
    let raw = GameEntity::find().all(&conn).await.unwrap();
    raw.into_iter().map(Into::into).collect()
}

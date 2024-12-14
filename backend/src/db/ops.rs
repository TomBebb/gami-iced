use crate::{db, PLUGINS};
use db::game::Entity as GameEntity;
use sea_orm::EntityTrait;
use crate::models::GameData;

pub async fn sync_library() {
    let my_plugin = &*PLUGINS;
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

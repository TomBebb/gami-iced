use crate::db;
use db::game::Entity as GameEntity;
use gami_sdk::GameData;
use sea_orm::EntityTrait;

pub async fn sync_library(code: &str) {}
pub async fn get_games() -> Vec<GameData> {
    println!("Getting games");
    let conn = crate::db::connect().await;
    let raw = GameEntity::find().all(&conn).await.unwrap();
    raw.into_iter().map(Into::into).collect()
}

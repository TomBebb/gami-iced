use crate::db::game::Column;
use crate::{db, ADDONS};
use db::game::{ActiveModel as GameModel, Entity as GameEntity};
use gami_sdk::GameLibrary;
use gami_sdk::{GameData, ScannedGameLibraryMetadata};
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;

pub async fn sync_library() {
    for key in ADDONS.get_keys() {
        if let Some(lib) = ADDONS.get_game_library(key) {
            let items = lib.scan();
            log::info!("Pushing {} games to DB", items.len());
            let mut conn = crate::db::connect().await;
            GameEntity::insert_many(items.into_iter().map(|i: ScannedGameLibraryMetadata| {
                GameModel {
                    name: Set(i.name.into()),
                    description: Set("".into()),
                    icon_url: Set(i.icon_url.into_rust().map(safer_ffi::string::String::into)),
                    install_status: Set(i.install_status.into()),
                    play_time_secs: Set(i.playtime_secs as i64),
                    //last_played:Set( i.last_played_epoch),
                    library_id: Set(i.library_id.into()),
                    library_type: Set(i.library_type.into()),
                    ..Default::default()
                }
            }))
            .on_conflict(OnConflict::columns([
                Column::LibraryType,
                Column::LibraryId,
            ]))
            .do_nothing()
            .exec(&mut conn)
            .await
            .unwrap();

            log::info!("Pushed games to DB");
        }
    }
}
pub async fn get_games() -> Vec<GameData> {
    println!("Getting games");
    let conn = crate::db::connect().await;
    let raw = GameEntity::find().all(&conn).await.unwrap();
    raw.into_iter().map(Into::into).collect()
}

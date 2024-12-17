use crate::db::game::Column;
use crate::{db, ADDONS};
use db::game::Entity as GameEntity;
use gami_sdk::GameData;
use gami_sdk::GameLibrary;
use sea_orm::sea_query::{OnConflict, Query, SqliteQueryBuilder};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub async fn delete_game(game_id: i32) {
    let mut conn = db::connect().await;

    GameEntity::delete_by_id(game_id)
        .exec(&mut conn)
        .await
        .unwrap();
}
pub async fn sync_library() {
    for key in ADDONS.get_keys() {
        if let Some(lib) = ADDONS.get_game_library(key) {
            let items = lib.scan();
            log::info!("Pushing {} games to DB", items.len());
            let conn = db::connect().await;
            let mut raw = Query::insert();
            let mut query_raw = raw
                .into_table(GameEntity)
                .columns(vec![
                    Column::LibraryType,
                    Column::LibraryId,
                    Column::Name,
                    Column::Description,
                    Column::InstallStatus,
                    Column::PlayTimeSecs,
                    Column::LastPlayed,
                    Column::IconUrl,
                ])
                .on_conflict(OnConflict::columns([
                    Column::LibraryType,
                    Column::LibraryId,
                ]));
            for item in items {
                query_raw = query_raw.values_panic(vec![
                    item.library_type.to_string().into(),
                    item.library_id.to_string().into(),
                    item.name.to_string().into(),
                    "".into(),
                    (item.install_status as u8).into(),
                    item.playtime_secs.into(),
                    item.last_played_epoch.into_rust().into(),
                    item.icon_url
                        .into_rust()
                        .map(<safer_ffi::String as Into<String>>::into)
                        .into(),
                ]);
            }
            let mut query = query_raw.to_string(SqliteQueryBuilder);
            if query.ends_with(')') {
                query.push_str(" DO NOTHING");
            }
            conn.execute_unprepared(&query).await.unwrap();
            log::info!("Pushed games to DB");
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GamesFilters {
    pub search: String,
}
pub async fn get_games(filters: GamesFilters) -> Vec<GameData> {
    let conn = db::connect().await;
    let mut query = GameEntity::find();
    if !filters.search.is_empty() {
        query = query.filter(Column::Name.contains(&filters.search));
    }
    let raw = query.all(&conn).await.unwrap();
    raw.into_iter().map(Into::into).collect()
}

use crate::db::game;
use crate::db::game::Column;
use crate::{db, ADDONS};
use db::game::Entity as GameEntity;
use gami_sdk::GameData;
use gami_sdk::GameLibrary;
use sea_orm::sea_query::{OnConflict, Query, SqliteQueryBuilder};
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder,
};
use std::fmt;

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
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}
impl Into<Order> for SortOrder {
    fn into(self) -> Order {
        match self {
            SortOrder::Ascending => Order::Asc,
            SortOrder::Descending => Order::Desc,
        }
    }
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum SortField {
    #[default]
    Name,
    LastPlayed,
    Playtime,
    ReleaseDate,
}
impl fmt::Display for SortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::LastPlayed => "Last played",
            Self::Playtime => "Playtime",
            Self::ReleaseDate => "Release date",
        })
    }
}
impl SortField {
    pub const ALL: [SortField; 4] = [
        SortField::Name,
        SortField::LastPlayed,
        SortField::Playtime,
        SortField::ReleaseDate,
    ];
}
impl Into<Column> for SortField {
    fn into(self) -> Column {
        match self {
            Self::Name => Column::Name,
            Self::LastPlayed => Column::LastPlayed,
            Self::Playtime => Column::PlayTimeSecs,
            Self::ReleaseDate => Column::ReleaseDate,
        }
    }
}
#[derive(Debug, Default, Copy, Clone)]
pub struct Sort {
    pub field: SortField,
    pub order: SortOrder,
}
#[derive(Debug, Default, Clone)]
pub struct GamesFilters {
    pub search: String,
    pub sort: Sort,
}
pub async fn get_games(filters: GamesFilters) -> Vec<GameData> {
    let conn = db::connect().await;
    let mut query = GameEntity::find();
    if !filters.search.is_empty() {
        query = query.filter(Column::Name.contains(&filters.search));
    }
    let sort_field: Column = filters.sort.field.into();
    let sort_ord: Order = filters.sort.order.into();
    query = query.order_by(sort_field, sort_ord);
    let raw = query.all(&conn).await.unwrap();
    raw.into_iter().map(Into::into).collect()
}

pub async fn update_game(game: GameData) {
    let mut conn = db::connect().await;
    GameEntity::update(game::ActiveModel {
        id: ActiveValue::Set(game.id),
        logo_url: ActiveValue::Set(game.logo_url),
        icon_url: ActiveValue::Set(game.icon_url),
        name: ActiveValue::Set(game.name),
        header_url: ActiveValue::Set(game.header_url),
        library_type: ActiveValue::Set(game.library_type),
        library_id: ActiveValue::Set(game.library_id),
        install_status: ActiveValue::Set(game.install_status.into()),
        description: ActiveValue::Set(game.description),
        hero_url: ActiveValue::Set(game.hero_url),
        last_played: ActiveValue::Set(game.last_played),
        play_time_secs: ActiveValue::Set(game.play_time.num_seconds()),
        release_date: ActiveValue::Set(game.release_date),
    })
    .exec(&mut conn)
    .await
    .unwrap();
}

use crate::db::game::{ActiveModel, Column};
use crate::{db, LibrarySyncState, ADDONS};
use db::game::Entity as GameEntity;
use db::game_genres::Entity as GameGenresEntity;
use db::genre::Entity as GenreEntity;
use gami_sdk::{GameCommon, GameData, GameLibraryRefOwned, GameMetadata, GameMetadataScanner};
use gami_sdk::{GameLibrary, GameLibraryRef};
use iced::futures::{SinkExt, Stream};
use iced::stream::channel;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, SelectColumns,
};
use std::collections::{BTreeMap, HashSet};
use std::fmt;

pub async fn delete_game(game_id: i32) {
    let mut conn = db::connect().await;

    GameEntity::delete_by_id(game_id)
        .exec(&mut conn)
        .await
        .unwrap();
}

pub async fn clear_all() {
    let mut conn = db::connect().await;
    GameEntity::delete_many().exec(&mut conn).await.unwrap();
    GenreEntity::delete_many().exec(&mut conn).await.unwrap();
    GameGenresEntity::delete_many()
        .exec(&mut conn)
        .await
        .unwrap();
}

pub fn sync_library() -> impl Stream<Item = LibrarySyncState> {
    channel(1, move |mut output| async move {
        let conn = db::connect().await;
        for key in ADDONS.get_keys() {
            if let Some(lib) = ADDONS.get_game_library(key) {
                output.send(LibrarySyncState::LibraryScan).await.unwrap();
                let mut items: Vec<GameData> = lib.scan().into_iter().map(|v| v.into()).collect();
                let existing_query = GameEntity::find()
                    .select_column(Column::LibraryId)
                    .filter(Column::LibraryType.eq(key))
                    .filter(Column::LibraryId.is_in(items.iter().map(|v| v.library_id.as_str())));
                let existing_items: HashSet<String> = existing_query
                    .all(&conn)
                    .await
                    .unwrap()
                    .into_iter()
                    .map(|v| v.library_id)
                    .collect();

                items = items
                    .into_iter()
                    .filter(|v| !existing_items.contains(&v.library_id))
                    .collect();
                log::info!("Pushing {} games to DB", items.len());
                log::info!("Pre-Scanning {} games metadata ", items.len());

                output
                    .send(LibrarySyncState::FetchingMetadata {
                        total: items.len() as u32,
                        current: 0,
                    })
                    .await
                    .unwrap();
                log::info!("Scanning {} games metadata ", items.len());

                let metadatas = ADDONS
                    .get_game_metadata(key)
                    .map(|scanner| {
                        scanner.get_metadatas(
                            &items
                                .iter()
                                .map(GameCommon::get_ref)
                                .collect::<Vec<GameLibraryRef>>(),
                        )
                    })
                    .unwrap_or_default();
                let mut metadatas: BTreeMap<GameLibraryRefOwned, GameMetadata> = metadatas
                    .into_iter()
                    .map(|(k, v)| (GameLibraryRefOwned::from(k), v))
                    .collect();

                for item in &mut items {
                    if let Some(metadata) =
                        metadatas.remove(&GameLibraryRefOwned::from(item.get_ref()))
                    {
                        item.extend(metadata);
                    }
                }

                GameEntity::insert_many(items.iter().cloned().map(|g| ActiveModel {
                    id: ActiveValue::NotSet,
                    name: ActiveValue::Set(g.name),
                    description: ActiveValue::Set(g.description),
                    install_status: ActiveValue::Set(g.install_status.into()),
                    play_time_secs: ActiveValue::Set(g.play_time.num_seconds().into()),
                    last_played: ActiveValue::Set(g.last_played),
                    release_date: ActiveValue::Set(g.release_date),
                    icon_url: ActiveValue::Set(g.icon_url),
                    header_url: ActiveValue::Set(g.header_url),
                    cover_url: ActiveValue::Set(g.cover_url),
                    library_type: ActiveValue::Set(g.library_type),
                    library_id: ActiveValue::Set(g.library_id),
                    completion_status: ActiveValue::Set(g.completion_status.into()),
                }))
                .exec(&conn)
                .await
                .unwrap();
                log::info!("Pushed games to DB");
                output.send(LibrarySyncState::Done).await.unwrap();
            }
        }
    })
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
    CompletionStatus,
}
impl fmt::Display for SortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
            Self::LastPlayed => "Last played",
            Self::Playtime => "Playtime",
            Self::ReleaseDate => "Release date",
            Self::CompletionStatus => "Completion status",
        })
    }
}
impl SortField {
    pub const ALL: [SortField; 5] = [
        SortField::Name,
        SortField::LastPlayed,
        SortField::Playtime,
        SortField::ReleaseDate,
        SortField::CompletionStatus,
    ];
}
impl Into<Column> for SortField {
    fn into(self) -> Column {
        match self {
            Self::Name => Column::Name,
            Self::LastPlayed => Column::LastPlayed,
            Self::Playtime => Column::PlayTimeSecs,
            Self::ReleaseDate => Column::ReleaseDate,
            Self::CompletionStatus => Column::CompletionStatus,
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
    GameEntity::update(ActiveModel {
        id: ActiveValue::Set(game.id),
        icon_url: ActiveValue::Set(game.icon_url),
        name: ActiveValue::Set(game.name),
        header_url: ActiveValue::Set(game.header_url),
        library_type: ActiveValue::Set(game.library_type),
        library_id: ActiveValue::Set(game.library_id),
        install_status: ActiveValue::Set(game.install_status.into()),
        description: ActiveValue::Set(game.description),
        cover_url: ActiveValue::Set(game.cover_url),
        last_played: ActiveValue::Set(game.last_played),
        play_time_secs: ActiveValue::Set(game.play_time.num_seconds()),
        release_date: ActiveValue::Set(game.release_date),
        completion_status: ActiveValue::Set(game.completion_status.into()),
    })
    .exec(&mut conn)
    .await
    .unwrap();
}

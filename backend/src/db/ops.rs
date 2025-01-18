use crate::db::game::{Column, DbGameCompletionStatus, DbGameInstallStatus};
use crate::db::genre::Genre;
use crate::db::{game, game_genres, genre};
use crate::{db, GameFilter, ADDONS};
use chrono::{DateTime, Local, Utc};
use db::game::Entity as GameEntity;
use db::game_genres::Entity as GameGenresEntity;
use db::genre::Entity as GenreEntity;
use gami_sdk::{
    GameCommon, GameData, GameLibraryRefOwned, GameMetadata, GameMetadataScanner, GenreData,
};
use gami_sdk::{GameLibrary, GameLibraryRef};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, SelectColumns,
    TransactionTrait,
};
use std::collections::{HashMap, HashSet};
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

pub async fn sync_library() {
    let mut conn = db::connect().await;
    for key in ADDONS.get_keys() {
        if let Some(lib) = ADDONS.get_game_library(key) {
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
            log::info!("Pushing {} games to DB", items.len());

            items = items
                .into_iter()
                .filter(|v| !existing_items.contains(&v.library_id))
                .collect();
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

            log::debug!("Got metadatas: {:?}", metadatas);
            log::info!("Scanned items metadata: {:?}", metadatas);
            fn get_genres(
                metadatas: &HashMap<GameLibraryRefOwned, GameMetadata>,
            ) -> impl Iterator<Item = &str> {
                metadatas
                    .values()
                    .flat_map(|v| v.genres.iter().map(|g| g.library_id.trim_ascii()))
            }

            let existing_genres_query = GenreEntity::find()
                .select_column(genre::Column::MetadataId)
                .select_column(genre::Column::Id)
                .filter(genre::Column::MetadataSource.eq(key))
                .filter(genre::Column::MetadataId.is_in(get_genres(&metadatas)));

            let mut existing_genres_by_lib_id: HashMap<String, i32> = existing_genres_query
                .all(&conn)
                .await
                .unwrap()
                .into_iter()
                .map(|v| (v.metadata_id, v.id))
                .collect();

            let raw_genres: HashSet<GenreData> = metadatas
                .values()
                .into_iter()
                .flat_map(|m| {
                    m.genres.iter().cloned().filter(|gen| {
                        !existing_genres_by_lib_id.contains_key(gen.library_id.trim_end())
                    })
                })
                .collect();
            log::info!("Genres to skip: {:?}", existing_genres_by_lib_id);
            log::info!("Pushing genres: {:?}", raw_genres);

            let mut txn = conn.begin().await.unwrap();
            for g in raw_genres {
                let res = GenreEntity::insert(genre::ActiveModel {
                    id: ActiveValue::NotSet,
                    name: ActiveValue::Set(g.name.clone().into()),
                    metadata_id: ActiveValue::Set(g.library_id.clone().into()),
                    metadata_source: ActiveValue::Set(key.into()),
                })
                .exec(&mut txn)
                .await
                .unwrap();
                existing_genres_by_lib_id.insert(g.library_id.into(), res.last_insert_id);
            }
            for mut item in items.iter().cloned() {
                let metadata: GameMetadata = metadatas
                    .get(&GameCommon::get_owned_ref(&item))
                    .cloned()
                    .unwrap_or_default();
                item.extend(metadata);
                let res = GameEntity::insert(game::ActiveModel {
                    library_type: ActiveValue::Set(item.library_type),
                    library_id: ActiveValue::Set(item.library_id),
                    name: ActiveValue::Set(item.name),
                    description: ActiveValue::Set(item.description),
                    install_status: ActiveValue::Set(item.install_status.into()),
                    play_time_secs: ActiveValue::Set(item.play_time.num_seconds()),
                    last_played: ActiveValue::Set(item.last_played),
                    icon_url: ActiveValue::Set(item.icon_url),
                    release_date: ActiveValue::Set(item.release_date),
                    ..Default::default()
                })
                .exec(&mut txn)
                .await
                .unwrap();
                log::info!("Got game metadata genres: {:?}", item.genres);
                let to_insert = item
                    .genres
                    .iter()
                    .map(|gg| game_genres::ActiveModel {
                        game_id: ActiveValue::Set(res.last_insert_id),
                        genre_id: ActiveValue::Set(
                            existing_genres_by_lib_id[gg.library_id.trim_end()],
                        ),
                    })
                    .collect::<Vec<game_genres::ActiveModel>>();
                log::info!(
                    "Inserted game metadata: {:?}; GameGenres: {:?}",
                    res.last_insert_id,
                    to_insert
                );
                if !to_insert.is_empty() {
                    GameGenresEntity::insert_many(to_insert)
                        .exec(&mut txn)
                        .await
                        .unwrap();
                }
            }
            txn.commit().await.unwrap();
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
pub struct GameSyncArgs {
    pub search: String,
    pub sort: Sort,
}

pub async fn get_genres() -> Vec<Genre> {
    let conn = db::connect().await;
    GenreEntity::find().all(&conn).await.unwrap()
}
pub async fn get_games(args: GameSyncArgs, filter: GameFilter) -> Vec<GameData> {
    let conn = db::connect().await;
    let mut query = GameEntity::find().find_with_related(GenreEntity);

    if let Some(genre_id) = filter.genre_id {
        query = query.filter(genre::Column::Id.eq(genre_id));
    }

    if !args.search.is_empty() {
        query = query.filter(Column::Name.contains(&args.search));
    }

    if let Some(status) = filter.completion_status {
        query = query.filter(Column::CompletionStatus.eq(DbGameCompletionStatus::from(status)));
    }

    if filter.installed || filter.not_installed {
        if filter.installed && filter.not_installed {
            query = query.filter(
                Column::InstallStatus
                    .eq(DbGameInstallStatus::Installed)
                    .or(Column::InstallStatus.eq(DbGameInstallStatus::InLibrary)),
            );
        }
        query = query.filter(Column::InstallStatus.eq(if filter.installed {
            DbGameInstallStatus::Installed
        } else {
            DbGameInstallStatus::InLibrary
        }));
    }
    let sort_field: Column = args.sort.field.into();
    let sort_ord: Order = args.sort.order.into();
    query = query.order_by(sort_field, sort_ord);
    let raw = query.all(&conn).await.unwrap();
    raw.into_iter()
        .map(|(game, genres)| GameData {
            genres: genres.into_iter().map(|v| v.into()).collect(),
            ..game.into()
        })
        .collect()
}

pub async fn update_game_played(id: i32) -> DateTime<Utc> {
    let mut conn = db::connect().await;
    let curr = Local::now().into();
    GameEntity::update(game::ActiveModel {
        id: ActiveValue::Unchanged(id),
        last_played: ActiveValue::Set(Some(curr)),
        ..Default::default()
    })
    .exec(&mut conn)
    .await
    .unwrap();
    curr
}
pub async fn update_game(game: GameData) {
    let mut conn = db::connect().await;
    GameEntity::update(game::ActiveModel {
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

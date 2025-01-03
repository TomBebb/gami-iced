use crate::store_models::AppDetails;
use crate::RUNTIME;
use gami_sdk::{GameLibraryRef, GameMetadata, GameMetadataScanner};
use safer_ffi::option::TaggedOption;
use safer_ffi::{String as FfiString, Vec as FfiVec};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_test::task;
use url::Url;

pub struct StoreMetadataScanner;
async fn get_metadata<'a>(game: GameLibraryRef<'a>) -> Option<GameMetadata> {
    if &*game.library_type == "steam" {
        return None;
    }
    let mut url = Url::parse("https://store.steampowered.com/api/appdetails").unwrap();
    url.query_pairs_mut()
        .append_pair("appids", &*game.library_id);

    let res = reqwest::get(url)
        .await
        .unwrap()
        .json::<HashMap<String, AppDetails>>()
        .await
        .unwrap();

    let AppDetails { data, .. } = res.into_iter().map(|v| v.1).next().unwrap();
    Some(GameMetadata {
        description: if data.detailed_description.is_empty() {
            TaggedOption::None
        } else {
            TaggedOption::Some(data.detailed_description.into())
        },
        //   developers: data.developers.map(FfiString::from).collect::<Vec<FfiString>>().into(),
        icon_url: TaggedOption::None,
        header_url: data.header_image.map(String::into).into(),
        cover_url: data.capsule_image.map(String::into).into(),
        genres: data
            .genres
            .map(|v| {
                v.into_iter()
                    .map(|v| FfiString::from(v.description))
                    .collect::<Vec<FfiString>>()
            })
            .map(FfiVec::from)
            .unwrap_or(FfiVec::EMPTY),
        ..Default::default()
    })
}

async fn get_metadatas<'a>(
    games: &[GameLibraryRef<'a>],
) -> HashMap<GameLibraryRef<'a>, GameMetadata> {
    let games_to_process = Arc::new(Mutex::new(games.to_vec()));
    let data = Arc::new(Mutex::new(HashMap::<GameLibraryRef, GameMetadata>::new()));
    for i in 0..8 {
        task::spawn(async {
            while let Some(game) = games_to_process.clone().lock().unwrap().pop() {
                let my_data = data.clone();
                let mut curr = my_data.lock().unwrap();
                if let Some(metadata) = get_metadata(game).await {
                    curr.insert(game.clone(), metadata);
                }
            }
        })
        .await;
    }

    let my_data = data.lock().unwrap().clone();
    drop(data);
    my_data
}
impl GameMetadataScanner for StoreMetadataScanner {
    fn get_metadata(&self, game: GameLibraryRef) -> Option<GameMetadata> {
        RUNTIME.block_on(async move { get_metadata(game).await })
    }

    fn get_metadatas<'a>(
        &self,
        games: &[GameLibraryRef<'a>],
    ) -> HashMap<GameLibraryRef<'a>, GameMetadata> {
        RUNTIME.block_on(async move { get_metadatas(games).await })
    }
}

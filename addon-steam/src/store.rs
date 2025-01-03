use crate::store_models::{AppDetails, AppGenre};
use crate::RUNTIME;
use gami_sdk::{GameLibraryRef, GameMetadata, GameMetadataScanner};
use safer_ffi::option::TaggedOption;
use safer_ffi::{String as FfiString, Vec as FfiVec};
use std::collections::HashMap;
use url::Url;

pub struct StoreMetadataScanner;
impl GameMetadataScanner for StoreMetadataScanner {
    fn get_metadata(&self, game: GameLibraryRef) -> Option<GameMetadata> {
        if &*game.library_type == "steam" {
            return None;
        }
        RUNTIME.block_on(async move {
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
        })
    }
}

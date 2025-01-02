use crate::store_models::AppDetails;
use crate::RUNTIME;
use gami_sdk::{GameLibraryRef, GameMetadata, GameMetadataScanner};
use std::collections::HashMap;
use url::Url;

pub struct StoreMetadataScanner;
impl GameMetadataScanner for StoreMetadataScanner {
    fn get_metadata(&self, game: GameLibraryRef) -> Option<GameMetadata> {
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
            println!("{:#?}", res);
            None
        })
    }
}

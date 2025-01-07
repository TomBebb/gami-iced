use crate::store_models::AppDetails;
use crate::RUNTIME;
use chrono::NaiveDate;
use gami_sdk::{GameLibraryRef, GameLibraryRefOwned, GameMetadata, GameMetadataScanner};
use once_cell::sync::Lazy;
use regex::Regex;
use safer_ffi::option::TaggedOption;
use safer_ffi::{String as FfiString, Vec as FfiVec};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;
use url::Url;

const RELEASE_DATE_RAW: &str =
    r"^([0-9]{1,2}) ([A-Z][a-z]+), ([0-9]+)|([A-Z][a-z]+) ([0-9]+), ([0-9]+)$";
fn to_month(month: &str) -> u8 {
    match month {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => panic!("Invalid month: {:?}", month),
    }
}
const RELEASE_DATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(RELEASE_DATE_RAW).unwrap());

fn parse_release_date(date: &str) -> Option<NaiveDate> {
    RELEASE_DATE_REGEX.captures(date).map(|parts| {
        if parts.get(3).is_some() {
            NaiveDate::from_ymd_opt(
                parts[3].parse().unwrap(),
                to_month(&parts[2]) as u32,
                parts[1].parse().unwrap(),
            )
        } else {
            NaiveDate::from_ymd_opt(
                parts[6].parse().unwrap(),
                to_month(&parts[4]) as u32,
                parts[5].parse().unwrap(),
            )
        }
        .unwrap()
    })
}
pub struct StoreMetadataScanner;
async fn get_metadata<'a>(game: GameLibraryRef<'a>) -> Option<GameMetadata> {
    if &*game.library_type != "steam" {
        return None;
    }
    let mut url = Url::parse("https://store.steampowered.com/api/appdetails").unwrap();
    url.query_pairs_mut()
        .append_pair("appids", &*game.library_id);

    println!("Fetch URL: {}", url);
    let raw_res = reqwest::get(url)
        .await
        .unwrap()
        .json::<Option<HashMap<String, AppDetails>>>()
        .await
        .unwrap();
    let res = if let Some(res) = raw_res {
        res
    } else {
        return None;
    };

    let AppDetails { data, .. } = res.into_iter().map(|v| v.1).next().unwrap();
    data.map(|data| GameMetadata {
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
        release_date_timestamp: data
            .release_date
            .and_then(|v| v.date)
            .as_ref()
            .and_then(|v| if v.is_empty() { None } else { Some(v) })
            .and_then(|v| parse_release_date(&v))
            .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as u32)
            .into(),
        ..Default::default()
    })
}

async fn get_metadatas<'a>(
    games: &[GameLibraryRef<'a>],
) -> HashMap<GameLibraryRefOwned, GameMetadata> {
    let games: Vec<GameLibraryRefOwned> = games
        .into_iter()
        .cloned()
        .map(GameLibraryRefOwned::from)
        .collect();
    let data = Arc::new(Mutex::new(
        HashMap::<GameLibraryRefOwned, GameMetadata>::new(),
    ));
    let mut tasks = JoinSet::new();
    for game in games {
        let my_data = data.clone();

        tasks.spawn(async move {
            if let Some(metadata) = get_metadata(game.as_ref()).await {
                let mut curr = my_data.lock().unwrap();
                curr.insert(game, metadata);
            }
        });
    }
    while let Some(res) = tasks.join_next().await {
        res.unwrap();
    }
    let my_data = data.clone().lock().unwrap().clone();

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

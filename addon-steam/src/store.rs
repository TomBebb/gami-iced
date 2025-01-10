use crate::curl::Collector;
use crate::store_models::AppDetails;
use crate::RUNTIME;
use chrono::NaiveDate;
use curl::easy::Easy2;
use curl::multi::{Easy2Handle, Multi};
use gami_sdk::{BoxFuture, GameLibraryRef, GameLibraryRefOwned, GameMetadata, GameMetadataScanner};
use once_cell::sync::Lazy;
use regex::Regex;
use safer_ffi::option::TaggedOption;
use safer_ffi::{String as FfiString, Vec as FfiVec};
use std::collections::HashMap;
use std::time::Duration;
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

fn request_metadata(game: GameLibraryRef) -> Easy2<Collector> {
    let mut request = Easy2::new(Collector::default());
    let mut url = Url::parse("https://store.steampowered.com/api/appdetails").unwrap();
    url.query_pairs_mut()
        .append_pair("appids", &*game.library_id);

    request.url(url.as_str()).unwrap();
    request
}

fn map_metadata(res: &str) -> Option<GameMetadata> {
    let raw_res: Option<HashMap<String, AppDetails>> = serde_json::from_str(res).unwrap();
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
    on_process_one: Box<dyn Fn() -> BoxFuture<'a, ()>>,
) -> HashMap<GameLibraryRefOwned, GameMetadata> {
    let mut multi = Multi::new();
    multi.pipelining(true, true).unwrap();
    let mut data = HashMap::<GameLibraryRefOwned, GameMetadata>::new();
    let handles = games
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, game)| {
            let req = request_metadata(game);
            let mut handle = multi.add2(req).unwrap();
            handle.set_token(index).unwrap();
            handle
        })
        .collect::<Vec<Easy2Handle<Collector>>>();
    let mut still_alive = true;
    while still_alive {
        // We still need to process the last messages when
        // `Multi::perform` returns "0".
        if multi.perform().unwrap() == 0 {
            still_alive = false;
        }
        multi.messages(|message| {
            let index = message.token().expect("failed to get token");
            let handle: &Easy2Handle<Collector> = &handles[index];

            let text: &str = std::str::from_utf8(handle.get_ref().as_ref()).unwrap();
            if let Some(metadata) = map_metadata(text) {
                data.insert(games[index].to_owned(), metadata);
            }

            on_process_one();
        })
    }
    if still_alive {
        // The sleeping time could be reduced to allow other processing.
        // For instance, a thread could check a condition signalling the
        // thread shutdown.
        multi.wait(&mut [], Duration::from_secs(60)).unwrap();
    }
    data
}

async fn get_metadata<'a>(my_ref: GameLibraryRefOwned) -> Option<GameMetadata> {
    tokio::task::spawn_blocking(move || {
        let req = request_metadata(my_ref.as_ref());
        req.perform().unwrap();
        let text: &str = std::str::from_utf8(req.get_ref().as_ref()).unwrap();
        map_metadata(text)
    })
    .await
    .unwrap()
}
impl GameMetadataScanner for StoreMetadataScanner {
    fn get_metadata(&self, game: GameLibraryRef) -> Option<GameMetadata> {
        RUNTIME.block_on(async move { get_metadata(game.into()).await })
    }

    fn get_metadatas<'a>(
        &self,
        games: &[GameLibraryRef<'a>],
        on_process_one: Box<dyn Fn() -> BoxFuture<'a, ()>>,
    ) -> HashMap<GameLibraryRefOwned, GameMetadata> {
        RUNTIME.block_on(async move { get_metadatas(games, on_process_one).await })
    }
}

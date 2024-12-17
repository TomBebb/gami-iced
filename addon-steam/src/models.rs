use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct OwnedGame {
    pub appid: u64,
    pub name: String,
    pub playtime_forever: u64,
    pub img_icon_url: String,
    pub rtime_last_played: u64,
    pub playtime_disconnected: u8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OwnedGames {
    pub games: Vec<OwnedGame>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub response: T,
}

pub type OwnedGamesResponse = Response<OwnedGames>;

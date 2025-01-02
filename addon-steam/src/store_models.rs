use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppDetailsData {
    pub name: String,
    pub detailed_description: String,
    pub short_description: String,
    pub about_the_game: String,
    pub website: Option<String>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub genres: Vec<AppGenre>,
    pub release_date: Option<AppReleaseDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppGenre {
    pub id: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppReleaseDate {
    pub coming_soon: bool,
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppDetails {
    pub success: bool,
    pub data: AppDetailsData,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AppDetailsData {
    pub name: String,
    pub detailed_description: String,
    pub short_description: String,
    pub about_the_game: String,
    pub website: Option<String>,
    pub developers: Option<Vec<String>>,
    pub publishers: Option<Vec<String>>,
    pub genres: Option<Vec<AppGenre>>,
    pub release_date: Option<AppReleaseDate>,
    pub header_image: Option<String>,
    pub capsule_image: Option<String>,
    pub capsule_imagev5: Option<String>,
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

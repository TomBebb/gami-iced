use gami_sdk::CompletionStatus;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameTextField {
    Name,
    Description,
    IconUrl,
    HeaderUrl,
    CoverUrl,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameDateField {
    LastPlayed,
    ReleaseDate,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameDurationField {
    TimePlayed,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct GameFilter {
    completion_status: Option<CompletionStatus>,
    pub installed: bool,
    pub not_installed: bool,
}

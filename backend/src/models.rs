use std::fmt;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LibrarySyncState {
    LibraryScan,
    FetchingMetadata,
    Done,
}
impl fmt::Display for LibrarySyncState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::LibraryScan => "Scanning Library",
            Self::FetchingMetadata => "Fetching Metadata",
            Self::Done => "Done",
        })
    }
}

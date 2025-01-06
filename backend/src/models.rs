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
    FetchingMetadata { total: u32, current: u32 },
    Done,
}
impl fmt::Display for LibrarySyncState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::LibraryScan => f.write_str("Scanning Library"),
            Self::FetchingMetadata { current, total } => {
                write!(f, "Fetching Metadata: {} / {}", current, total)
            }
            Self::Done => f.write_str("Done"),
        }
    }
}

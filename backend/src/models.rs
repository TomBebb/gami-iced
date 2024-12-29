#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameTextField {
    Name,
    Description,
    IconUrl,
    HeaderUrl,
    LogoUrl,
    HeroUrl,
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

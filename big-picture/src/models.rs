use gilrs::Button;
use iced::keyboard::key::Named;
use iced::keyboard::Key;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NavDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Input {
    Nav(NavDirection),
    Confirm,
    Back,
}
impl TryFrom<Key> for Input {
    type Error = ();
    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::Named(Named::ArrowLeft) => Ok(Input::Nav(NavDirection::Left)),
            Key::Named(Named::ArrowRight) => Ok(Input::Nav(NavDirection::Right)),
            Key::Named(Named::ArrowUp) => Ok(Input::Nav(NavDirection::Up)),
            Key::Named(Named::ArrowDown) => Ok(Input::Nav(NavDirection::Down)),
            Key::Named(Named::Space) => Ok(Input::Confirm),
            Key::Named(Named::Backspace) => Ok(Input::Back),
            _ => Err(()),
        }
    }
}
impl TryFrom<Button> for Input {
    type Error = ();
    fn try_from(value: Button) -> Result<Self, Self::Error> {
        match value {
            Button::DPadLeft => Ok(Input::Nav(NavDirection::Left)),
            Button::DPadRight => Ok(Input::Nav(NavDirection::Right)),
            Button::DPadUp => Ok(Input::Nav(NavDirection::Up)),
            Button::DPadDown => Ok(Input::Nav(NavDirection::Down)),
            Button::South => Ok(Input::Confirm),
            Button::East => Ok(Input::Back),
            _ => Err(()),
        }
    }
}

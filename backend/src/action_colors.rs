use iced::widget::button;
use iced::widget::button::Status;
use iced::Theme;

#[derive(Debug, Copy, Clone)]
pub enum StyleVariant {
    Primary,
    Secondary,
    Success,
    Danger,
}
impl StyleVariant {
    pub fn button_style<'a, Message>(self, theme: &Theme, status: Status) -> button::Style {
        match self {
            StyleVariant::Primary => button::primary(theme, status),
            StyleVariant::Secondary => button::secondary(theme, status),
            StyleVariant::Success => button::success(theme, status),
            StyleVariant::Danger => button::danger(theme, status),
        }
    }
}

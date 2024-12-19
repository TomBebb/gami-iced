use gami_backend::GameAction;
use iced::widget::{text, Text};
use iced::Font;

pub const FONT_BYTES: &[u8] = include_bytes!("icons/mdi.ttf");

const MDI_FONT: Font = Font::with_name("Material Symbols Outlined");
#[derive(Debug, Clone, Copy)]
pub enum IconKind {
    Action(GameAction),
    Library,
    Settings,
    Achievements,
    Addons,
    Appearance,
}

pub fn get_icon(kind: IconKind) -> Text<'static> {
    let code = match kind {
        IconKind::Action(GameAction::Play) => '\u{e037}',
        IconKind::Action(GameAction::Install) => '\u{e145}',
        IconKind::Action(GameAction::Uninstall) => '\u{e15b}',
        IconKind::Action(GameAction::Delete) => '\u{e037}',
        IconKind::Action(GameAction::Edit) => '\u{f88d}',
        IconKind::Library => '\u{e02f}',
        IconKind::Achievements => '\u{e71a}',
        IconKind::Settings => '\u{e8b8}',
        IconKind::Addons => '\u{e87b}',
        IconKind::Appearance => '\u{e8f4}',
    };
    text(code).font(MDI_FONT)
}

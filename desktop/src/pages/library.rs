use gami_backend::db;
use gami_sdk::{GameData, GameInstallStatus};
use iced::advanced::svg::Handle;
use iced::alignment::Vertical;
use iced::widget::{button, column, combo_box, row, scrollable, text, Container, Svg};
use iced::{ContentFit, Element, Fill, Task, Theme};
use iced_aw::ContextMenu;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum LibraryViewType {
    List,
    Table,
    Grid,
}
impl LibraryViewType {
    const ALL: [LibraryViewType; 3] = [Self::List, Self::Table, Self::Grid];
}
impl fmt::Display for LibraryViewType {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::List => "List",
            Self::Table => "Table",
            Self::Grid => "Grid",
        })
    }
}
#[derive(Clone, Debug)]
pub struct LibraryPage {
    view_types: combo_box::State<LibraryViewType>,
    view_type: LibraryViewType,
    games: Vec<GameData>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewSelected(LibraryViewType),
    ShowAddDialog,
    GameAction(GameAction, GameData),
    RefreshGames,
    ReloadCache,
    CacheReloaded(Vec<GameData>),
}
#[derive(Debug, Clone, Copy)]
pub enum GameAction {
    Play,
    Install,
    Uninstall,
    Delete,
    Edit,
}
#[derive(Debug, Clone, Copy)]
struct GameActionData {
    name: &'static str,
    icon: &'static [u8],
    kind: GameAction,
}
const PLAY_ACTION: GameActionData = GameActionData {
    name: "Play",
    icon: include_bytes!("../icons/tabler--play.svg"),
    kind: GameAction::Play,
};
const INSTALL_ACTION: GameActionData = GameActionData {
    name: "Install",
    icon: include_bytes!("../icons/tabler--plus.svg"),
    kind: GameAction::Install,
};
const UNINSTALL_ACTION: GameActionData = GameActionData {
    name: "Uninstall",
    icon: include_bytes!("../icons/tabler--minus.svg"),
    kind: GameAction::Uninstall,
};
const DELETE_ACTION: GameActionData = GameActionData {
    name: "Delete",
    icon: include_bytes!("../icons/tabler--x.svg"),
    kind: GameAction::Delete,
};
const EDIT_ACTION: GameActionData = GameActionData {
    name: "Edit",
    icon: include_bytes!("../icons/tabler--edit.svg"),
    kind: GameAction::Edit,
};
const fn get_actions(status: GameInstallStatus) -> &'static [GameActionData] {
    match status {
        GameInstallStatus::Installed => {
            &[PLAY_ACTION, UNINSTALL_ACTION, EDIT_ACTION, DELETE_ACTION]
        }
        _ => &[INSTALL_ACTION, EDIT_ACTION, DELETE_ACTION],
    }
}
impl LibraryPage {
    pub fn new() -> Self {
        let me = Self {
            view_types: combo_box::State::new(LibraryViewType::ALL.to_vec()),
            view_type: LibraryViewType::List,
            games: Vec::new(),
        };
        me
    }
    fn game_menu<'a>(
        &'a self,
        game: &'a GameData,
        underlay: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let actions = get_actions(game.install_status);
        ContextMenu::new(underlay, || {
            column(actions.iter().map(|ga| {
                let svg: Svg<'static, Theme> = Svg::new(Handle::from_memory(ga.icon));
                button(row![svg.width(24), text(ga.name)])
                    .on_press_with(|| Message::GameAction(ga.kind, game.clone()))
                    .width(Fill)
                    .into()
            }))
            .width(120)
            .into()
        })
        .into()
    }
    pub fn view(&self) -> Element<Message> {
        let items: Vec<Element<Message>> = self
            .games
            .iter()
            .map(|game| (game, Element::from(row![text(&game.name)].width(Fill))))
            .map(|(game, raw)| self.game_menu(game, Container::new(raw).into()))
            .collect();

        let toolbar = Element::from(
            row![
                combo_box(
                    &self.view_types,
                    "Pick a view type",
                    Some(&self.view_type),
                    Message::ViewSelected,
                ),
                button(
                    Svg::new(Handle::from_memory(include_bytes!(
                        "../icons/tabler--plus.svg"
                    )))
                    .content_fit(ContentFit::Contain)
                )
                .width(30)
                .on_press(Message::ShowAddDialog),
                button(
                    Svg::new(Handle::from_memory(include_bytes!(
                        "../icons/tabler--refresh.svg"
                    )))
                    .content_fit(ContentFit::Contain)
                )
                .width(30)
                .on_press(Message::RefreshGames)
            ]
            .spacing(3)
            .align_y(Vertical::Center),
        );
        column![toolbar, scrollable(column(items))].into()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RefreshGames => {
                return Task::perform(db::ops::sync_library(), |_| Message::ReloadCache);
            }
            Message::ReloadCache => {
                return Task::perform(db::ops::get_games(), Message::CacheReloaded)
            }
            Message::ViewSelected(view_type) => {
                self.view_type = view_type;
            }
            Message::CacheReloaded(cache) => self.games = cache,
            Message::GameAction(GameAction::Play, game) if game.library_type == "steam" => {
                //TODO: use addon
                open::that(&format!("steam://rungameid/{}", game.library_id)).unwrap();
            }
            Message::GameAction(GameAction::Install, game) if game.library_type == "steam" => {
                //TODO: use addon
                open::that(&format!("steam://install/{}", game.library_id)).unwrap();
            }
            Message::GameAction(GameAction::Uninstall, game) if game.library_type == "steam" => {
                //TODO: use addon
                open::that(&format!("steam://uninstall/{}", game.library_id)).unwrap();
            }
            v => println!("{:?}", v),
        }

        Task::none()
    }
}

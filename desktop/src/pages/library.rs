use crate::widgets::library_table::{LibraryTable, TableMessage};
use gami_backend::db;
use gami_sdk::{GameData, GameInstallStatus};
use iced::advanced::svg::Handle;
use iced::alignment::Vertical;
use iced::widget::{
    button, column, container, image, row, scrollable, text, tooltip, Container, Svg,
};
use iced::{ContentFit, Element, Fill, Task, Theme};
use iced_aw::ContextMenu;
use std::cell::LazyCell;
use std::cmp::PartialEq;
use url::Url;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LibraryViewType {
    List,
    Table,
    Grid,
}

#[derive(Debug, Clone)]
struct LibraryViewTypeMeta {
    value: LibraryViewType,
    name: &'static str,
    icon: Handle,
}
const VIEW_TYPES: LazyCell<[LibraryViewTypeMeta; 3]> = LazyCell::new(|| {
    [
        LibraryViewTypeMeta {
            value: LibraryViewType::List,
            name: "List",
            icon: Handle::from_memory(include_bytes!("../icons/tabler--list.svg").to_vec()),
        },
        LibraryViewTypeMeta {
            value: LibraryViewType::Table,
            name: "Table",
            icon: Handle::from_memory(include_bytes!("../icons/tabler--table.svg").to_vec()),
        },
        LibraryViewTypeMeta {
            value: LibraryViewType::Grid,
            name: "Grid",
            icon: Handle::from_memory(include_bytes!("../icons/tabler--grid-4x4.svg").to_vec()),
        },
    ]
});
#[derive(Clone, Debug)]
pub struct LibraryPage {
    curr_index: usize,
    view_type: LibraryViewType,
    games: Vec<GameData>,
    table: LibraryTable,
}

#[derive(Debug, Clone)]
pub enum Message {
    Table(TableMessage),
    ViewSelected(LibraryViewType),
    ShowAddDialog,
    GameAction(GameAction, GameData),
    RefreshGames,
    ReloadCache,
    CacheReloaded(Vec<GameData>),
    SelectGame(usize),
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

impl Default for LibraryPage {
    fn default() -> Self {
        Self {
            view_type: LibraryViewType::List,
            games: Vec::new(),
            curr_index: 0,
            table: LibraryTable::new(),
        }
    }
}

impl LibraryPage {
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
        let items: Element<Message> = match self.view_type {
            LibraryViewType::Table => self.table.view().map(Message::Table),

            LibraryViewType::List => scrollable(column(
                self.games
                    .iter()
                    .enumerate()
                    .map(|(index, game)| {
                        let raw_icon_url = game.icon_url.as_ref().map(String::as_str).unwrap_or("");
                        (
                            game,
                            Element::from(
                                button(
                                    row![
                                        text(&game.name).width(Fill),
                                        image(if raw_icon_url.is_empty() {
                                            "".into()
                                        } else {
                                            let url = Url::parse(raw_icon_url).unwrap();
                                            println!("scheme: {:?}; URL: {:?}", url.scheme(), url);
                                            if url.scheme() == "file" {
                                                url.path().to_owned()
                                            } else {
                                                raw_icon_url.to_owned()
                                            }
                                        }),
                                    ]
                                    .width(Fill),
                                )
                                .style(if index == self.curr_index {
                                    button::primary
                                } else {
                                    button::text
                                })
                                .on_press(Message::SelectGame(index)),
                            ),
                        )
                    })
                    .map(|(game, raw)| self.game_menu(game, Container::new(raw).into()))
                    .collect::<Vec<Element<Message>>>(),
            ))
            .into(),
            LibraryViewType::Grid => text("TODO: GRID").into(),
        };
        let toolbar = Element::from(
            row![
                Container::new(
                    row(VIEW_TYPES.iter().cloned().map(|v| {
                        tooltip(
                            button(Svg::new(v.icon)).on_press_maybe(if self.view_type == v.value {
                                None
                            } else {
                                Some(Message::ViewSelected(v.value))
                            }),
                            container(text(v.name))
                                .padding(6)
                                .style(container::rounded_box),
                            tooltip::Position::Bottom,
                        )
                        .into()
                    }))
                    .spacing(2)
                ),
                text("").width(Fill),
                tooltip(
                    button(
                        Svg::new(Handle::from_memory(include_bytes!(
                            "../icons/tabler--plus.svg"
                        )))
                        .content_fit(ContentFit::Contain)
                    )
                    .style(button::success)
                    .width(30)
                    .on_press(Message::ShowAddDialog),
                    container(text("Add a new game"))
                        .padding(6)
                        .style(container::rounded_box),
                    tooltip::Position::Bottom,
                ),
                tooltip(
                    button(
                        Svg::new(Handle::from_memory(include_bytes!(
                            "../icons/tabler--refresh.svg"
                        )))
                        .content_fit(ContentFit::Contain)
                    )
                    .style(button::success)
                    .width(30)
                    .on_press(Message::RefreshGames),
                    container(text("Re-sync your games library"))
                        .padding(6)
                        .style(container::rounded_box),
                    tooltip::Position::Bottom,
                )
            ]
            .spacing(3)
            .align_y(Vertical::Center),
        );
        column![toolbar, items].into()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Table(tbl) => {
                return self.table.update(tbl).map(Message::Table);
            }
            Message::RefreshGames => {
                return Task::perform(db::ops::sync_library(), |_| Message::ReloadCache);
            }
            Message::ReloadCache => {
                return Task::perform(db::ops::get_games(), Message::CacheReloaded)
            }
            Message::ViewSelected(view_type) => {
                self.view_type = view_type;
            }
            Message::CacheReloaded(cache) => {
                self.games = cache.clone();
                self.table.rows = cache;
            }
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
            Message::SelectGame(index) => {
                self.curr_index = index;
            }
            v => println!("{:?}", v),
        }

        Task::none()
    }
}

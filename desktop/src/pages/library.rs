use crate::widgets::library_table::{LibraryTable, TableMessage};
use gami_backend::db::ops::{GamesFilters, SortField, SortOrder};
use gami_backend::{db, get_actions, GameAction, ADDONS};
use gami_sdk::{GameData, GameInstallStatus, GameLibrary};
use iced::advanced::svg::Handle;
use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::{
    button, column, container, image, pick_list, row, scrollable, text, text_input, tooltip,
    Container, Svg,
};
use iced::{ContentFit, Element, Fill, Font, Length, Task, Theme};
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
    filters: GamesFilters,
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
    SearchChanged(String),
    SortFieldChanged(SortField),
    ToggleSortDirection,
}
impl LibraryPage {
    fn auto_installer_icon(status: GameInstallStatus) -> Handle {
        Handle::from_memory(match status {
            GameInstallStatus::Installed => {
                include_bytes!("../icons/tabler--circle-check.svg").as_slice()
            }
            GameInstallStatus::Installing => {
                include_bytes!("../icons/tabler--loader-2.svg").as_slice()
            }
            GameInstallStatus::Queued => {
                include_bytes!("../icons/tabler--player-pause.svg").as_slice()
            }
            _ => include_bytes!("../icons/tabler--circle-x.svg"),
        })
    }
    pub fn new() -> Self {
        let me = Self {
            view_type: LibraryViewType::List,
            games: Vec::new(),
            curr_index: 0,
            table: LibraryTable::new(),
            filters: GamesFilters::default(),
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
        let curr: Option<&GameData> = self.games.as_slice().get(self.curr_index);
        let items: Element<Message> = match self.view_type {
            LibraryViewType::Table => self.table.view().map(Message::Table),

            LibraryViewType::List => row![
                scrollable(column(
                    self.games
                        .iter()
                        .enumerate()
                        .map(|(index, game)| {
                            let raw_icon_url =
                                game.icon_url.as_ref().map(String::as_str).unwrap_or("");
                            (
                                game,
                                Element::from(
                                    button(
                                        row![
                                            image(if raw_icon_url.is_empty() {
                                                "".into()
                                            } else {
                                                Url::parse(raw_icon_url).unwrap().path().to_owned()
                                            })
                                            .width(32),
                                            text(&game.name).width(Fill),
                                            Svg::new(Self::auto_installer_icon(
                                                game.install_status
                                            ))
                                            .width(Length::Shrink),
                                        ]
                                        .width(Fill)
                                        .spacing(2),
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
                        .map(|(game, raw)| self.game_menu(game, raw))
                        .collect::<Vec<Element<Message>>>(),
                ))
                .width(Fill),
                scrollable(
                    if let Some(curr) = curr {
                        let last_played = curr
                            .last_played
                            .map(|t| t.to_string())
                            .unwrap_or("None".into());
                        column![
                            text(&curr.name),
                            text(&curr.description),
                            column![
                                text("Last played:").font(Font {
                                    weight: Weight::Semibold,
                                    ..Font::default()
                                }),
                                text(last_played)
                            ]
                        ]
                    } else {
                        column![]
                    }
                    .spacing(2)
                )
                .width(Fill)
            ]
            .into(),
            LibraryViewType::Grid => text("TODO: GRID").into(),
        };
        let toolbar = Element::from(
            row![
                text_input("Enter search", &self.filters.search)
                    .on_input(Message::SearchChanged)
                    .width(Length::FillPortion(7)),
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
                )
                .width(Length::FillPortion(3)),
                row![
                    button(
                        Svg::new(if self.filters.sort.order == SortOrder::Ascending {
                            Handle::from_memory(include_bytes!(
                                "../icons/tabler--sort-descending.svg"
                            ))
                        } else {
                            Handle::from_memory(include_bytes!(
                                "../icons/tabler--sort-ascending.svg"
                            ))
                        })
                        .width(24)
                        .height(24)
                    )
                    .on_press(Message::ToggleSortDirection),
                    pick_list(
                        &SortField::ALL[..],
                        Some(self.filters.sort.field),
                        Message::SortFieldChanged
                    )
                ]
                .width(Length::FillPortion(3)),
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
            Message::SearchChanged(query) => {
                self.filters.search = query;
                return self.update(Message::ReloadCache);
            }
            Message::ReloadCache => {
                return Task::perform(
                    db::ops::get_games(self.filters.clone()),
                    Message::CacheReloaded,
                );
            }
            Message::ViewSelected(view_type) => {
                self.view_type = view_type;
            }
            Message::CacheReloaded(cache) => {
                self.games = cache.clone();
                self.table.rows = cache;
            }
            Message::GameAction(GameAction::Delete, game) => {
                return Task::perform(db::ops::delete_game(game.id), |_| Message::ReloadCache)
            }
            Message::GameAction(GameAction::Play, game) => {
                let addon = ADDONS
                    .get_game_library(&game.library_type)
                    .cloned()
                    .expect("Failed to load library");
                addon.launch(&game.into());
            }
            Message::GameAction(GameAction::Install, game) => {
                let addon = ADDONS
                    .get_game_library(&game.library_type)
                    .cloned()
                    .expect("Failed to load library");
                addon.install(&game.into());
            }
            Message::GameAction(GameAction::Uninstall, game) => {
                let addon = ADDONS
                    .get_game_library(&game.library_type)
                    .cloned()
                    .expect("Failed to load library");
                addon.uninstall(&game.into());
            }
            Message::SelectGame(index) => {
                self.curr_index = index;
            }
            Message::SortFieldChanged(field) => {
                self.filters.sort.field = field;
                return self.update(Message::ReloadCache);
            }
            Message::ToggleSortDirection => {
                self.filters.sort.order = match self.filters.sort.order {
                    SortOrder::Ascending => SortOrder::Descending,
                    SortOrder::Descending => SortOrder::Ascending,
                };
                return self.update(Message::ReloadCache);
            }
            v => println!("{:?}", v),
        }

        Task::none()
    }
}

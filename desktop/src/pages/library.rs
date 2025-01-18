use crate::widgets::library_table::{LibraryTable, TableMessage};
use crate::widgets::number_input::number_input;
use chrono::{DateTime, Utc};
use gami_backend::db::genre::Genre;
use gami_backend::db::ops;
use gami_backend::db::ops::{GameSyncArgs, SortField, SortOrder};
use gami_backend::{db, get_actions, Direction, GameAction, GameFilter, GameTextField, ADDONS};
use gami_sdk::{
    CompletionStatus, EditableEnum, GameCommon, GameData, GameInstallStatus, GameLibrary, GenreData,
};
use iced::advanced::svg::Handle;
use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::{
    button, checkbox, column, container, image, pick_list, row, scrollable, text, text_input,
    tooltip, Button, Column, Container, Row, Scrollable, Svg,
};
use iced::{ContentFit, Element, Fill, Font, Length, Task, Theme};
use iced_aw::ContextMenu;
use std::cell::LazyCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
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
    edit_game: Option<GameData>,
    curr_index: usize,
    view_type: LibraryViewType,
    games: Vec<GameData>,
    table: LibraryTable,
    args: GameSyncArgs,
    filter: GameFilter,
    display_filter: bool,
    all_genres_by_meta_id: HashMap<String, Genre>,
}

#[derive(Debug, Clone)]
pub enum FilterMessage {
    SetInstalled(bool),
    SetNotInstalled(bool),
    SetCompletionStatus(Option<CompletionStatus>),
    SetGenreLibraryId(Option<GenreData>),
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
    CloseEditor,
    EditorTextChanged(GameTextField, String),
    EditorCompletionStatusChanged(CompletionStatus),
    SaveEditor,
    MoveInDir(Direction),
    UpdateLastPlayed(i32, DateTime<Utc>),
    ToggleFilterDisplay,
    Filter(FilterMessage),
    NoOp,
    OnGenresLoad(Vec<Genre>),
}

fn update_filter(filter: &mut GameFilter, message: FilterMessage) {
    match message {
        FilterMessage::SetInstalled(v) => filter.installed = v,
        FilterMessage::SetNotInstalled(v) => filter.not_installed = v,
        FilterMessage::SetCompletionStatus(v) => filter.completion_status = v,
        FilterMessage::SetGenreLibraryId(v) => {
            filter.genre_metadata_id = v.map(|g| g.library_id.into())
        }
    }
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
            edit_game: None,
            view_type: LibraryViewType::List,
            games: Vec::new(),
            curr_index: 0,
            table: LibraryTable::new(),
            args: GameSyncArgs::default(),
            filter: GameFilter::default(),
            display_filter: false,
            all_genres_by_meta_id: HashMap::new(),
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
                    .style(|theme, status| ga.color.button_style::<Message>(theme, status))
                    .on_press_with(|| Message::GameAction(ga.kind, game.clone()))
                    .width(Fill)
                    .into()
            }))
            .width(120)
            .into()
        })
        .into()
    }
    fn filter_row<'a>(
        &'a self,
        name: &'a str,
        elem: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        column![text(name), elem.into()].into()
    }
    fn filter_row_pick_list<'a, TEnum>(
        &'a self,
        name: &'a str,
        value: Option<TEnum>,
        map: impl Fn(Option<TEnum>) -> FilterMessage + 'a,
    ) -> Element<'a, Message>
    where
        TEnum: EditableEnum + Clone,
    {
        self.filter_row(
            name,
            row![
                pick_list(TEnum::ALL, value.clone(), move |v| Message::Filter(map(
                    Some(v)
                )))
                .width(Length::FillPortion(4)),
                button(
                    Svg::new(Handle::from_memory(include_bytes!(
                        "../icons/tabler--backspace.svg"
                    )))
                    .content_fit(ContentFit::Contain)
                )
                .style(button::danger)
                .width(Length::FillPortion(1))
                .on_press_maybe(
                    value.map(|_| Message::Filter(FilterMessage::SetCompletionStatus(None)))
                )
                .height(30),
            ]
            .width(Fill),
        )
    }
    fn filter_view(&self) -> Element<Message> {
        let filter = &self.filter;
        column![
            checkbox("Installed", filter.installed)
                .on_toggle(|v| Message::Filter(FilterMessage::SetInstalled(v))),
            checkbox("Not installed", filter.not_installed)
                .on_toggle(|v| Message::Filter(FilterMessage::SetNotInstalled(v))),
            self.filter_row_pick_list(
                "Completion Status",
                self.filter.completion_status,
                FilterMessage::SetCompletionStatus
            ),
            self.filter_row(
                "Genres",
                row![
                    pick_list(
                        self.all_genres_by_meta_id
                            .values()
                            .map(|v| GenreData {
                                library_id: v.metadata_id.clone().into(),
                                name: v.name.clone().into()
                            })
                            .collect::<Box<[GenreData]>>(),
                        filter.genre_metadata_id.as_ref().map(|mid| GenreData {
                            library_id: mid.to_owned().into(),
                            name: self.all_genres_by_meta_id[mid.trim_end()]
                                .name
                                .clone()
                                .into()
                        }),
                        move |v| Message::Filter(FilterMessage::SetGenreLibraryId(Some(v)))
                    )
                    .width(Length::FillPortion(4)),
                    button(
                        Svg::new(Handle::from_memory(include_bytes!(
                            "../icons/tabler--backspace.svg"
                        )))
                        .content_fit(ContentFit::Contain)
                    )
                    .style(button::danger)
                    .width(Length::FillPortion(1))
                    .on_press_maybe(
                        filter
                            .genre_metadata_id
                            .clone()
                            .map(|_| Message::Filter(FilterMessage::SetGenreLibraryId(None)))
                    )
                    .height(30),
                ]
                .width(Fill),
            )
        ]
        .padding(2)
        .spacing(3)
        .into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        Element::from(
            row![
                tooltip(
                    button(
                        Svg::new(Handle::from_memory(include_bytes!(
                            "../icons/tabler--filter.svg"
                        )))
                        .content_fit(ContentFit::Contain)
                    )
                    .on_press(Message::ToggleFilterDisplay)
                    .style(button::secondary)
                    .height(30),
                    container(text("Filter games"))
                        .padding(6)
                        .style(container::rounded_box),
                    tooltip::Position::Bottom,
                ),
                text_input("Enter search", &self.args.search)
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
                        Svg::new(if self.args.sort.order == SortOrder::Ascending {
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
                        Some(self.args.sort.field),
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
                    .height(30)
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
                    .style(button::primary)
                    .height(30)
                    .on_press(Message::RefreshGames),
                    container(text("Re-sync your games library"))
                        .padding(6)
                        .style(container::rounded_box),
                    tooltip::Position::Bottom,
                )
            ]
            .spacing(3)
            .align_y(Vertical::Center),
        )
    }

    fn editor(game: &GameData) -> Column<Message> {
        fn editor_row<'a>(
            name: &'a str,
            value: impl Into<Element<'a, Message>>,
        ) -> Row<'a, Message> {
            row![
                text(format!("{}:", name))
                    .font(Font {
                        weight: Weight::Semibold,
                        ..Font::default()
                    })
                    .width(Length::FillPortion(3)),
                column![value.into()].width(Length::FillPortion(7))
            ]
        }
        fn editor_text_row<'a>(
            field: GameTextField,
            name: &'a str,
            curr: &'a str,
            placeholder: &'a str,
        ) -> Row<'a, Message> {
            editor_row(
                name,
                text_input(placeholder, curr)
                    .on_input(move |txt| Message::EditorTextChanged(field, txt)),
            )
        }
        fn editor_enum_row<'a, TEnum, TMapper>(
            name: &'a str,
            curr: TEnum,
            mapper: TMapper,
        ) -> Row<'a, Message>
        where
            TEnum: Copy + EditableEnum,
            TMapper: Fn(TEnum) -> Message + 'a,
        {
            editor_row(name, pick_list(TEnum::ALL, Some(curr), mapper))
        }
        fn editor_btn(
            text_content: &'static str,
            bytes: &'static [u8],
        ) -> Button<'static, Message> {
            button(
                row![
                    Svg::new(Handle::from_memory(bytes)).width(Length::FillPortion(1)),
                    text(text_content).width(Length::FillPortion(9)),
                ]
                .spacing(10.0),
            )
        }
        column![
            row![
                editor_btn(
                    "Close",
                    include_bytes!("../icons/tabler--arrow-back.svg").as_slice()
                )
                .on_press(Message::CloseEditor),
                editor_btn(
                    "Save",
                    include_bytes!("../icons/tabler--device-floppy.svg").as_slice()
                )
                .style(button::success)
                .on_press(Message::SaveEditor),
            ]
            .padding(6)
            .spacing(20.0),
            editor_text_row(
                GameTextField::Name,
                "Name",
                game.name.as_str(),
                "Enter name"
            ),
            editor_text_row(
                GameTextField::Description,
                "Description",
                game.description.as_str(),
                "Enter description"
            ),
            editor_text_row(
                GameTextField::IconUrl,
                "Icon URL",
                game.icon_url.as_ref().map(|v| v.as_str()).unwrap_or(""),
                "Enter icon URL"
            ),
            editor_text_row(
                GameTextField::CoverUrl,
                "Cover URL",
                game.cover_url.as_ref().map(|v| v.as_str()).unwrap_or(""),
                "Enter hero URL"
            ),
            editor_text_row(
                GameTextField::HeaderUrl,
                "Header URL",
                game.header_url.as_ref().map(|v| v.as_str()).unwrap_or(""),
                "Enter header URL"
            ),
            editor_enum_row(
                "Completion Status",
                game.completion_status,
                Message::EditorCompletionStatusChanged
            ),
            number_input("Enter ID", game.id.clone())
                .map(|v| Message::SearchChanged(format!("{:?}", v)))
        ]
    }

    fn game_details<'a>(&'a self, curr: &'a GameData) -> Column<'a, Message> {
        fn detail_row<'b>(
            name: &'static str,
            content: impl Into<Element<'b, Message>>,
        ) -> Row<'b, Message> {
            row![
                text(name)
                    .font(Font {
                        weight: Weight::Semibold,
                        ..Font::default()
                    })
                    .width(Length::FillPortion(3)),
                container(content.into()).width(Length::FillPortion(7)),
            ]
        }
        fn detail_row_text<'b>(name: &'static str, content: String) -> Row<'b, Message> {
            detail_row(name, text(content))
        }
        let actions = get_actions(curr.install_status);
        let last_played = curr
            .last_played
            .map(|t| t.to_string())
            .unwrap_or("None".into());
        column![
            Row::with_children(actions.into_iter().map(|ga| {
                button(
                    row![
                        Svg::new(Handle::from_memory(ga.icon)).width(Length::FillPortion(1)),
                        text(ga.name).width(Length::FillPortion(4)),
                    ]
                    .align_y(Vertical::Center)
                    .spacing(4),
                )
                .style(|theme, status| ga.color.button_style::<Message>(theme, status))
                .on_press(Message::GameAction(ga.kind, curr.clone()))
                .into()
            }))
            .height(30)
            .spacing(2),
            text(&curr.name),
            text(&curr.description),
            detail_row_text("ID", curr.id.to_string()),
            detail_row_text("Last Played", last_played),
            detail_row_text("Install Status", curr.install_status.to_string()),
            detail_row_text("Completion Status", curr.completion_status.to_string()),
            detail_row_text("Playtime", curr.play_time.to_string()),
            detail_row_text(
                "Release Date",
                curr.release_date
                    .map(|v| v.format("%Y-%m-%d").to_string())
                    .unwrap_or("None".into())
            ),
            detail_row(
                "Genres",
                Row::with_children(curr.genres.iter().map(|g| text(g.name.trim_end()).into()))
                    .spacing(2),
            ),
        ]
    }
    pub fn view(&self) -> Element<Message> {
        let curr: Option<&GameData> = self.games.as_slice().get(self.curr_index);

        let items: Scrollable<Message> = scrollable(match self.view_type {
            LibraryViewType::Table => self.table.view().map(Message::Table),

            LibraryViewType::List => scrollable(column(
                self.games
                    .iter()
                    .enumerate()
                    .map(|(index, game)| {
                        let raw_icon_url = game.icon_url.as_ref().map(String::as_str).unwrap_or("");
                        let icon_url: Option<Url> = if raw_icon_url.is_empty() {
                            None
                        } else {
                            Some(Url::parse(raw_icon_url).unwrap())
                        };
                        let icon_path = if let Some(url) = icon_url.as_ref() {
                            let mut raw = urlencoding::decode(url.path()).unwrap();
                            if cfg!(windows) {
                                let my_path: String = (&raw[1..]).into();
                                raw = my_path.into();
                            }
                            raw
                        } else {
                            "".into()
                        };

                        (
                            game,
                            Element::from(
                                button(
                                    row![
                                        image(icon_path.as_ref()).width(32),
                                        text(&game.name).width(Fill),
                                        Svg::new(Self::auto_installer_icon(game.install_status))
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
            .into(),
            LibraryViewType::Grid => text("TODO: GRID").into(),
        });
        let toolbar = self.toolbar();

        let raw_side_content = if let Some(game) = self.edit_game.as_ref() {
            Some(Self::editor(game))
        } else if let Some(curr) = curr {
            Some(self.game_details(curr))
        } else {
            None
        };

        let mut wrapped_items: Element<Message> = if let Some(side) = raw_side_content {
            row![
                items.width(Length::FillPortion(3)),
                scrollable(side).width(Length::FillPortion(7)),
            ]
            .into()
        } else {
            items.into()
        };
        if self.display_filter {
            wrapped_items = row![self.filter_view(), wrapped_items].into();
        }
        column![toolbar, wrapped_items].into()
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
                self.args.search = query;
                return self.update(Message::ReloadCache);
            }
            Message::ReloadCache => {
                return Task::perform(
                    db::ops::get_games(self.args.clone(), self.filter.clone()),
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
                addon.launch(game.get_ref());
                let game_id = game.id;
                return Task::perform(db::ops::update_game_played(game.id), move |dt| {
                    Message::UpdateLastPlayed(game_id, dt)
                });
                /*
                let settings = settings::load().unwrap();
                match settings.general.post_launch_action {
                    PostLaunchAction::DoNothing => {}
                    PostLaunchAction::Exit => {
                        return window::get_oldest().and_then(window::close);
                    }
                    PostLaunchAction::Minimize => {
                        return window::get_oldest().and_then(|w| window::minimize(w, true));
                    }
                }

                 */
            }
            Message::GameAction(GameAction::Install, game) => {
                let addon = ADDONS
                    .get_game_library(&game.library_type)
                    .cloned()
                    .expect("Failed to load library");
                addon.install(game.get_ref());
            }
            Message::GameAction(GameAction::Uninstall, game) => {
                let addon = ADDONS
                    .get_game_library(&game.library_type)
                    .cloned()
                    .expect("Failed to load library");
                addon.uninstall(game.get_ref());
            }
            Message::GameAction(GameAction::Edit, game) => {
                self.edit_game = Some(game);
            }
            Message::SaveEditor => {
                if let Some(game) = self.edit_game.clone() {
                    return Task::perform(db::ops::update_game(game), |_| Message::ReloadCache);
                }
            }
            Message::CloseEditor => {
                self.edit_game = None;
            }
            Message::EditorTextChanged(field, value) => {
                fn map_opt_empty(opt: String) -> Option<String> {
                    if opt.is_empty() {
                        None
                    } else {
                        Some(opt)
                    }
                }
                if let Some(edit_game) = self.edit_game.as_mut() {
                    match field {
                        GameTextField::Name => edit_game.name = value,
                        GameTextField::Description => edit_game.description = value,
                        GameTextField::HeaderUrl => edit_game.header_url = map_opt_empty(value),
                        GameTextField::IconUrl => edit_game.icon_url = map_opt_empty(value),
                        GameTextField::CoverUrl => edit_game.cover_url = map_opt_empty(value),
                    }
                }
            }
            Message::EditorCompletionStatusChanged(status) => {
                if let Some(edit_game) = self.edit_game.as_mut() {
                    edit_game.completion_status = status;
                }
            }
            Message::SelectGame(index) => {
                self.curr_index = index;
            }
            Message::SortFieldChanged(field) => {
                self.args.sort.field = field;
                return self.update(Message::ReloadCache);
            }
            Message::ToggleSortDirection => {
                self.args.sort.order = match self.args.sort.order {
                    SortOrder::Ascending => SortOrder::Descending,
                    SortOrder::Descending => SortOrder::Ascending,
                };
                return self.update(Message::ReloadCache);
            }
            Message::MoveInDir(direction) => {
                match self.view_type {
                    LibraryViewType::Table | LibraryViewType::List => {
                        // basic vertical
                        match direction {
                            Direction::Up if self.curr_index != 0 => self.curr_index -= 1,
                            Direction::Down if self.curr_index + 1 < self.games.len() => {
                                self.curr_index += 1
                            }
                            _ => {}
                        }
                    }
                    LibraryViewType::Grid => todo!(),
                }
            }
            Message::UpdateLastPlayed(game_id, dt) => {
                log::info!(
                    "Process game details update raw: {:?}; id: {:?}",
                    game_id,
                    dt,
                );
                if let Some(ref mut data) = self.games.as_mut_slice().get_mut(self.curr_index) {
                    log::info!("Process game details update: {:?}; id: {:?}", data, game_id);
                    if data.id == game_id {
                        data.last_played = Some(dt);
                    }
                }
            }
            Message::ToggleFilterDisplay => {
                self.display_filter = !self.display_filter;
                if self.display_filter {
                    return Task::perform(ops::get_genres(), Message::OnGenresLoad);
                }
            }
            Message::OnGenresLoad(genres) => {
                self.all_genres_by_meta_id = genres
                    .into_iter()
                    .map(move |g| (g.metadata_id.clone(), g))
                    .collect()
            }
            Message::Filter(filter) => {
                update_filter(&mut self.filter, filter);
                return self.update(Message::ReloadCache);
            }
            v => println!("{:?}", v),
        }

        Task::none()
    }
}

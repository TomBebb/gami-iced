use gami_backend::db;
use gami_sdk::{GameData, GameInstallStatus};
use iced::advanced::svg::Handle;
use iced::alignment::Vertical;
use iced::font::Weight;
use iced::widget::{
    button, column, combo_box, container, image, responsive, row, scrollable, text, Button,
    Container, Svg,
};
use iced::{ContentFit, Element, Fill, Font, Length, Renderer, Task, Theme};
use iced_aw::ContextMenu;
use iced_table::table;
use std::fmt::{self};
use std::time::{Duration, SystemTime};
use url::Url;

#[derive(Copy, Clone, Debug)]
enum ColumnKind {
    Name,
    LastPlayed,
    Playtime,
    Description,
}

impl fmt::Display for ColumnKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ColumnKind::Name => "Name",
            ColumnKind::LastPlayed => "Last Played",
            ColumnKind::Playtime => "Time Played",
            ColumnKind::Description => "Description",
        })
    }
}
#[derive(Debug, Default, Clone)]
struct Row {
    name: String,
    description: String,
    play_time: Duration,
    install_status: GameInstallStatus,
    release_date: Option<SystemTime>,
    last_played: Option<SystemTime>,
}

impl From<GameData> for Row {
    fn from(value: GameData) -> Self {
        Self {
            name: value.name,
            description: value.description,
            install_status: value.install_status,
            play_time: value.play_time,
            release_date: value.release_date,
            last_played: value.last_played,
        }
    }
}

#[derive(Clone, Debug)]
struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        Self {
            kind,
            width: 50.,
            resize_offset: None,
        }
    }
}

impl<'a, Message> table::Column<'a, Message, Theme, Renderer> for Column
where
    Message: 'a,
{
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message, Theme, Renderer> {
        container(text(self.kind.to_string()).font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        }))
        .center_y(24)
        .into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, Message, Theme, Renderer> {
        let content: Element<_> = match self.kind {
            ColumnKind::Description => text(&row.description).into(),
            ColumnKind::Name => text(&row.name).into(),
            ColumnKind::LastPlayed => {
                /*
                let d= row.last_played;

                text(d.map(|t| format("{}", t.duration_since(earlier)))).into()
                */
                text("TODO").into()
            }
            ColumnKind::Playtime => text(row.play_time.as_secs()).into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
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
    curr_index: usize,
    view_types: combo_box::State<LibraryViewType>,
    view_type: LibraryViewType,
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    games: Vec<GameData>,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
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
impl LibraryPage {
    pub fn new() -> Self {
        let me = Self {
            view_types: combo_box::State::new(LibraryViewType::ALL.to_vec()),
            view_type: LibraryViewType::List,
            games: Vec::new(),
            curr_index: 0,
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
            rows: Vec::new(),
            columns: vec![
                Column::new(ColumnKind::Name),
                Column::new(ColumnKind::Playtime),
                Column::new(ColumnKind::Description),
            ],
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
        let items: Element<Message> = match self.view_type {
            LibraryViewType::Table => responsive(|size| {
                table(
                    self.header.clone(),
                    self.body.clone(),
                    &self.columns,
                    &self.rows,
                    Message::SyncHeader,
                )
                .min_width(size.width)
                .into()
            })
            .into(),

            LibraryViewType::List => scrollable(column(
                self.games
                    .iter()
                    .enumerate()
                    .map(|(index, game)| {
                        (
                            game,
                            Element::from(
                                Button::new(
                                    row![
                                        text(&game.name).width(Fill),
                                        image(
                                            Url::parse(
                                                &game
                                                    .icon_url
                                                    .as_ref()
                                                    .map(String::as_str)
                                                    .unwrap_or("")
                                            )
                                            .unwrap()
                                            .path()
                                        )
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
                .style(button::success)
                .width(30)
                .on_press(Message::RefreshGames)
            ]
            .spacing(3)
            .align_y(Vertical::Center),
        );
        column![toolbar, items].into()
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
            Message::CacheReloaded(cache) => {
                self.rows = cache.iter().cloned().map(Row::from).collect::<Vec<Row>>();
                self.games = cache;
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

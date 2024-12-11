use iced::advanced::svg::Handle;
use iced::alignment::Vertical;
use iced::widget::{button, column, combo_box, row, scrollable, svg, text, Container, Svg};
use iced::{ContentFit, Element, Fill, Theme};
use iced_aw::ContextMenu;
use std::fmt;

#[derive(Clone, Debug, Copy)]
pub enum InstallStatus {
    Installed,
    InLibrary,
    InProgress,
}
#[derive(Clone, Debug)]
pub struct LibraryGame {
    install_status: InstallStatus,
    name: String,
    library_id: String,
    library_type: String,
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
    view_types: combo_box::State<LibraryViewType>,
    view_type: LibraryViewType,
    games: Vec<LibraryGame>,
}

impl Default for LibraryPage {
    fn default() -> Self {
        Self {
            view_types: combo_box::State::new(LibraryViewType::ALL.to_vec()),
            view_type: LibraryViewType::List,
            games: vec![LibraryGame {
                install_status: InstallStatus::Installed,
                library_type: "steam".into(),
                name: "Atelier Sophie The Alchemist of the Mysterious Book DX".into(),
                library_id: "1502970".into(),
            }],
        }
    }
}
#[derive(Debug, Clone)]
pub enum Message {
    ViewSelected(LibraryViewType),
    ShowAddDialog,
    GameAction(GameActionKind, LibraryGame),
}
#[derive(Debug, Clone, Copy)]
enum GameActionKind {
    Play,
    Install,
    Uninstall,
    Delete,
    Edit,
}
#[derive(Debug, Clone, Copy)]
struct GameAction {
    name: &'static str,
    icon: &'static [u8],
    kind: GameActionKind,
}
const PLAY_ACTION: GameAction = GameAction {
    name: "Play",
    icon: include_bytes!("../icons/tabler--play.svg"),
    kind: GameActionKind::Play,
};
const INSTALL_ACTION: GameAction = GameAction {
    name: "Install",
    icon: include_bytes!("../icons/tabler--plus.svg"),
    kind: GameActionKind::Install,
};
const UNINSTALL_ACTION: GameAction = GameAction {
    name: "Uninstall",
    icon: include_bytes!("../icons/tabler--minus.svg"),
    kind: GameActionKind::Uninstall,
};
const DELETE_ACTION: GameAction = GameAction {
    name: "Delete",
    icon: include_bytes!("../icons/tabler--x.svg"),
    kind: GameActionKind::Delete,
};
const EDIT_ACTION: GameAction = GameAction {
    name: "Edit",
    icon: include_bytes!("../icons/tabler--edit.svg"),
    kind: GameActionKind::Edit,
};
const fn get_actions(status: InstallStatus) -> &'static [GameAction] {
    match status {
        InstallStatus::Installed => &[PLAY_ACTION, EDIT_ACTION, DELETE_ACTION],
        _ => &[INSTALL_ACTION, EDIT_ACTION, DELETE_ACTION],
    }
}
impl LibraryPage {
    fn game_menu<'a>(
        &'a self,
        game: &'a LibraryGame,
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
            .width(100)
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
                    Svg::new(svg::Handle::from_memory(include_bytes!(
                        "../icons/tabler--plus.svg"
                    )))
                    .content_fit(ContentFit::Contain)
                )
                .width(30)
                .on_press(Message::ShowAddDialog)
            ]
            .spacing(3)
            .align_y(Vertical::Center),
        );
        column![toolbar, scrollable(column(items))].into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ViewSelected(view_type) => {
                self.view_type = view_type;
            }
            v => println!("{:?}", v),
        }
    }
}

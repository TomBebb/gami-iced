use iced::alignment::Vertical;
use iced::widget::{button, column, combo_box, row, scrollable, svg, text, Svg};
use iced::{ContentFit, Element};
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
    PlayGame(LibraryGame),
    InstallGame(LibraryGame),
}

impl LibraryPage {
    fn game_menu<'a>(
        &'a self,
        game: &'a LibraryGame,
        underlay: Element<'a, Message>,
    ) -> Element<'a, Message> {
        ContextMenu::new(underlay, || {
            column(vec![
                iced::widget::button("Play")
                    .on_press_with(|| Message::PlayGame(game.clone()))
                    .into(),
                iced::widget::button("Install")
                    .on_press_with(|| Message::InstallGame(game.clone()))
                    .into(),
            ])
            .into()
        })
        .into()
    }
    pub fn view(&self) -> Element<Message> {
        let mut items: Vec<Element<Message>> = Vec::new();
        for game in &self.games {
            let raw = text(&game.name);

            items.push(self.game_menu(game, raw.into()).into());
        }

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

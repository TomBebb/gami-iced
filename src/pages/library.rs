use iced::alignment::Vertical;
use iced::widget::{combo_box, row};
use iced::Element;
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
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
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
}

impl Default for LibraryPage {
    fn default() -> Self {
        Self {
            view_types: combo_box::State::new(LibraryViewType::ALL.to_vec()),
            view_type: LibraryViewType::List,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    ViewSelected(LibraryViewType),
}

impl LibraryPage {
    pub fn view(&self) -> Element<Message> {
        row![combo_box(
            &self.view_types,
            "Pick a view type",
            Some(&self.view_type),
            Message::ViewSelected,
        )]
        .align_y(Vertical::Center)
        .into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ViewSelected(view_type) => {
                self.view_type = view_type;
            }
        }
    }
}

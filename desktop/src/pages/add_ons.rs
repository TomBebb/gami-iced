use gami_backend::ADDONS;
use gami_sdk::PluginMetadata;
use iced::font::Weight;
use iced::widget::{button, column, row, scrollable, text};
use iced::{Element, Font, Length};

#[derive(Clone, Debug)]
pub struct AddOns {
    metadatas: Vec<PluginMetadata>,
    selected: usize,
}
impl AddOns {
    pub fn new() -> Self {
        Self {
            metadatas: ADDONS.get_addon_metadatas().to_vec(),
            selected: 0,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum AddOnMessage {
    Selected(usize),
}

impl AddOns {
    pub fn view(&self) -> Element<AddOnMessage> {
        row![
            scrollable(column(self.metadatas.iter().enumerate().map(
                |(index, m)| {
                    button(m.name.trim())
                        .on_press_maybe(if index == self.selected {
                            None
                        } else {
                            Some(AddOnMessage::Selected(index))
                        })
                        .into()
                }
            ))),
            column![text("Settings").font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),]
            .padding(10)
            .width(Length::Fill)
        ]
        .into()
    }
    pub fn update(&mut self, message: AddOnMessage) {
        match message {
            AddOnMessage::Selected(index) => self.selected = index,
        }
    }
}

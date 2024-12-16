use gami_backend::ADDONS;
use gami_sdk::{ConfigSchemaMetadata, PluginMetadata};
use iced::font::Weight;
use iced::widget::{button, column, row, scrollable, text, Column};
use iced::{Element, Font, Length};
use std::collections::HashMap;

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
        let curr: &HashMap<String, ConfigSchemaMetadata> = &self.metadatas[self.selected].configs;
        println!(
            "{:?}; all: {:?}",
            &self.metadatas[self.selected], self.metadatas
        );
        let items: Element<AddOnMessage> = Column::with_children(curr.into_iter().map(|(k, v)| {
            text(v.name.trim_start())
                .font(Font {
                    weight: Weight::Semibold,
                    ..Font::default()
                })
                .into()
        }))
        .into();
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
            column![
                text("Settings").font(Font {
                    weight: Weight::Bold,
                    ..Font::default()
                }),
                items
            ]
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

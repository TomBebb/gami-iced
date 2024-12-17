use gami_backend::ADDONS;
use gami_sdk::{load_schema, ConfigsSchema, PluginMetadata};
use iced::font::Weight;
use iced::widget::{button, column, row, scrollable, text, text_input, Column};
use iced::{Alignment, Element, Font, Length};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AddOns {
    metadatas: Vec<PluginMetadata>,
    selected: usize,
    curr: ConfigsSchema,
}
impl AddOns {
    pub fn new() -> Self {
        let metadatas = ADDONS.get_addon_metadatas().to_vec();
        let curr = if metadatas.len() == 0 {
            HashMap::new()
        } else {
            load_schema(&metadatas[0].id)
        };
        Self {
            metadatas,
            selected: 0,
            curr,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AddOnMessage {
    Selected(usize),
    InputChanged(String, String),
}
impl AddOns {
    pub fn view(&self) -> Element<AddOnMessage> {
        let items: Element<AddOnMessage> = Column::with_children(self.curr.iter().map(|(k, v)| {
            row![
                text(v.name.trim_start().to_owned())
                    .font(Font {
                        weight: Weight::Semibold,
                        ..Font::default()
                    })
                    .align_x(Alignment::End)
                    .width(Length::FillPortion(1)),
                text_input("Enter value", "default")
                    .on_input(move |v| AddOnMessage::InputChanged(k.clone(), v))
                    .width(Length::FillPortion(2))
            ]
            .spacing(10)
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
            AddOnMessage::Selected(index) => {
                self.selected = index;
                self.curr = load_schema(self.metadatas[self.selected].id.trim_end());
            }
            AddOnMessage::InputChanged(key, value) => {
                println!("{} => {}", key, value);
            }
        }
    }
}

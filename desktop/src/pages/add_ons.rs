use gami_backend::ADDONS;
use gami_sdk::{load_schema, resolve_addon_config_json_path, ConfigsSchema, PluginMetadata};
use iced::font::Weight;
use iced::widget::{button, column, row, scrollable, text, text_input, Column};
use iced::{Alignment, Element, Font, Length, Task};
use std::collections::HashMap;
use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

type Config = HashMap<String, String>;
fn get_json(id: &str) -> Config {
    let path = resolve_addon_config_json_path(id);
    if !path.exists() {
        Config::new()
    } else {
        serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap()
    }
}
async fn write_json(id: String, config: Arc<Mutex<Config>>) {
    tokio::task::spawn_blocking(move || {
        let config = config.lock().unwrap();
        let file = File::create(resolve_addon_config_json_path(&id)).unwrap();
        serde_json::to_writer(file, config.deref()).unwrap();
    });
}
#[derive(Clone, Debug)]
pub struct AddOns {
    metadatas: Vec<PluginMetadata>,
    selected: usize,
    curr: ConfigsSchema,
    curr_config: Arc<Mutex<Config>>,
}
impl AddOns {
    pub fn new() -> Self {
        let metadatas = ADDONS.get_addon_metadatas().to_vec();
        let curr_config = metadatas
            .iter()
            .map(|v| v.id)
            .map(get_json)
            .next()
            .unwrap_or_default();
        let curr = if metadatas.len() == 0 {
            HashMap::new()
        } else {
            load_schema(&metadatas[0].id)
        };
        Self {
            metadatas,
            selected: 0,
            curr,
            curr_config: Arc::new(Mutex::new(curr_config)),
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
        let curr_config = self.curr_config.lock().unwrap();
        let items: Element<AddOnMessage> = Column::with_children(self.curr.iter().map(|(k, v)| {
            row![
                text(v.name.to_owned())
                    .font(Font {
                        weight: Weight::Semibold,
                        ..Font::default()
                    })
                    .align_x(Alignment::End)
                    .width(Length::FillPortion(1)),
                text_input("Enter value", curr_config.get(k).unwrap_or(&"".to_owned()))
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
                    button(&m.name)
                        .on_press_maybe(if index == self.selected {
                            None
                        } else {
                            Some(AddOnMessage::Selected(index))
                        })
                        .width(Length::Fill)
                        .into()
                }
            )))
            .width(Length::FillPortion(1)),
            column![
                text("Settings").font(Font {
                    weight: Weight::Bold,
                    ..Font::default()
                }),
                items
            ]
            .width(Length::FillPortion(5)),
        ]
        .spacing(5)
        .into()
    }
    pub fn update(&mut self, message: AddOnMessage) -> Task<AddOnMessage> {
        let id = self.metadatas[self.selected].id;
        let curr_config = self.curr_config.clone();
        let selected = self.selected;
        match message {
            AddOnMessage::Selected(index) => {
                self.selected = index;
                self.curr = load_schema(id);

                *self.curr_config.lock().unwrap() = get_json(id);
            }
            AddOnMessage::InputChanged(key, value) => {
                self.curr_config.lock().unwrap().insert(key, value);
                return Task::perform(write_json(id.into(), curr_config.clone()), move |_| {
                    AddOnMessage::Selected(selected)
                });
            }
        }
        Task::none()
    }
}

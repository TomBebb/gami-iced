use gami_backend::ADDONS;
use gami_sdk::PluginMetadata;
use iced::widget::{button, column, scrollable, Column};

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
    pub fn view(&self) -> Column<AddOnMessage> {
        column![scrollable(column(self.metadatas.iter().enumerate().map(
            |(index, m)| {
                button(m.name.trim())
                    .on_press_maybe(if index == self.selected {
                        None
                    } else {
                        Some(AddOnMessage::Selected(index))
                    })
                    .into()
            }
        )))]
    }
    pub fn update(&mut self, message: AddOnMessage) {
        match message {
            AddOnMessage::Selected(index) => self.selected = index,
        }
    }
}

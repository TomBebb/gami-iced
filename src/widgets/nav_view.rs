use iced::widget::{button, Column};
use iced::{Element, Fill};

#[derive(Copy, Clone, Debug)]
pub enum Message {
    NavSelected(usize),
}
const PAGE_NAMES: &'static [&'static str] = &["Counter", "Library", "Settings"];
#[derive(Debug, Clone, Copy, Default)]
pub struct NavView {
    pub active_item: usize,
}
impl NavView {
    pub fn view(&self) -> Column<Message> {
        Column::with_children(PAGE_NAMES.into_iter().enumerate().map(|(index, &name)| {
            Element::from(
                button(name)
                    .width(Fill)
                    .on_press_maybe(if self.active_item == index {
                        None
                    } else {
                        Some(Message::NavSelected(index))
                    }),
            )
        }))
        .width(100)
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavSelected(v) => {
                self.active_item = v;
            }
        }
    }
}

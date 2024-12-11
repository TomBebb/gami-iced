use iced::widget::{button, column, Column};
use iced::Fill;

#[derive(Copy, Clone, Debug)]
pub enum Message {
    NavSelected(usize),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NavView {
    pub active_item: usize,
}
impl NavView {
    pub fn view(&self) -> Column<Message> {
        column![
            button("Counter demo")
                .on_press(Message::NavSelected(0))
                .width(Fill),
            button("Library")
                .on_press(Message::NavSelected(1))
                .width(Fill),
            button("Settings")
                .on_press(Message::NavSelected(2))
                .width(Fill),
        ]
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

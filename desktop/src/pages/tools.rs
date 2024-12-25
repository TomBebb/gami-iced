use gami_backend::db;
use iced::widget::{button, column};
use iced::{Element, Task};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ClearDatabase,
    NoOp,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ToolsPage;

impl ToolsPage {
    pub fn view(&self) -> Element<Message> {
        column![button("Clear Data").on_press(Message::ClearDatabase)].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoOp => Task::none(),
            Message::ClearDatabase => Task::future(db::ops::clear_all()).map(|_| Message::NoOp),
        }
    }
}

use chrono::{DateTime, Local};
use iced::widget::{row, text};
use iced::Element;
#[derive(Debug, Clone)]
pub struct Header {
    curr_time: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub enum Message {}
impl Header {
    pub fn new() -> Self {
        Self {
            curr_time: Local::now(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        row![text(self.curr_time.to_string())].into()
    }
    pub fn update(&mut self, _message: Message) {}
}

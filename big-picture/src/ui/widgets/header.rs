use iced::widget::{row, text};
use iced::Element;

#[derive(Default, Debug, Clone)]
pub struct Header {}

#[derive(Debug, Clone)]
pub enum Message {}
impl Header {
    pub fn view(&self) -> Element<Message> {
        row![text("Demo")].into()
    }
    pub fn update(&mut self, _message: Message) {
    }
}

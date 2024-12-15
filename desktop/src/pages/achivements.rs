use iced::widget::text;
use iced::Element;

// TODO
#[derive(Default, Clone, Debug)]
pub struct Achievements;
#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Achievements {
    pub fn view(&self) -> Element<Message> {
        text("Achievements: TODO").into()
    }
    pub fn update(&mut self, message: Message) {}
}

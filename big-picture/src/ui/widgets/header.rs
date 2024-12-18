use chrono::{DateTime, Local, TimeDelta};
use iced::time::Instant;
use iced::widget::{row, text};
use iced::Element;
#[derive(Debug, Clone)]
pub struct Header {
    start: DateTime<Local>,
    epoch: Instant,
    curr_time: Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateTime(Instant),
}
impl Header {
    pub fn new() -> Self {
        Self {
            start: Local::now(),
            epoch: Instant::now(),
            curr_time: Instant::now(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        let curr = self.start + TimeDelta::from_std(self.curr_time - self.epoch).unwrap();
        row![text(curr.time().to_string())].into()
    }
    pub fn update(&mut self, message: Message) {
        log::info!("Updating {:?}", message);
        match message {
            Message::UpdateTime(curr_time) => {
                self.curr_time = curr_time;
            }
        }
    }
}

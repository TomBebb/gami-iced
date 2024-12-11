#[derive(Default, Clone, Debug)]
pub struct Counter {
    value: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    Increment,
    Decrement,
}
use iced::alignment::Vertical;
use iced::widget::{button, row, text, Row};

impl Counter {
    pub fn view(&self) -> Row<CounterMessage> {
        row![
            button("+").on_press(CounterMessage::Increment),
            text(self.value).size(50),
            button("-").on_press(CounterMessage::Decrement),
        ]
        .align_y(Vertical::Center)
    }
    pub fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => {
                self.value += 1;
            }
            CounterMessage::Decrement => {
                self.value -= 1;
            }
        }
    }
}

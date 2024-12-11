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
        // We use a column: a simple vertical layout
        row![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(CounterMessage::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
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

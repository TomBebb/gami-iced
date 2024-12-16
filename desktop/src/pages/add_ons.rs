use iced::alignment::Vertical;
use iced::widget::{button, row, text, Row};
#[derive(Default, Clone, Debug)]
pub struct AddOns {
    value: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum AddOnMessage {
    Increment,
    Decrement,
}

impl AddOns {
    pub fn view(&self) -> Row<AddOnMessage> {
        row![
            button("+")
                .style(button::success)
                .on_press(AddOnMessage::Increment),
            text(self.value).size(50),
            button("-")
                .style(button::danger)
                .on_press(AddOnMessage::Decrement),
        ]
        .align_y(Vertical::Center)
    }
    pub fn update(&mut self, message: AddOnMessage) {
        match message {
            AddOnMessage::Increment => {
                self.value += 1;
            }
            AddOnMessage::Decrement => {
                self.value -= 1;
            }
        }
    }
}

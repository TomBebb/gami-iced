use html5gum::{Token, Tokenizer};
use iced::widget::{text, Column, Row};
use iced::{Element, Length, Renderer, Theme};

pub fn draw_html<'a, TMessage: 'a>(html: &'a str) -> Element<'a, TMessage> {
    let mut col = Column::with_capacity(4);
    let mut row: Row<'a, TMessage, Theme, Renderer> = Row::with_capacity(4);
    for Ok(token) in Tokenizer::new(html) {
        match token {
            Token::StartTag(tag) => match tag.name.as_ref() {
                b"br" => {
                    col = col.push(row.width(Length::FillPortion(1)));
                    row = Row::with_capacity(4);
                }
                _ => row = row.push(text!("<{}>", String::from_utf8_lossy(&tag.name))),
            },
            Token::String(raw_text) => {
                let mapped = String::from_utf8(raw_text.0).unwrap();
                row = row.push(text(mapped));
            }
            Token::EndTag(tag) => {
                row = row.push(text!("</{}>", String::from_utf8_lossy(&tag.name)));
            }
            _ => panic!("unexpected input"),
        }
    }
    col = col.push(row.width(Length::FillPortion(1)));
    col.into()
}

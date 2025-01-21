use html5gum::{Token, Tokenizer};
use iced::widget::{text, Column, Text};
use iced::Element;
use std::fmt::Write;

pub fn draw_html<'a, TMessage>(html: &'a str) -> Element<'a, TMessage> {
    let mut new_html = String::new();
    for Ok(token) in Tokenizer::new(html) {
        match token {
            Token::StartTag(tag) => {
                write!(new_html, "<{}>", String::from_utf8_lossy(&tag.name)).unwrap();
            }
            Token::String(hello_world) => {
                write!(new_html, "{}", String::from_utf8_lossy(&hello_world)).unwrap();
            }
            Token::EndTag(tag) => {
                write!(new_html, "</{}>", String::from_utf8_lossy(&tag.name)).unwrap();
            }
            _ => panic!("unexpected input"),
        }
    }
    text(new_html).into()
}

use html5gum::{Token, Tokenizer};
use iced::widget::{text, Column, Row};
use iced::{Element, Length, Renderer, Theme};

pub fn draw_html<'a, TMessage: 'a>(html: &'a str) -> Element<'a, TMessage> {
    let mut col = Column::with_capacity(4);
    let mut row: Row<'a, TMessage, Theme, Renderer> = Row::with_capacity(4);
    let mut nested_by_tags = Vec::<Box<str>>::new();
    for Ok(token) in Tokenizer::new(html) {
        match token {
            Token::StartTag(tag) => match tag.name.as_ref() {
                b"br" => {
                    col = col.push(row.width(Length::FillPortion(1)));
                    row = Row::with_capacity(4);
                }
                _ => nested_by_tags
                    .push(unsafe { std::str::from_utf8_unchecked(&tag.name.0) }.into()),
            },
            Token::String(raw_text) => {
                let curr_tag = nested_by_tags.last_mut();
                if let Some(curr_tag) = curr_tag {
                    println!("Tag: {:?}", curr_tag);
                }
                let mapped = String::from_utf8(raw_text.0).unwrap();
                row = row.push(text(mapped));
            }
            Token::EndTag(tag) => {
                nested_by_tags.pop();
            }
            _ => panic!("unexpected input"),
        }
    }
    col = col.push(row.width(Length::FillPortion(1)));
    col.into()
}

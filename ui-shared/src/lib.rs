use iced::widget::{text, Column};
use iced::Element;
use tl::{Node, Parser, ParserOptions};

fn from_html<'a, 'b, TMessage>(parser: &'a Parser, node: &'a Node<'a>) -> Element<'b, TMessage> {
    match node.as_tag() {
        Some(v) => text(format!("HTML tag: {}", v.inner_text(parser))).into(),
        None => text("Missing tag").into(),
    }
}
pub fn show_html<'a, TMessage: 'a>(html: String) -> Element<'a, TMessage> {
    let dom = tl::parse(&html, ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let mut items = Column::with_capacity(dom.children().len());
    for child in dom.children() {
        let node = child.get(parser).expect("Invalid html node");
        items = items.push(from_html(parser, node));
    }
    items.into()
}

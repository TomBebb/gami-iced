use iced::widget::{text, Column};
use iced::Element;
use tl::{NodeHandle, Parser, ParserOptions};

fn from_html<'a, 'b, TMessage>(parser: &'a Parser, node_handle: &'a NodeHandle) -> Element<'b, TMessage> {
    let node = node_handle.get(parser).expect("Invalid HTML parse");
    match node.as_tag() {
        Some(v) => text(format!("HTML tag: {}", v.inner_text(parser))).into(),
        None => text("Missing tag").into(),
    }
}
pub fn show_html<'a, TMessage: 'a>(html: &'a str) -> Element<'a, TMessage> {
    let dom = tl::parse(html, ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let mut items = Column::with_capacity(dom.children().len());
    for child in dom.children() {
        items = items.push(from_html(parser, child));
    }
    items.into()
}

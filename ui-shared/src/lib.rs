use iced::widget::{text, Column};
use iced::Element;
use tl::{Node, Parser, ParserOptions};

fn from_html<'a, 'b, TMessage:'b>(parser: &'a Parser, node: &'a Node) -> Element<'b, TMessage> {
    if let Some(ch) = node.children() {
        let children: Box<[&'a Node]> =  ch.all(parser).into_iter().collect();
        from_html_children(parser, &*children)
    } else {
        let raw = node.as_raw();
        text(format!("Tag: {:?}; raw: {:?}", node.as_tag(), raw)).into()
    }
}
fn from_html_children<'a, 'b, TMessage: 'b>(parser: &'a Parser, children: &'a [&'a Node]) -> Element<'b, TMessage> {
    let mut items = Column::with_capacity(children.len());
    for child in children {
        items = items.push(from_html(parser, child));
    }
    items.into()
}
pub fn show_html<'a, TMessage: 'a>(html: &'a str) -> Element<'a, TMessage> {
    let dom = tl::parse(html, ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let children: Box<[& Node]> =  dom.children().into_iter().map(|v| v.get(parser).expect("Invalid child")).collect();
    from_html_children(parser,&*children)
}

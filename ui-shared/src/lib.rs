use forest_ds::tree::Tree;
pub use html_parse::html::{Html, Node};
use iced::widget::{text, Column};
use iced::Element;

fn from_html<'a: 'b, 'b, TMessage: 'b>(node: &'a Node) -> Element<'b, TMessage> {
    match node {
        &Node::Element(ref element) => text!(
            "el: {:?}; attrs: {:?}; full : {:?}",
            element.name.local,
            element.attrs,
            element
        )
        .into(),
        &Node::Text(ref val) => text(val.text.chars().as_str()).into(),
        &Node::Document => text("document").into(),
        _ => text!("unsupported node type: {:?}", node).into(),
    }
}
fn from_html_children<'a: 'b, 'b, TMessage: 'b>(children: &'a [&'a Node]) -> Element<'b, TMessage> {
    let mut items = Column::with_capacity(children.len());
    for child in children {
        items = items.push(from_html(*child));
    }
    items.into()
}

pub fn show_dom_ref<'a, TMessage: 'a>(tree: &'a Tree<Node>) -> Element<'a, TMessage> {
    let mut items = Column::with_capacity(8);
    for child in tree {
        items = items.push(from_html(child));
    }
    items.into()
}
pub fn parse_html(text: &str) -> Html {
    Html::parse_document(text)
}

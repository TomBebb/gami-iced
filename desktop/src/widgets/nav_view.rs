use iced::widget::svg::Handle;
use iced::widget::tooltip::Position;
use iced::widget::{button, container, text, tooltip, Column, Svg};
use iced::{ContentFit, Element, Fill, Theme};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Message {
    NavSelected(usize),
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NavLocation {
    Top,
    Bottom,
}

struct PageInfo<'a> {
    icon: &'a [u8],
    name: &'a str,
    location: NavLocation,
}
const PAGES: &[PageInfo] = &[
    PageInfo {
        icon: include_bytes!("../icons/tabler--books.svg"),
        name: "Library",
        location: NavLocation::Top,
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--trophy.svg"),
        name: "Achievements",
        location: NavLocation::Top,
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--script.svg"),
        name: "Add-ons",
        location: NavLocation::Bottom,
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--tools.svg"),
        name: "Tools",
        location: NavLocation::Bottom,
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--settings.svg"),
        name: "Settings",
        location: NavLocation::Bottom,
    },
];
#[derive(Debug, Clone, Copy, Default)]
pub struct NavView {
    pub active_item: usize,
}
impl NavView {
    pub fn get_name(&self) -> String {
        PAGES[self.active_item].name.to_string()
    }
    pub fn view(&self) -> Column<Message> {
        let mut raw_page_items: HashMap<NavLocation, Vec<Element<Message>>> = HashMap::from_iter(
            [NavLocation::Top, NavLocation::Bottom]
                .into_iter()
                .map(|loc| {
                    (
                        loc,
                        PAGES
                            .into_iter()
                            .enumerate()
                            .filter(|(_, info)| info.location == loc)
                            .map(|(index, &PageInfo { name, icon, .. })| {
                                let svg: Svg<'static, Theme> = Svg::new(Handle::from_memory(icon));
                                Element::from(tooltip(
                                    button(svg.content_fit(ContentFit::Contain)).on_press_maybe(
                                        if self.active_item == index {
                                            None
                                        } else {
                                            Some(Message::NavSelected(index))
                                        },
                                    ),
                                    container(text(name))
                                        .padding(6)
                                        .style(container::rounded_box),
                                    Position::Right,
                                ))
                            })
                            .collect::<Vec<Element<Message>>>(),
                    )
                }),
        );
        Column::with_children(
            raw_page_items
                .remove(&NavLocation::Top)
                .unwrap()
                .into_iter()
                .chain([button("").height(Fill).width(Fill).into()].into_iter())
                .chain(
                    raw_page_items
                        .remove(&NavLocation::Bottom)
                        .unwrap()
                        .into_iter(),
                ),
        )
        .width(50)
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavSelected(v) => {
                self.active_item = v;
            }
        }
    }
}

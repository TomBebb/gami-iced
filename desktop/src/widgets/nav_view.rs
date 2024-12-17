use iced::widget::svg::Handle;
use iced::widget::{button, row, text, Column, Svg};
use iced::{Alignment, ContentFit, Element, Fill, Theme};
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
        icon: include_bytes!("../icons/fluent--library-24-regular.svg"),
        name: "Library",
        location: NavLocation::Top,
    },
    PageInfo {
        icon: include_bytes!("../icons/fluent--trophy-24-regular.svg"),
        name: "Achievements",
        location: NavLocation::Top,
    },
    PageInfo {
        icon: include_bytes!("../icons/fluent--script-24-regular.svg"),
        name: "Add-ons",
        location: NavLocation::Bottom,
    },
    PageInfo {
        icon: include_bytes!("../icons/fluent--settings-24-regular.svg"),
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
                                Element::from(
                                    button(Element::from(row![
                                        svg.content_fit(ContentFit::Contain).width(30),
                                        text(name).align_x(Alignment::End).width(Fill),
                                    ]))
                                    .width(Fill)
                                    .on_press_maybe(
                                        if self.active_item == index {
                                            None
                                        } else {
                                            Some(Message::NavSelected(index))
                                        },
                                    ),
                                )
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
        .width(160)
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavSelected(v) => {
                self.active_item = v;
            }
        }
    }
}

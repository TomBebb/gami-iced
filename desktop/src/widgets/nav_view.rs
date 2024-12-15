use iced::widget::svg::Handle;
use iced::widget::{button, row, text, Column, Svg};
use iced::{Alignment, ContentFit, Element, Fill, Theme};

#[derive(Copy, Clone, Debug)]
pub enum Message {
    NavSelected(usize),
}

struct PageInfo<'a> {
    icon: &'a [u8],
    name: &'a str,
}
const PAGES: &[PageInfo] = &[
    PageInfo {
        icon: include_bytes!("../icons/tabler--minus.svg"),
        name: "Counter",
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--books.svg"),
        name: "Library",
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--trophy.svg"),
        name: "Achievements",
    },
    PageInfo {
        icon: include_bytes!("../icons/tabler--settings.svg"),
        name: "Settings",
    },
];
#[derive(Debug, Clone, Copy, Default)]
pub struct NavView {
    pub active_item: usize,
}
impl NavView {
    pub fn view(&self) -> Column<Message> {
        Column::with_children(PAGES.into_iter().enumerate().map(
            |(index, &PageInfo { name, icon })| {
                let svg: Svg<'static, Theme> = Svg::new(Handle::from_memory(icon));
                Element::from(
                    button(Element::from(row![
                        svg.content_fit(ContentFit::Contain).width(30),
                        text(name).align_x(Alignment::End).width(Fill),
                    ]))
                    .width(Fill)
                    .on_press_maybe(if self.active_item == index {
                        None
                    } else {
                        Some(Message::NavSelected(index))
                    }),
                )
            },
        ))
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

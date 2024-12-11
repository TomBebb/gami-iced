use crate::pages::library::LibraryPage;
use crate::widgets::nav_view::NavView;
use iced::widget::Row;
use iced::Element;
use pages::app_page::{AppPage, PageMessage};
use pages::counter::Counter;

mod pages;
mod widgets;

#[derive(Copy, Clone, Debug)]
enum Message {
    Page(PageMessage),
    NavView(widgets::nav_view::Message),
}
#[derive(Clone, Default)]
struct App {
    pub nav: NavView,
    pub page: AppPage,
}
impl App {
    pub fn view(&self) -> Row<Message> {
        let nav = Element::new(self.nav.view()).map(Message::NavView);
        let page = self.page.view().map(Message::Page);
        iced::widget::row![nav, page]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavView(v) => {
                match v {
                    widgets::nav_view::Message::NavSelected(index) => {
                        self.page = match index {
                            0 => AppPage::Counter(Counter::default()),
                            1 => AppPage::Library(LibraryPage::default()),
                            2 => AppPage::Settings,
                            _ => unimplemented!(),
                        }
                    }
                }
                self.nav.update(v)
            }
            Message::Page(p) => self.page.update(p),
        }
    }
}
pub fn main() -> iced::Result {
    iced::run("A cool counter", App::update, App::view)
}

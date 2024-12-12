use crate::pages::library;
use crate::pages::library::LibraryPage;
use crate::widgets::nav_view::NavView;
use gami_backend::plugin::ExternalAddons;
use iced::widget::Row;
use iced::{Element, Task};
use pages::app_page::{AppPage, PageMessage};
use pages::counter::Counter;
use std::cell::LazyCell;
use tokio::runtime::Builder;

mod pages;
mod widgets;

#[derive(Clone, Debug)]
enum Message {
    Page(PageMessage),
    NavView(widgets::nav_view::Message),
}
#[derive(Clone, Default)]
struct App {
    pub nav: NavView,
    pub page: AppPage,
}

pub const ADDONS: LazyCell<ExternalAddons> = LazyCell::new(|| unsafe {
    let mut addons = ExternalAddons::new();
    addons.auto_load_addons().unwrap();
    addons
});
impl App {
    pub fn view(&self) -> Row<Message> {
        let nav = Element::new(self.nav.view()).map(Message::NavView);
        let page = self.page.view().map(Message::Page);
        iced::widget::row![nav, page]
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavView(v) => {
                self.nav.update(v);
                match v {
                    widgets::nav_view::Message::NavSelected(index) => {
                        self.page = match index {
                            0 => AppPage::Counter(Counter::default()),
                            1 => AppPage::Library(LibraryPage::new()),
                            2 => AppPage::Settings,
                            _ => unimplemented!(),
                        };
                        if let AppPage::Library(inner_lib) = &mut self.page {
                            return inner_lib
                                .update(library::Message::ReloadCache)
                                .map(PageMessage::Library)
                                .map(Message::Page);
                        }
                    }
                }
            }
            Message::Page(p) => return self.page.update(p).map(Message::Page),
        }

        Task::none()
    }
}
pub fn main() -> iced::Result {
    env_logger::init();
    log::info!("Starting Application");
    log::info!("Addons: {:?}", ADDONS.get_keys());
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    runtime.block_on(gami_backend::db::init());
    iced::run("A cool counter", App::update, App::view)
}

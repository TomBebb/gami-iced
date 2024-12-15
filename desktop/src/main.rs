use crate::pages::achivements::Achievements;
use crate::pages::library;
use crate::pages::library::LibraryPage;
use crate::widgets::nav_view::NavView;
use gami_backend::PLUGINS;
use iced::widget::Row;
use iced::{Element, Task};
use pages::app_page::{AppPage, PageMessage};
use pages::counter::Counter;
use std::path::Path;

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
                            2 => AppPage::Achivements(Achievements::default()),
                            3 => AppPage::Settings,
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
#[tokio::main]
pub async fn main() -> iced::Result {
    env_logger::init();
    PLUGINS
        .load(Path::new(
            "/home/tom/Code/gami-iced/addon-steam/dist/index.js",
        ))
        .await
        .unwrap();

    log::info!("Starting Application");
    gami_backend::db::init().await;
    iced::application("Gami", App::update, App::view).run()
}

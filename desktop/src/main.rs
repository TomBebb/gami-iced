use crate::pages::achievements::Achievements;
use crate::pages::library;
use crate::pages::library::LibraryPage;
use crate::pages::settings::SettingsPage;
use crate::widgets::nav_view::NavView;
use iced::application::Title;
use iced::widget::Row;
use iced::{Element, Task, Theme};
use pages::add_ons::AddOns;
use pages::app_page::{AppPage, PageMessage};

mod models;
mod pages;
mod settings;
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
                            0 => AppPage::Library(LibraryPage::new()),
                            1 => AppPage::Achievements(Achievements::default()),
                            2 => AppPage::AddOns(AddOns::new()),
                            3 => AppPage::Settings(SettingsPage::default()),
                            _ => {
                                log::error!("No such page with index {}", index);
                                self.page.clone()
                            }
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
struct AppTitle;
impl Title<App> for AppTitle {
    fn title(&self, state: &App) -> String {
        format!("{} - Gami", state.nav.get_name())
    }
}
#[tokio::main]
pub async fn main() -> iced::Result {
    env_logger::init();

    log::info!("Starting Application");
    gami_backend::db::init().await;
    iced::application(AppTitle, App::update, App::view)
        .theme(|_| Theme::Dark)
        .run()
}

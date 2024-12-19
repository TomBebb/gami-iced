use crate::icons::FONT_BYTES;
use crate::pages::achievements::Achievements;
use crate::pages::library;
use crate::pages::library::LibraryPage;
use crate::pages::settings::SettingsPage;
use crate::widgets::nav_view::NavView;
use iced::application::Title;
use iced::futures::{SinkExt, Stream};
use iced::widget::Row;
use iced::{font, stream, Element, Settings, Subscription, Task};
use pages::add_ons::AddOns;
use pages::app_page::{AppPage, PageMessage};

pub mod icons;
mod models;
mod pages;
mod settings;
mod widgets;

#[derive(Clone, Debug)]
enum Message {
    Startup,
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
            Message::Startup => {
                if let AppPage::Library(inner_lib) = &mut self.page {
                    inner_lib
                        .update(library::Message::ReloadCache)
                        .map(PageMessage::Library)
                        .map(Message::Page)
                } else {
                    Task::none()
                }
            }
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
                        match &mut self.page {
                            AppPage::Library(inner_lib) => inner_lib
                                .update(library::Message::ReloadCache)
                                .map(PageMessage::Library)
                                .map(Message::Page),
                            AppPage::Settings(page) => page
                                .update(pages::settings::Message::LoadSettings)
                                .map(PageMessage::Settings)
                                .map(Message::Page),
                            _ => Task::none(),
                        }
                    }
                }
            }
            Message::Page(p) => self.page.update(p).map(Message::Page),
        }
    }
}
struct AppTitle;
impl Title<App> for AppTitle {
    fn title(&self, state: &App) -> String {
        format!("{} - Gami", state.nav.get_name())
    }
}
fn startup_msg_worker() -> impl Stream<Item = ()> {
    stream::channel(100, |mut output| async move {
        output.send(()).await.unwrap();
    })
}
#[tokio::main]
pub async fn main() -> iced::Result {
    env_logger::init();

    log::info!("Starting Application");
    gami_backend::db::init().await;

    let settings = settings::load().ok().unwrap_or_default();
    iced::application(AppTitle, App::update, App::view)
        .subscription(|_| Subscription::run(startup_msg_worker).map(|_| Message::Startup))
        .settings(Settings {
            fonts: vec![FONT_BYTES.into()].into(),
            ..Settings::default()
        })
        .theme(move |_| settings.appearance.theme.into())
        .run()
}

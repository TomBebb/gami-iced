use crate::pages::achievements::Achievements;
use crate::pages::add_ons::AddOns;
use crate::pages::app_page::{AppPage, PageMessage};
use crate::pages::library;
use crate::pages::library::LibraryPage;
use crate::pages::settings::SettingsPage;
use crate::pages::tools::ToolsPage;
use crate::widgets::nav_view::NavView;
use crate::{pages, widgets};
use gami_backend::Direction;
use iced::advanced::graphics::image::image_rs::ImageFormat;
use iced::application::Title;
use iced::keyboard::key::Named;
use iced::widget::Row;
use iced::window::{icon, Icon, Id};
use iced::{keyboard, window, Element, Task};

#[derive(Clone, Default)]
pub struct App {
    pub nav: NavView,
    pub page: AppPage,
}

impl App {
    pub fn view(&self) -> Row<Message> {
        let nav = Element::new(self.nav.view()).map(Message::NavView);
        let page = self.page.view().map(Message::Page);
        iced::widget::row![nav, page]
    }
    pub fn move_dir_auto(&mut self, dir: Direction) -> Task<Message> {
        if let AppPage::Library(inner_lib) = &mut self.page {
            inner_lib
                .update(library::Message::MoveInDir(dir))
                .map(PageMessage::Library)
                .map(Message::Page)
        } else {
            Task::none()
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Startup => {
                if let AppPage::Library(inner_lib) = &mut self.page {
                    inner_lib
                        .update(library::Message::ReloadCache)
                        .map(PageMessage::Library)
                        .map(Message::Page)
                        .then(|p| {
                            window::get_oldest().and_then(move |id: Id| {
                                let my_p = p.clone();
                                window::change_icon::<Icon>(
                                    id,
                                    icon::from_file_data(
                                        include_bytes!("icons/icon.png"),
                                        Some(ImageFormat::Png),
                                    )
                                    .unwrap(),
                                )
                                .map(move |_| my_p.clone())
                            })
                        })
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
                            3 => AppPage::Tools(ToolsPage::default()),
                            4 => AppPage::Settings(SettingsPage::default()),
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
            Message::KeyDown(keyboard::Key::Named(Named::ArrowUp), _) => {
                self.move_dir_auto(Direction::Up)
            }
            Message::KeyDown(keyboard::Key::Named(Named::ArrowDown), _) => {
                self.move_dir_auto(Direction::Down)
            }
            msg => {
                println!("{:?}", msg);
                Task::none()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Startup,
    Page(PageMessage),
    NavView(widgets::nav_view::Message),
    KeyDown(keyboard::Key, keyboard::Modifiers),
}

pub struct AppTitle;

impl Title<App> for AppTitle {
    fn title(&self, state: &App) -> String {
        format!("{} - Gami", state.nav.get_name())
    }
}

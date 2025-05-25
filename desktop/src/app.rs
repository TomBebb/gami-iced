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
use iced::window::{icon, Icon, Id, Mode};
use iced::{keyboard, window, Element, Task};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum AppState {
    Fullscreen,
    #[default]
    Desktop,
}
#[derive(Clone, Default)]
pub struct App {
    pub state: AppState,
    pub nav: NavView,
    pub page: AppPage,
}

impl App {
    pub fn view(&self) -> Row<Message> {
        let nav = Element::new(self.nav.view()).map(Message::NavView);
        let page = self.page.view().map(Message::Page);
        if self.state == AppState::Desktop {
            iced::widget::row![nav, page]
        } else {
            iced::widget::row![page]
        }
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
            Message::Startup1 => {
                if let AppPage::Library(inner_lib) = &mut self.page {
                    inner_lib
                        .update(library::Message::ReloadCache)
                        .map(PageMessage::Library)
                        .map(Message::Page)
                } else {
                    Task::none()
                }
            }
            Message::Startup2 => window::get_oldest()
                .and_then(move |id: Id| {
                    window::change_icon::<Icon>(
                        id,
                        icon::from_file_data(
                            include_bytes!("icons/icon.png"),
                            Some(ImageFormat::Png),
                        )
                        .unwrap(),
                    )
                })
                .map(|_| Message::Page(PageMessage::Library(library::Message::NoOp))),
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
            Message::KeyDown(keyboard::Key::Named(Named::F11), _) => {
                self.update(Message::SwitchState(if self.state == AppState::Desktop {
                    AppState::Fullscreen
                } else {
                    AppState::Desktop
                }))
            }
            Message::KeyDown(keyboard::Key::Named(Named::ArrowUp), _) => {
                self.move_dir_auto(Direction::Up)
            }
            Message::KeyDown(keyboard::Key::Named(Named::ArrowDown), _) => {
                self.move_dir_auto(Direction::Down)
            }
            Message::SwitchState(state) => {
                self.state = state;
                Task::none()
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
    Startup1,
    Page(PageMessage),
    NavView(widgets::nav_view::Message),
    KeyDown(keyboard::Key, keyboard::Modifiers),
    SwitchState(AppState),
    Startup2,
}

pub struct AppTitle;

impl Title<App> for AppTitle {
    fn title(&self, state: &App) -> String {
        format!("{} - Gami", state.nav.get_name())
    }
}

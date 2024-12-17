use crate::ui::widgets::header;
use crate::ui::widgets::header::Header;
use gami_backend::db;
use gami_backend::db::ops::GamesFilters;
use gami_sdk::GameData;
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::{column, text, Column};
use iced::{keyboard, Element, Task, Theme};

mod ui;
#[derive(Clone, Debug)]
enum Message {
    NavLeft,
    NavRight,
    NavUp,
    NavDown,
    ReloadCache,
    LoadedCache(Vec<GameData>),
    Header(header::Message),
}
#[derive(Clone)]
struct App {
    pub header: Header,
    pub games: Vec<GameData>,
    pub filter: GamesFilters,
    pub curr_index: usize,
}
impl Default for App {
    fn default() -> Self {
        Self {
            header: Header::new(),
            games: Vec::new(),
            filter: GamesFilters::default(),
            curr_index: 0,
        }
    }
}
impl App {
    pub fn view(&self) -> Column<Message> {
        let page = text("TODO");
        let wrapped_header: Element<Message> = self.header.view().map(Message::Header);

        column![wrapped_header, page].into()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ReloadCache => {
                return Task::perform(
                    db::ops::get_games(self.filter.clone()),
                    Message::LoadedCache,
                )
            }
            Message::LoadedCache(cache) => {
                self.games = cache;
            }
            Message::Header(header) => self.header.update(header),
            _ => println!("Unhandled message: {:?}", message),
        }
        Task::none()
    }
}
pub fn main() -> iced::Result {
    env_logger::init();

    log::info!("Starting Big Picture Mode");
    iced::application("Gami Big Picture", App::update, App::view)
        .theme(|_| Theme::Dark)
        .subscription(|a| {
            keyboard::on_key_press(|key, mods| {
                println!("Key press:{:?} w/ {:?}", key, mods);
                match key {
                    Key::Named(Named::ArrowLeft) => Some(Message::NavLeft),
                    Key::Named(Named::ArrowRight) => Some(Message::NavRight),
                    Key::Named(Named::ArrowUp) => Some(Message::NavUp),
                    Key::Named(Named::ArrowDown) => Some(Message::NavDown),
                    _ => None,
                }
            })
        })
        .run()
}

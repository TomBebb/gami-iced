use crate::inputs::Input;
use crate::ui::widgets::header;
use crate::ui::widgets::header::Header;
use gami_backend::db;
use gami_backend::db::ops::GamesFilters;
use gami_sdk::GameData;
use gilrs::{Event, EventType, Gilrs};
use iced::futures::sink::SinkExt;
use iced::stream;
use iced::widget::{column, text, Column};
use iced::Subscription;
use iced::{keyboard, Element, Task, Theme};
use std::time::{Duration, Instant};
use tokio::task;

use futures::stream::{Stream, StreamExt};
mod inputs;
mod ui;

#[derive(Clone, Debug, Copy)]
pub enum InputState {
    Pressed,
    Released,
}

#[derive(Clone, Debug)]
pub enum Message {
    Noop,
    RawInput(Input, InputState),
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
fn timed_worker() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        loop {
            output
                .send(Message::Header(header::Message::UpdateTime(Instant::now())))
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    })
}

fn gamepad_worker() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        let mut gilrs = Gilrs::new().unwrap();

        loop {
            // Examine new events
            while let Some(Event {
                id, event, time, ..
            }) = gilrs.next_event()
            {
                log::info!("{:?} New event from {}: {:?}", time, id, event);

                match event {
                    EventType::Disconnected => {
                        log::info!("Gamepad disconnected");
                    }
                    EventType::ButtonPressed(button, _) => {
                        if let Ok(input) = Input::try_from(button) {
                            log::info!("Gamepad pressed: {:?}", input);
                            output.send(Message::RawInput(input, InputState::Pressed)).await.unwrap();
                        } else {
                            log::warn!("Unknown button {:?}", button);
                        }
                    }
                    EventType::ButtonReleased(button, _) => {
                        if let Ok(input) = Input::try_from(button) {
                            log::info!("Gamepad released: {:?}", input);
                            output.send(Message::RawInput(input, InputState::Released)).await.unwrap();
                        } else {
                            log::warn!("Unknown button {:?}", button);
                        }
                    }
                    _ => {}
                }
            }
        }
    })
}
#[tokio::main]
pub async fn main() -> iced::Result {
    env_logger::init();

    let gp = task::spawn_blocking(move || {});
    log::info!("Starting Big Picture Mode");
    iced::application("Gami Big Picture", App::update, App::view)
        .theme(|_| Theme::Dark)
        .subscription(|_| Subscription::run(timed_worker))
        .subscription(|_| {
            keyboard::on_key_press(|key, mods| {
                log::info!("Key press:{:?} w/ {:?}", key, mods);
                let mapped = Input::try_from(key).ok();
                mapped.map(|v| Message::RawInput(v, InputState::Pressed))
            })
        })
        .subscription(|_| {
            Subscription::run(gamepad_worker)
        })
        .exit_on_close_request(true)
        .run()?;

    gp.await.unwrap();
    Ok(())
}

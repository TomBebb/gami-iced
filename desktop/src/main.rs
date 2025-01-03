use app::{App, AppTitle, Message};
use iced::futures::{SinkExt, Stream};
use iced::{keyboard, stream, Subscription};

mod app;
mod models;
mod pages;
mod settings;
mod widgets;

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
        .subscription(|_| {
            Subscription::batch([
                keyboard::on_key_press(|key, mods| Some(Message::KeyDown(key, mods))),
                Subscription::run(startup_msg_worker).map(|_| Message::Startup),
            ])
        })
        .theme(move |_| settings.appearance.theme.into())
        .run()
}

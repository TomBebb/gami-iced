use crate::pages;
use crate::pages::counter::Counter;
use iced::widget::text;
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub enum PageMessage {
    Counter(pages::counter::CounterMessage),
    Library(pages::library::Message),
    Achievements(pages::achievements::Message),
    Settings(pages::settings::Message),
}

#[derive(Clone, Debug)]

pub enum AppPage {
    Counter(Counter),
    Library(pages::library::LibraryPage),
    Achievements(pages::achievements::Achievements),
    Settings(pages::settings::SettingsPage),
}
impl Default for AppPage {
    fn default() -> Self {
        Self::Counter(Counter::default())
    }
}

impl AppPage {
    pub fn view(&self) -> Element<PageMessage> {
        match self {
            AppPage::Counter(counter) => Element::from(counter.view()).map(PageMessage::Counter),
            AppPage::Library(lib) => Element::from(lib.view()).map(PageMessage::Library),
            AppPage::Achievements(page) => page.view().map(PageMessage::Achievements),
            AppPage::Settings(page) => page.view().map(PageMessage::Settings),
        }
    }
    pub fn update(&mut self, message: PageMessage) -> Task<PageMessage> {
        match (self, message) {
            (AppPage::Counter(counter), PageMessage::Counter(v)) => counter.update(v),
            (AppPage::Library(lib), PageMessage::Library(v)) => {
                return lib.update(v).map(PageMessage::Library);
            }
            (AppPage::Achievements(page), PageMessage::Achievements(v)) => page.update(v),
            (AppPage::Settings(page), PageMessage::Settings(v)) => page.update(v),
            _ => unimplemented!(),
        }
        Task::none()
    }
}

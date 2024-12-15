use crate::pages;
use crate::pages::counter::Counter;
use iced::widget::text;
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub enum PageMessage {
    Counter(pages::counter::CounterMessage),
    Library(pages::library::Message),
    Achievements(pages::achivements::Message),
}

#[derive(Clone, Debug)]

pub enum AppPage {
    Counter(Counter),
    Library(pages::library::LibraryPage),
    Achivements(pages::achivements::Achievements),
    Settings,
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
            AppPage::Achivements(page) => page.view().map(PageMessage::Achievements),
            AppPage::Settings => Element::from(text("TODO settings")),
        }
    }
    pub fn update(&mut self, message: PageMessage) -> Task<PageMessage> {
        match (self, message) {
            (AppPage::Counter(counter), PageMessage::Counter(v)) => counter.update(v),
            (AppPage::Library(lib), PageMessage::Library(v)) => {
                return lib.update(v).map(PageMessage::Library);
            }
            (AppPage::Achivements(page), PageMessage::Achievements(v)) => page.update(v),
            _ => unimplemented!(),
        }
        Task::none()
    }
}

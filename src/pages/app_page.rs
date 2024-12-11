use crate::pages;
use crate::pages::counter::Counter;
use iced::widget::text;
use iced::Element;

#[derive(Debug, Copy, Clone)]
pub enum PageMessage {
    Counter(pages::counter::CounterMessage),
    Library(pages::library::Message),
}

#[derive(Clone, Debug)]

pub enum AppPage {
    Counter(Counter),
    Library(pages::library::LibraryPage),
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
            AppPage::Settings => Element::from(text("TODO settings")),
        }
    }
    pub fn update(&mut self, message: PageMessage) {
        match (self, message) {
            (AppPage::Counter(counter), PageMessage::Counter(v)) => counter.update(v),
            (AppPage::Library(lib), PageMessage::Library(v)) => lib.update(v),
            _ => panic!("Unregistered update: {:?}", message,),
        }
    }
}

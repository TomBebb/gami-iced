use crate::pages;
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub enum PageMessage {
    AddOns(pages::add_ons::AddOnMessage),
    Library(pages::library::Message),
    Achievements(pages::achievements::Message),
    Settings(pages::settings::Message),
}

#[derive(Clone, Debug)]

pub enum AppPage {
    AddOns(pages::add_ons::AddOns),
    Library(pages::library::LibraryPage),
    Achievements(pages::achievements::Achievements),
    Settings(pages::settings::SettingsPage),
}
impl Default for AppPage {
    fn default() -> Self {
        Self::Library(pages::library::LibraryPage::new())
    }
}

impl AppPage {
    pub fn view(&self) -> Element<PageMessage> {
        match self {
            AppPage::AddOns(counter) => Element::from(counter.view()).map(PageMessage::AddOns),
            AppPage::Library(lib) => Element::from(lib.view()).map(PageMessage::Library),
            AppPage::Achievements(page) => page.view().map(PageMessage::Achievements),
            AppPage::Settings(page) => page.view().map(PageMessage::Settings),
        }
    }
    pub fn update(&mut self, message: PageMessage) -> Task<PageMessage> {
        match (self, message) {
            (AppPage::AddOns(counter), PageMessage::AddOns(v)) => {
                return counter.update(v).map(PageMessage::AddOns);
            }
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

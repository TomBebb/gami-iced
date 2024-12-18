use crate::models::MyTheme;
use crate::settings;
use crate::settings::{AppearanceSettings, Settings};
use iced::font::Weight;
use iced::widget::{column, pick_list, row, text};
use iced::{Element, Font, Length, Task};
use iced_aw::{TabLabel, Tabs};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum TabId {
    #[default]
    General,
    Appearance,
    Metadata,
}

#[derive(Clone, Debug)]
pub enum Message {
    LoadSettings,
    TabSelected(TabId),
    Changed(Settings),
    Loaded(Settings),
}
#[derive(Default, Clone, Debug)]
pub struct SettingsPage {
    active_tab: TabId,
    settings: Settings,
}
impl SettingsPage {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::LoadSettings => {
                return Task::perform(
                    async { settings::load_async().await.unwrap() },
                    Message::Loaded,
                )
            }
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
            Message::Changed(settings) => {
                settings::save(&settings).unwrap();
                self.settings = settings;
            }
            Message::Loaded(settings) => {
                self.settings = settings;
            }
        }

        return Task::none();
    }
    pub fn view(&self) -> Element<Message> {
        Tabs::new_with_tabs(
            vec![
                (
                    TabId::General,
                    TabLabel::Text("General".into()),
                    text("TODO: General").into(),
                ),
                (
                    TabId::Appearance,
                    TabLabel::Text("Appearance".into()),
                    column![row![
                        text("Theme:")
                            .font(Font {
                                weight: Weight::Semibold,
                                ..Font::default()
                            })
                            .width(Length::FillPortion(3)),
                        pick_list(
                            MyTheme::ALL,
                            Some(self.settings.appearance.theme),
                            |theme| Message::Changed(Settings {
                                appearance: AppearanceSettings { theme },
                                ..self.settings.clone()
                            }),
                        )
                        .placeholder("Select your theme")
                        .width(Length::FillPortion(7)),
                    ]]
                    .into(),
                ),
                (
                    TabId::Metadata,
                    TabLabel::Text("Metadata".into()),
                    text("TODO: Metadata").into(),
                ),
            ],
            Message::TabSelected,
        )
        .set_active_tab(&self.active_tab)
        .into()
    }
}

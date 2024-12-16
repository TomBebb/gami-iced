use iced::widget::text;
use iced::Element;
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
    TabSelected(TabId),
}
#[derive(Default, Clone, Debug)]
pub struct SettingsPage {
    active_tab: TabId,
}
impl SettingsPage {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::TabSelected(tab) => self.active_tab = tab,
        }
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
                    text("TODO: Appearance").into(),
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

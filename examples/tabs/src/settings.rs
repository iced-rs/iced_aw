use crate::{Icon, Message, Tab};
use iced::{
    widget::{Column, Container, Radio, Text},
    Element,
};
use iced_aw::style::{tab_bar, StyleFn};
use iced_aw::tab_bar::TabLabel;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabBarPosition {
    #[default]
    Top,
    Bottom,
}

impl TabBarPosition {
    pub const ALL: [TabBarPosition; 2] = [TabBarPosition::Top, TabBarPosition::Bottom];
}

impl From<TabBarPosition> for String {
    fn from(position: TabBarPosition) -> Self {
        String::from(match position {
            TabBarPosition::Top => "Top",
            TabBarPosition::Bottom => "Bottom",
        })
    }
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub struct TabSettings {
    pub tab_bar_position: Option<TabBarPosition>,
    pub tab_bar_theme: Option<usize>,
    pub tab_bar_theme_id: Option<usize>,
}

impl TabSettings {
    pub fn new() -> Self {
        TabSettings {
            tab_bar_position: Some(TabBarPosition::Top),
            tab_bar_theme: Some(0),
            tab_bar_theme_id: Some(0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    PositionSelected(TabBarPosition),
    ThemeSelected(usize),
}

#[derive(Default)]
pub struct SettingsTab {
    settings: TabSettings,
}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {
            settings: TabSettings::new(),
        }
    }

    pub fn settings(&self) -> &TabSettings {
        &self.settings
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::PositionSelected(position) => {
                self.settings.tab_bar_position = Some(position)
            }
            SettingsMessage::ThemeSelected(index) => {
                self.settings.tab_bar_theme_id = Some(index);
                self.settings.tab_bar_theme = Some(index)
            }
        }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())
        TabLabel::IconText(Icon::CogAlt.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, SettingsMessage> = Container::new(
            Column::new()
                .push(Text::new("TabBar position:").size(20))
                .push(TabBarPosition::ALL.iter().cloned().fold(
                    Column::new().padding(10).spacing(10),
                    |column, position| {
                        column.push(
                            Radio::new(
                                position,
                                position,
                                self.settings().tab_bar_position,
                                SettingsMessage::PositionSelected,
                            )
                            .size(16),
                        )
                    },
                ))
                .push(Text::new("TabBar color:").size(20))
                .push(
                    (0..6).fold(Column::new().padding(10).spacing(10), |column, id| {
                        column.push(
                            Radio::new(
                                predefined_style(id),
                                id,
                                self.settings().tab_bar_theme_id,
                                SettingsMessage::ThemeSelected,
                            )
                            .size(16),
                        )
                    }),
                ),
        )
        .into();

        content.map(Message::Settings)
    }
}

fn predefined_style(index: usize) -> String {
    match index {
        0 => "Default".to_owned(),
        1 => "Dark".to_owned(),
        2 => "Red".to_owned(),
        3 => "Blue".to_owned(),
        4 => "Green".to_owned(),
        5 => "Purple".to_owned(),
        _ => "Default".to_owned(),
    }
}

pub fn style_from_index(index: usize) -> StyleFn<'static, iced::Theme, tab_bar::Style> {
    match index {
        0 => Box::new(tab_bar::primary),
        1 => Box::new(tab_bar::dark),
        2 => Box::new(tab_bar::red),
        3 => Box::new(tab_bar::blue),
        4 => Box::new(tab_bar::green),
        5 => Box::new(tab_bar::purple),
        _ => Box::new(tab_bar::primary),
    }
}

use crate::{Icon, Message, Tab};
use iced::{
    widget::{Column, Container, Radio, Text},
    Element,
};
use iced_aw::style::TabBarStyles;
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
pub struct TabSettings {
    pub tab_bar_position: Option<TabBarPosition>,
    pub tab_bar_theme: Option<TabBarStyles>,
}

impl TabSettings {
    pub fn new() -> Self {
        TabSettings {
            tab_bar_position: Some(TabBarPosition::Top),
            tab_bar_theme: Some(TabBarStyles::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    PositionSelected(TabBarPosition),
    ThemeSelected(TabBarStyles),
}

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
            SettingsMessage::ThemeSelected(theme) => self.settings.tab_bar_theme = Some(theme),
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
                                predefined_style(id),
                                self.settings().tab_bar_theme,
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

fn predefined_style(index: usize) -> TabBarStyles {
    match index {
        0 => TabBarStyles::Default,
        1 => TabBarStyles::Dark,
        2 => TabBarStyles::Red,
        3 => TabBarStyles::Blue,
        4 => TabBarStyles::Green,
        5 => TabBarStyles::Purple,
        _ => TabBarStyles::Default,
    }
}

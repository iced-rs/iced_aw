use crate::{Icon, Message, Tab};
use iced::{
    widget::{Column, Container, Radio, Text},
    Element,
};
use iced_aw::sidebar::TabLabel;
use iced_aw::style::{sidebar, StyleFn};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarPosition {
    #[default]
    Start,
    End,
}

impl SidebarPosition {
    pub const ALL: [SidebarPosition; 2] = [SidebarPosition::Start, SidebarPosition::End];
}

impl From<SidebarPosition> for String {
    fn from(position: SidebarPosition) -> Self {
        String::from(match position {
            SidebarPosition::Start => "Start",
            SidebarPosition::End => "End",
        })
    }
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub struct TabSettings {
    pub sidebar_position: Option<SidebarPosition>,
    pub sidebar_theme: Option<usize>,
    pub sidebar_theme_id: Option<usize>,
}

impl TabSettings {
    pub fn new() -> Self {
        TabSettings {
            sidebar_position: Some(SidebarPosition::Start),
            sidebar_theme: Some(0),
            sidebar_theme_id: Some(0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    PositionSelected(SidebarPosition),
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
                self.settings.sidebar_position = Some(position)
            }
            SettingsMessage::ThemeSelected(index) => {
                self.settings.sidebar_theme_id = Some(index);
                self.settings.sidebar_theme = Some(index)
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
                .push(SidebarPosition::ALL.iter().cloned().fold(
                    Column::new().padding(10).spacing(10),
                    |column, position| {
                        column.push(
                            Radio::new(
                                position,
                                position,
                                self.settings().sidebar_position,
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
                                self.settings().sidebar_theme_id,
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

pub fn style_from_index(index: usize) -> StyleFn<'static, iced::Theme, sidebar::Style> {
    match index {
        0 => Box::new(sidebar::primary),
        1 => Box::new(sidebar::dark),
        2 => Box::new(sidebar::red),
        3 => Box::new(sidebar::blue),
        4 => Box::new(sidebar::green),
        5 => Box::new(sidebar::purple),
        _ => Box::new(sidebar::primary),
    }
}

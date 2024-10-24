mod login;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
    Element, Font, Length,
};
use iced_aw::sidebar::{SidebarWithContent, TabLabel};
use login::{LoginMessage, LoginTab};

mod ferris;
use ferris::{FerrisMessage, FerrisTab};

mod counter;
use counter::{CounterMessage, CounterTab};

mod settings;
use settings::{style_from_index, SettingsMessage, SettingsTab, SidebarPosition};

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;
const ICON_BYTES: &[u8] = include_bytes!("fonts/icons.ttf");
const ICON: Font = Font::with_name("icons");

enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F1EC}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}

fn main() -> iced::Result {
    iced::application(
        "Sidebar example",
        TabBarExample::update,
        TabBarExample::view,
    )
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .font(ICON_BYTES)
        .run()
}

#[derive(Default)]
struct TabBarExample {
    active_tab: TabId,
    login_tab: LoginTab,
    ferris_tab: FerrisTab,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
enum TabId {
    #[default]
    Login,
    Ferris,
    Counter,
    Settings,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(TabId),
    Login(LoginMessage),
    Ferris(FerrisMessage),
    Counter(CounterMessage),
    Settings(SettingsMessage),
    TabClosed(TabId),
}

impl TabBarExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => self.login_tab.update(message),
            Message::Ferris(message) => self.ferris_tab.update(message),
            Message::Counter(message) => self.counter_tab.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
            Message::TabClosed(id) => println!("Tab {:?} event hit", id),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let position = self
            .settings_tab
            .settings()
            .sidebar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .sidebar_theme
            .unwrap_or_default();

        SidebarWithContent::new(Message::TabSelected)
            .tab_icon_position(iced_aw::sidebar::Position::End)
            .on_close(Message::TabClosed)
            .push(
                TabId::Login,
                self.login_tab.tab_label(),
                self.login_tab.view(),
            )
            .push(
                TabId::Ferris,
                self.ferris_tab.tab_label(),
                self.ferris_tab.view(),
            )
            .push(
                TabId::Counter,
                self.counter_tab.tab_label(),
                self.counter_tab.view(),
            )
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .sidebar_style(style_from_index(theme))
            .icon_font(ICON)
            .sidebar_position(match position {
                SidebarPosition::Start => iced_aw::sidebar::SidebarPosition::Start,
                SidebarPosition::End => iced_aw::sidebar::SidebarPosition::End,
            })
            .into()
    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}

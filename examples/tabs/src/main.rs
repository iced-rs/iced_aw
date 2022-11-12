mod login;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
    Element, Font, Length, Sandbox, Settings,
};
use iced_aw::{TabLabel, Tabs};
use login::{LoginMessage, LoginTab};

mod ferris;
use ferris::{FerrisMessage, FerrisTab};

mod counter;
use counter::{CounterMessage, CounterTab};

mod settings;
use settings::{SettingsMessage, SettingsTab, TabBarPosition};

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

const ICON_FONT: Font = iced::Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

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
    TabBarExample::run(Settings::default())
}

struct TabBarExample {
    active_tab: usize,
    login_tab: LoginTab,
    ferris_tab: FerrisTab,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(usize),
    Login(LoginMessage),
    Ferris(FerrisMessage),
    Counter(CounterMessage),
    Settings(SettingsMessage),
}

impl Sandbox for TabBarExample {
    type Message = Message;

    fn new() -> Self {
        TabBarExample {
            active_tab: 0,
            login_tab: LoginTab::new(),
            ferris_tab: FerrisTab::new(),
            counter_tab: CounterTab::new(),
            settings_tab: SettingsTab::new(),
        }
    }

    fn title(&self) -> String {
        String::from("TabBar Example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => self.login_tab.update(message),
            Message::Ferris(message) => self.ferris_tab.update(message),
            Message::Counter(message) => self.counter_tab.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let position = self
            .settings_tab
            .settings()
            .tab_bar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .tab_bar_theme
            .unwrap_or_default();

        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.login_tab.tab_label(), self.login_tab.view())
            .push(self.ferris_tab.tab_label(), self.ferris_tab.view())
            .push(self.counter_tab.tab_label(), self.counter_tab.view())
            .push(self.settings_tab.tab_label(), self.settings_tab.view())
            .tab_bar_style(theme)
            .icon_font(ICON_FONT)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
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
            .push(self.content());

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

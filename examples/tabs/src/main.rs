mod login;
use iced::{
    alignment::{self, Horizontal, Vertical},
    font,
    widget::{container, text, Column, Container, Text},
    Application, Command, Element, Font, Length, Settings, Theme,
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
const ICON_BYTES: &[u8] = include_bytes!("../fonts/icons.ttf");
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
    TabBarExample::run(Settings::default())
}

enum TabBarExample {
    Loading,
    Loaded(State),
}

struct State {
    active_tab: TabId,
    login_tab: LoginTab,
    ferris_tab: FerrisTab,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
}
#[derive(Clone, PartialEq, Eq, Debug)]
enum TabId {
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
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    TabClosed(TabId),
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for TabBarExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (TabBarExample, Command<Message>) {
        (
            TabBarExample::Loading,
            Command::batch(vec![
                font::load(ICON_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("TabBar Example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            TabBarExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = TabBarExample::Loaded(State {
                        active_tab: TabId::Login,
                        login_tab: LoginTab::new(),
                        ferris_tab: FerrisTab::new(),
                        counter_tab: CounterTab::new(),
                        settings_tab: SettingsTab::new(),
                    })
                }
            }
            TabBarExample::Loaded(state) => match message {
                Message::TabSelected(selected) => state.active_tab = selected,
                Message::Login(message) => state.login_tab.update(message),
                Message::Ferris(message) => state.ferris_tab.update(message),
                Message::Counter(message) => state.counter_tab.update(message),
                Message::Settings(message) => state.settings_tab.update(message),
                Message::TabClosed(id) => println!("Tab {:?} event hit", id),
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            TabBarExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            TabBarExample::Loaded(state) => {
                let position = state
                    .settings_tab
                    .settings()
                    .tab_bar_position
                    .unwrap_or_default();
                let theme = state
                    .settings_tab
                    .settings()
                    .tab_bar_theme
                    .clone()
                    .unwrap_or_default();

                Tabs::new(Message::TabSelected)
                    .tab_icon_position(iced_aw::tabs::Position::Bottom)
                    .on_close(Message::TabClosed)
                    .push(
                        TabId::Login,
                        state.login_tab.tab_label(),
                        state.login_tab.view(),
                    )
                    .push(
                        TabId::Ferris,
                        state.ferris_tab.tab_label(),
                        state.ferris_tab.view(),
                    )
                    .push(
                        TabId::Counter,
                        state.counter_tab.tab_label(),
                        state.counter_tab.view(),
                    )
                    .push(
                        TabId::Settings,
                        state.settings_tab.tab_label(),
                        state.settings_tab.view(),
                    )
                    .set_active_tab(&state.active_tab)
                    .tab_bar_style(theme.clone())
                    .icon_font(ICON)
                    .tab_bar_position(match position {
                        TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                        TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
                    })
                    .into()
            }
        }
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
            .align_items(iced::Alignment::Center);

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

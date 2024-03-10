use iced::{
    font,
    widget::{column, row, text},
    Application, Command, Element, Settings, Theme,
};
use iced_aw::core::icons::{
    bootstrap::{self, Bootstrap},
    nerd::{self, Nerd},
};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

#[derive(Debug)]
enum Message {
    FontLoaded(Result<(), font::Error>),
}

impl Application for MyApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (MyApp, Command<Message>) {
        (
            MyApp {},
            Command::batch(vec![
                // There is no automatic way for iced aw to load fonts, you the user have to load
                // them and this is as simple as we can make it currently.
                // Creating your own is easy, check out the source code of
                // [`iced_aw::core::icons`], that's the simplest way to learn.
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("nerd font aint working")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column!(
            text("Bootstrap icons"),
            row!(
                bootstrap::icon_to_text(Bootstrap::FiletypeJava),
                bootstrap::icon_to_text(Bootstrap::X),
            ),
            text("Nerd icons"),
            row!(
                nerd::icon_to_text(Nerd::LanguageJavascript),
                nerd::icon_to_text(Nerd::SplitVertical),
                nerd::icon_to_text(Nerd::X),
                text('\u{f247}'.to_string()).font(iced_aw::NERD_FONT),
            ),
        )
        .into()
    }
}

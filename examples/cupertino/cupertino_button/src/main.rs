use iced::{
    alignment, executor,
    widget::{column, container, text, Text},
    Application, Command, Element, Length, Renderer, Settings, Theme,
};

use iced_aw::native::cupertino::cupertino_button::CupertinoButton;

pub fn main() -> iced::Result {
    ButtonApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

enum ButtonApp {
    Loading,
    EnabledButtonClicked,
    EnabledFilledButtonClicked,
}

#[derive(Debug, Clone)]
enum Message {
    EnabledButtonClicked,
    EnabledFilledButtonClicked,
}

// `cargo fmt` becomes unreadable for this example, so switching off //
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Application for ButtonApp {
    type Executor = executor::Default;
    type Message  = Message;
    type Theme    = Theme;
    type Flags    = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (ButtonApp::Loading, Command::none())
    }

    fn title(&self) -> String {
        String::from("CupertinoButton - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EnabledButtonClicked => {
                println!("You clicked the enabled button!");
                *self = ButtonApp::EnabledButtonClicked;
            },

            Message::EnabledFilledButtonClicked => {
                println!("You clicked the filled enabled button!");
                *self = ButtonApp::EnabledFilledButtonClicked;
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let disabled: CupertinoButton<Message, Renderer> = CupertinoButton::new()
            .on_pressed(None)
            .body(Text::new("Disabled")
                .size(24)
                .horizontal_alignment(alignment::Horizontal::Center)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(50.0))
            );

        let disabled_filled: CupertinoButton<Message, Renderer> = CupertinoButton::new()
            .on_pressed(None)
            .is_filled(true)
            .body(Text::new("Disabled")
                .size(24)
                .horizontal_alignment(alignment::Horizontal::Center)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(50.0))
            );

        let enabled: CupertinoButton<Message, Renderer> = CupertinoButton::new()
            .on_pressed(Some(Message::EnabledButtonClicked))
            .body(Text::new("Enabled")
                .size(24)
                .horizontal_alignment(alignment::Horizontal::Center)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(50.0))
            );

        let enabled_filled: CupertinoButton<Message, Renderer> = CupertinoButton::new()
            .on_pressed(Some(Message::EnabledFilledButtonClicked))
            .is_filled(true)
            .body(Text::new("Enabled")
                .size(24)
                .horizontal_alignment(alignment::Horizontal::Center)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(50.0))
            );

        container(column![
            text("Cupertino Button Example!")
                .width(Length::Fill)
                .height(Length::Fixed(100.0))
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),

            disabled,
            disabled_filled,
            enabled,
            enabled_filled,
        ].width(Length::Fill).align_items(alignment::Horizontal::Center.into()))
            .center_y()
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

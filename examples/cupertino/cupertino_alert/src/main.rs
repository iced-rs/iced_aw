use iced::widget::{column, container, Text};
use iced::{alignment, executor, Application, Command, Element, Length, Settings, Theme};
use iced_aw::native::cupertino::cupertino_alert::{CupertinoAlert, CupertinoDialogAction};
use iced_aw::native::cupertino::cupertino_button::CupertinoButton;
use iced_aw::native::cupertino::cupertino_colours::system_red;

pub fn main() -> iced::Result {
    Alert::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug)]
enum Alert {
    BackdropClicked,
    CancelEvent,
    ConfirmEvent,
    DialogEscape,
    Loading,
    ShowModal,
}

#[derive(Debug, Clone)]
enum Message {
    BackdropClicked,
    CancelEvent,
    ConfirmEvent,
    DialogEscape,
    ShowModal,
}

#[rustfmt::skip]
mod constants {
    pub static PRIMARY_TEXT:   &'static str = "Allow \"Maps\" to access your location while using the app?";
    pub static SECONDARY_TEXT: &'static str = "Your current location will be displayed on the map and used for directions, nearby search results, and estimated travel times. ";
    pub static ALLOW:          &'static str = "Allow";
    pub static DONT_ALLOW:     &'static str = "Don't Allow";
}

use constants::*;

// `cargo fmt` becomes unreadable for this example, so switching off //
#[rustfmt::skip]
impl Application for Alert {
    type Executor = executor::Default;
    type Message  = Message;
    type Theme    = Theme;
    type Flags    = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Alert::Loading, Command::none())
    }

    fn title(&self) -> String {
        String::from("CupertinoAlert - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackdropClicked => *self = Alert::BackdropClicked,
            Message::CancelEvent     => *self = Alert::CancelEvent,
            Message::ConfirmEvent    => *self = Alert::ConfirmEvent,
            Message::DialogEscape    => *self = Alert::DialogEscape,
            Message::ShowModal       => *self = Alert::ShowModal,
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let modal_hidden: bool = match self {
            Alert::ShowModal => false,
            _                => true,
        };

        match self {
            Alert::CancelEvent  => println!("Received click for the cancel button"),
            Alert::ConfirmEvent => println!("Received click for the confirm button"),
            _                   => (),
        };

        let confirm_button: CupertinoButton<Message, _> = CupertinoButton::new()
            .on_pressed(Some(Message::ConfirmEvent))
            .body(Text::new(DONT_ALLOW)
                .size(24)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(50.0))
            );

        let cancel_button: CupertinoButton<Message, _> = CupertinoButton::new()
            .on_pressed(Some(Message::CancelEvent))
            .colour(Some(system_red(1.0)))
            .body(Text::new(ALLOW)
                .size(24)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(50.0))
            );

        let alert = CupertinoAlert::new()
            .is_hidden(modal_hidden)
            .title(PRIMARY_TEXT.to_string())
            .content(SECONDARY_TEXT.to_string())
            .actions(vec![
                CupertinoDialogAction::new()
                    .is_enabled(true)
                    .child(confirm_button.into())
                    .on_pressed(Some(Message::ConfirmEvent)),

                CupertinoDialogAction::new()
                    .is_enabled(true)
                    .child(cancel_button.into())
                    .on_pressed(Some(Message::CancelEvent)),

            ])
            .backdrop(Some(Message::BackdropClicked))
            .on_escape(Some(Message::DialogEscape));

        container(column![
            CupertinoButton::new()
                .body(Text::new("Click to show the CupertinoAlertDialog")
                    .size(24.0)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(75.0))
                    .horizontal_alignment(alignment::Horizontal::Center)
                )
                .on_pressed(Some(Message::ShowModal)),

            alert,
        ].align_items(alignment::Horizontal::Center.into()))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

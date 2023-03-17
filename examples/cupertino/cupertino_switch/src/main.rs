use iced::{alignment, Application, Color, Command, Element, executor, Length, Settings, Theme};
use iced::widget::{column, container, row, text};
use iced_aw::native::cupertino::cupertino_switch::CupertinoSwitch;

pub fn main() -> iced::Result {
    Switch::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

enum Switch {
    Loading,
    LeftSwitchChanged(bool),
    RightSwitchChanged(bool),
}

#[derive(Debug, Clone)]
enum Message {
    LeftSwitchChanged(bool),
    RightSwitchChanged(bool),
}

// `cargo fmt` becomes unreadable for this example, so switching off //
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Application for Switch {
    type Executor = executor::Default;
    type Message  = Message;
    type Theme    = Theme;
    type Flags    = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Switch::Loading, Command::none())
    }

    fn title(&self) -> String {
        String::from("CupertinoSwitch - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LeftSwitchChanged(value) => {
                println!("App detected new value for left switch: {}", value);

                *self = Switch::LeftSwitchChanged(value);
            },

            Message::RightSwitchChanged(value) => {
                println!("App detected new value for right switch: {}", value);

                *self = Switch::RightSwitchChanged(value);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let toggle_1: CupertinoSwitch<Message> = CupertinoSwitch::new().on_changed(Some(Box::new(
            Message::LeftSwitchChanged
        )));

        let toggle_2: CupertinoSwitch<Message> = CupertinoSwitch::new()
            .value(false)
            .on_changed(Some(Box::new(Message::RightSwitchChanged)));

        let left_text: String = match self {
            Switch::LeftSwitchChanged(v) => format!("Left Toggle State: {}", v),
            _                            => format!("Left Toggle State: {}", toggle_1.value),
        };

        let right_text: String = match self {
            Switch::RightSwitchChanged(v) => format!("Right Toggle State: {}", v),
            _                             => format!("Right Toggle State: {}", toggle_2.value),
        };

        let content = row![
            toggle_1,

            container(column![
                text(left_text)
                    .width(Length::Fill)
                    .size(25)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),

                text(right_text)
                    .width(Length::Fill)
                    .size(25)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            ]).width(Length::Fill).center_y(),

            toggle_2,
        ].spacing(100).align_items(alignment::Alignment::Center).width(Length::Shrink);

        // No effect, but here for demonstrative purposes //
        let style: fn(&iced::Theme) -> container::Appearance = |_theme| container::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        };
        //

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}


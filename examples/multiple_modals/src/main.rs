use iced::widget::{button, column, container, row, text, vertical_space};
use iced::{alignment, executor, font, window};
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use iced_aw::{card, modal, CardStyles};

fn main() -> iced::Result {
    MultipleModalsExample::run(Settings {
        window: window::Settings {
            size: iced::Size {
                width: 500.0,
                height: 150.0,
            },
            position: window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    })
}

enum State {
    Start,
    Button1,
    Button2,
    Button3,
    End,
}

enum ButtonPressed {
    Correct,
    Wrong,
}

struct MultipleModalsExample {
    state: State,
    button_pressed: Option<ButtonPressed>,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    ButtonStartPressed,
    Button1Pressed,
    Button2Pressed,
    Button3Pressed,
    ButtonRestartPressed,
    ButtonQuitPressed,
    CloseOverlay,
}

impl Application for MultipleModalsExample {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: State::Start,
                button_pressed: None,
            },
            font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Multiple Modals example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FontLoaded(_) => Command::none(),
            Message::ButtonStartPressed => {
                match self.state {
                    State::Start => self.state = State::Button1,
                    _ => panic!("button `Start` should be shown only with state `Start`"),
                }
                Command::none()
            }
            Message::Button1Pressed => {
                match self.state {
                    State::Button1 => self.button_pressed = Some(ButtonPressed::Correct),
                    State::Button2 | State::Button3 => {
                        self.button_pressed = Some(ButtonPressed::Wrong)
                    }
                    _ => panic!(
                    "button 1 should be shown only with states `Button1`, `Button2`, or `Button3`"
                ),
                }
                Command::none()
            }
            Message::Button2Pressed => {
                match self.state {
                    State::Button2 => self.button_pressed = Some(ButtonPressed::Correct),
                    State::Button1 | State::Button3 => {
                        self.button_pressed = Some(ButtonPressed::Wrong)
                    }
                    _ => panic!(
                    "button 2 should be shown only with states `Button1`, `Button2`, or `Button3`"
                ),
                }
                Command::none()
            }
            Message::Button3Pressed => {
                match self.state {
                    State::Button3 => self.button_pressed = Some(ButtonPressed::Correct),
                    State::Button1 | State::Button2 => {
                        self.button_pressed = Some(ButtonPressed::Wrong)
                    }
                    _ => panic!(
                    "button 3 should be shown only with states `Button1`, `Button2`, or `Button3`"
                ),
                }
                Command::none()
            }
            Message::ButtonRestartPressed => {
                self.state = State::Button1;
                Command::none()
            }
            Message::ButtonQuitPressed => window::close(window::Id::MAIN),
            Message::CloseOverlay => {
                match (&self.state, &self.button_pressed) {
                    (State::Button1, Some(ButtonPressed::Correct)) => self.state = State::Button2,
                    (State::Button2, Some(ButtonPressed::Correct)) => self.state = State::Button3,
                    (State::Button3, Some(ButtonPressed::Correct)) => self.state = State::End,
                    (
                        State::Button1 | State::Button2 | State::Button3,
                        Some(ButtonPressed::Wrong),
                    ) => (),
                    _ => panic!(
                        "overlays should open only with states `Button1`, `Button2`, or `Button3`"
                    ),
                }
                self.button_pressed = None;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let underlay = {
            let body_string = match self.state {
                State::Start => "Press the Start button to begin",
                State::Button1 => "Press Button 1",
                State::Button2 => "Press Button 2",
                State::Button3 => "Press Button 3",
                State::End => "All done!",
            };

            let button = |label, message| {
                button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                    .width(90)
                    .on_press(message)
            };

            let foot_row = match self.state {
                State::Start => Some(row![button("Start", Message::ButtonStartPressed)]),
                State::Button1 | State::Button2 | State::Button3 => Some(row![
                    button("Button 1", Message::Button1Pressed),
                    button("Button 2", Message::Button2Pressed),
                    button("Button 3", Message::Button3Pressed)
                ]),
                State::End => Some(row![
                    button("Restart", Message::ButtonRestartPressed),
                    button("Quit", Message::ButtonQuitPressed),
                ]),
            };

            match foot_row {
                Some(foot_row) => container(
                    column![
                        //vertical_space(Length::Fill),
                        vertical_space(),
                        text(body_string),
                        //vertical_space(Length::Fill),
                        vertical_space(),
                        foot_row.spacing(20)
                    ]
                    .spacing(20)
                    .padding(20)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .center_x(),
                None => container(text(body_string))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y(),
            }
        };

        let overlay = self.button_pressed.as_ref().map(|button_pressed| {
            let head_string = match button_pressed {
                ButtonPressed::Correct => "Correct button",
                ButtonPressed::Wrong => "Wrong button",
            };

            let body_string = match button_pressed {
                ButtonPressed::Correct => match self.state {
                    State::Button1 => "You pressed button 1!\nClose this dialogue to continue.",
                    State::Button2 => "You pressed button 2!\nClose this dialogue to continue.",
                    State::Button3 => "You pressed button 3!\nClose this dialogue to continue.",
                    _ => panic!(
                        "overlays should open only with states `Button1`, `Button2`, or `Button3`"
                    ),
                },
                ButtonPressed::Wrong => {
                    "You pressed the wrong button.\nClose this dialogue to continue."
                }
            };

            let card_style = match button_pressed {
                ButtonPressed::Correct => CardStyles::Success,
                ButtonPressed::Wrong => CardStyles::Danger,
            };

            card(
                text(head_string),
                text(body_string)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .style(card_style)
            .max_width(300.0)
            .on_close(Message::CloseOverlay)
        });

        modal(underlay, overlay)
            .backdrop(Message::CloseOverlay)
            .on_esc(Message::CloseOverlay)
            .into()
    }
}

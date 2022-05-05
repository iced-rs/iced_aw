use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    button, text_input, Button, Column, Container, Element, Length, Row, Text, TextInput,
};
use iced_aw::TabLabel;

use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    LoginPressed,
}

pub struct LoginTab {
    username: String,
    username_state: text_input::State,
    password: String,
    password_state: text_input::State,
    clear_button: button::State,
    login_button: button::State,
}

impl LoginTab {
    pub fn new() -> Self {
        LoginTab {
            username: String::new(),
            username_state: text_input::State::default(),
            password: String::new(),
            password_state: text_input::State::default(),
            clear_button: button::State::default(),
            login_button: button::State::default(),
        }
    }

    pub fn update(&mut self, message: LoginMessage) {
        match message {
            LoginMessage::UsernameChanged(value) => self.username = value,
            LoginMessage::PasswordChanged(value) => self.password = value,
            LoginMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
            }
            LoginMessage::LoginPressed => {}
        }
    }
}

impl Tab for LoginTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())
        TabLabel::IconText(Icon::User.into(), self.title())
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let content: Element<'_, LoginMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextInput::new(
                        &mut self.username_state,
                        "Username",
                        &self.username,
                        LoginMessage::UsernameChanged,
                    )
                    .padding(10)
                    .size(32),
                )
                .push(
                    TextInput::new(
                        &mut self.password_state,
                        "Password",
                        &self.password,
                        LoginMessage::PasswordChanged,
                    )
                    .padding(10)
                    .size(32)
                    .password(),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.clear_button,
                                Text::new("Clear").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(LoginMessage::ClearPressed),
                        )
                        .push(
                            Button::new(
                                &mut self.login_button,
                                Text::new("Login").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(LoginMessage::LoginPressed),
                        ),
                ),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::Login)
    }
}

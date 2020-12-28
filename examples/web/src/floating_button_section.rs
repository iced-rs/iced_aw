
use iced::{button, scrollable, Button, Checkbox, Column, Container, Element, Length, Scrollable, Text};
use iced_aw::{floating_button::Anchor, FloatingButton};
use crate::Section;

pub struct FloatingButtonSection {
    lines: Vec<String>,
    scrollable_state: scrollable::State,
    button_state: button::State,
    hide: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    FloatingButtonPressed,
    HideButton(bool),
}

impl FloatingButtonSection {
    pub fn new() -> Self {
        Self {
            lines: vec!["Hello".into(), "World".into()],
            scrollable_state: scrollable::State::new(),
            button_state: button::State::new(),
            hide: false,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FloatingButtonPressed => {
                self.lines.push("This is a newly added line.".into())
            },
            Message::HideButton(hide) => self.hide = hide,
        }
    }
}

impl Section for FloatingButtonSection {
    
    type Message = crate::Message;

    fn header(&self) -> String {
        String::from("Floating Button")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let column = self.lines.iter()
            .fold(
                Column::new(),
                |col, line| {
                    col.push(Text::new(line.to_owned()))
                }
            )
            .width(Length::Fill);

        let scrollable = Scrollable::new(&mut self.scrollable_state)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_height(100)
            .push(column);

        let container = Container::new(
            FloatingButton::new(
                &mut self.button_state,
                scrollable,
                |state| Button::new(state, Text::new("Press Me!"))
                    .on_press(Message::FloatingButtonPressed)
                    .style(iced_aw::style::button::Primary)
            )
            .anchor(Anchor::SouthEast)
            .offset([20.0, 5.0])
            .hide(self.hide)
        )
        .width(Length::Fill);

        let column: Element<'_, Message> = Column::new()
            .spacing(10)
            .push(
                Checkbox::new(self.hide, "Hide button", Message::HideButton)
            )
            .push(
                container
            ).into();

        column.map(|msg| crate::Message::FloatingButton(msg))
    }

}
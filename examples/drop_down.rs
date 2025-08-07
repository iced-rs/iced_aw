// This example demonstrates how to use the [`DropDown`] widget
//
// It was written by wiiznokes <wiiznokes2@gmail.com>

use std::fmt::Display;

use iced::{
    alignment::Vertical,
    widget::{scrollable, Button, Column, Row, Text},
    Element, Length,
};

use iced_aw::{drop_down, DropDown};

fn main() -> iced::Result {
    iced::application(
        DropDownExample::default,
        DropDownExample::update,
        DropDownExample::view,
    )
    .run()
}

#[derive(Clone, Debug, Default)]
enum Choice {
    #[default]
    Choice1,
    Choice2,
    Choice3,
    Choice4,
    Choice5,
    Choice6,
}

const CHOICES: [Choice; 6] = [
    Choice::Choice1,
    Choice::Choice2,
    Choice::Choice3,
    Choice::Choice4,
    Choice::Choice5,
    Choice::Choice6,
];

#[derive(Clone, Debug)]
enum Message {
    Select(Choice),
    Dismiss,
    Expand,
}

#[derive(Default)]
struct DropDownExample {
    selected: Choice,
    expanded: bool,
}

impl DropDownExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::Select(choice) => {
                self.selected = choice;
                self.expanded = false;
            }
            Message::Dismiss => self.expanded = false,
            Message::Expand => self.expanded = !self.expanded,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let underlay = Row::new()
            .push(Text::new(format!("Selected: {}", self.selected)))
            .push(Button::new(Text::new("expand")).on_press(Message::Expand))
            .spacing(8)
            .align_y(Vertical::Center);

        let options = Column::with_children(CHOICES.map(|choice| {
            Row::new()
                .push(Text::new(choice.to_string()))
                .push(Button::new(Text::new("choose")).on_press(Message::Select(choice)))
                .spacing(6)
                .align_y(Vertical::Center)
                .into()
        }));
        let overlay = scrollable(options.padding([0, 4]));

        let drop_down = DropDown::new(underlay, overlay, self.expanded)
            .width(Length::Fill)
            .on_dismiss(Message::Dismiss)
            .alignment(drop_down::Alignment::Bottom);

        Column::new()
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::Alignment::Center)
            .push(drop_down)
            .into()
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Choice1 => write!(f, "1"),
            Choice::Choice2 => write!(f, "2"),
            Choice::Choice3 => write!(f, "3"),
            Choice::Choice4 => write!(f, "4"),
            Choice::Choice5 => write!(f, "5"),
            Choice::Choice6 => write!(f, "6"),
        }
    }
}

//! Test from [issue #334](https://github.com/iced-rs/iced_aw/issues/334)
//!
//! It is used to test the position/limits of the dropdown overlay, both
//! in a plain column and inside a scrollable container.

use iced::{
    widget::{scrollable, Button, Column, Container, Row, Text},
    Element, Length,
};

use iced_aw::{drop_down, DropDown};

fn main() -> iced::Result {
    iced::application(
        DropDownTest::default,
        DropDownTest::update,
        DropDownTest::view,
    )
    .scale_factor(|_| 1.0)
    .window_size((700.0, 600.0))
    .run()
}

#[derive(Clone, Copy, Debug)]
enum Message {
    Toggle(usize),
    Dismiss(usize),
}

const OPTIONS: [drop_down::Alignment; 8] = [
    drop_down::Alignment::TopStart,
    drop_down::Alignment::Top,
    drop_down::Alignment::TopEnd,
    drop_down::Alignment::End,
    drop_down::Alignment::BottomStart,
    drop_down::Alignment::Bottom,
    drop_down::Alignment::BottomEnd,
    drop_down::Alignment::Start,
];

#[derive(Debug, Default)]
struct DropDownTest {
    expanded: [bool; 8],
    expanded_scroll: [bool; 8],
}

impl DropDownTest {
    fn update(&mut self, message: Message) {
        use Message::*;
        match message {
            Toggle(i) => {
                if i < 8 {
                    self.expanded[i] = !self.expanded[i];
                } else {
                    self.expanded_scroll[i - 8] = !self.expanded_scroll[i - 8];
                }
            }
            Dismiss(i) => {
                if i < 8 {
                    self.expanded[i] = false;
                } else {
                    self.expanded_scroll[i - 8] = false;
                }
            }
        }
    }

    fn dropdowns<'a>(expanded: &[bool; 8], offset: usize) -> Vec<Element<'a, Message>> {
        let underlay = |i: usize, opt: &drop_down::Alignment| {
            Row::new().push(
                Button::new(Text::new(format!("{opt:?}"))).on_press(Message::Toggle(i + offset)),
            )
        };

        let overlay = || {
            let options = Column::with_children((1..=60).map(|i| {
                Row::new()
                    .push(Text::new(format!("{i:02}=====================")))
                    .into()
            }))
            .padding([0, 16]);
            scrollable(options)
        };

        OPTIONS
            .iter()
            .enumerate()
            .map(|(i, alignment)| {
                DropDown::new(underlay(i, alignment), overlay(), expanded[i])
                    .width(Length::Fill)
                    .on_dismiss(Message::Dismiss(i + offset))
                    .alignment(alignment.clone())
                    .into()
            })
            .collect()
    }

    fn view(&self) -> Element<'_, Message> {
        let plain = Column::with_children(Self::dropdowns(&self.expanded, 0))
            .padding(20)
            .spacing(20)
            .width(Length::Fill);

        let inside_scrollable = Column::with_children(Self::dropdowns(&self.expanded_scroll, 8))
            .padding(20)
            .spacing(20)
            .width(Length::Fill);

        let right = scrollable(inside_scrollable).width(Length::Fill);

        Row::new()
            .push(Container::new(plain).width(Length::FillPortion(1)))
            .push(Container::new(right).width(Length::FillPortion(1)))
            .into()
    }
}

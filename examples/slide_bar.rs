// This example demonstrates how to use the slide bar widget
//
// It was written by Andrew Wheeler <genusistimelord@gmail.com>

use iced::{
    widget::{Column, Container, Text},
    Element, Length,
};

use iced_aw::{SlideBar, ICED_AW_FONT_BYTES};

fn main() -> iced::Result {
    iced::application(
        SlideBarExample::default,
        SlideBarExample::update,
        SlideBarExample::view,
    )
    .title(SlideBarExample::title)
    .font(ICED_AW_FONT_BYTES)
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    SliderBarChange(u32),
}

#[derive(Debug, Default)]
struct SlideBarExample {
    value: u32,
}

impl SlideBarExample {
    fn title(&self) -> String {
        String::from("Slider Bar example")
    }

    fn update(&mut self, message: Message) {
        let Message::SliderBarChange(v) = message;
        self.value = v;
    }

    fn view(&self) -> Element<'_, Message> {
        let bar = SlideBar::new(0..=100, self.value, Message::SliderBarChange).width(100.0);

        let content_all = Column::new()
            .spacing(10)
            .push(
                Text::new(format!("Value is {}", self.value))
                    .width(Length::Fill)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(bar)
            .align_x(iced::Alignment::Center);

        Container::new(content_all)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

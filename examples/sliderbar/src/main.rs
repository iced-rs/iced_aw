use iced::{
    widget::{Column, Container, Text},
    Element, Length,
};

use iced_aw::SlideBar;

fn main() -> iced::Result {
    iced::program(
        "Slider Bar example",
        SlideBarExample::update,
        SlideBarExample::view,
    )
    .font(iced_aw::BOOTSTRAP_FONT_BYTES)
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
    fn update(&mut self, message: Message) {
        let Message::SliderBarChange(v) = message;
        self.value = v;
    }

    fn view(&self) -> Element<Message> {
        let bar = SlideBar::new(0..=100, self.value, Message::SliderBarChange).width(100.0);

        let content_all = Column::new()
            .spacing(10)
            .push(
                Text::new(format!("Value is {}", self.value))
                    .width(Length::Fill)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .push(bar)
            .align_items(iced::Alignment::Center);

        Container::new(content_all)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

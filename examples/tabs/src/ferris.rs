use iced::{
    widget::{Column, Container, Image, Slider, Text},
    Alignment, Element, Length,
};
use iced_aw::tab_bar::TabLabel;

use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum FerrisMessage {
    ImageWidthChanged(u16),
}

pub struct FerrisTab {
    ferris_width: u16,
}

impl FerrisTab {
    pub fn new() -> Self {
        FerrisTab { ferris_width: 100 }
    }

    pub fn update(&mut self, message: FerrisMessage) {
        match message {
            FerrisMessage::ImageWidthChanged(value) => self.ferris_width = value,
        }
    }
}

impl Tab for FerrisTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Ferris")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Heart.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, FerrisMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(Text::new(if self.ferris_width == 500 {
                    "Hugs!!!"
                } else {
                    "Pull me closer!"
                }))
                .push(ferris(self.ferris_width))
                .push(Slider::new(
                    100..=500,
                    self.ferris_width,
                    FerrisMessage::ImageWidthChanged,
                )),
        )
        .into();

        content.map(Message::Ferris)
    }
}

fn ferris<'a>(width: u16) -> Container<'a, FerrisMessage> {
    Container::new(if cfg!(target_carch = "wasm32") {
        Image::new("images/ferris.png")
    } else {
        Image::new(format!("{}/images/ferris.png", env!("CARGO_MANIFEST_DIR")))
            .width(Length::Units(width))
    })
    .width(Length::Fill)
    .center_x()
}

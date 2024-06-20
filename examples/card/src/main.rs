use iced::{
    widget::{Button, Column, Container, Scrollable, Text},
    Element, Length,
};

use iced_aw::{helpers::card, style};

fn main() -> iced::Result {
    iced::application("Card example", CardExample::update, CardExample::view)
        .font(iced_aw::BOOTSTRAP_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    CloseCard,
    OpenCard,
}

#[derive(Debug, Default)]
struct CardExample {
    card_open: bool,
}

impl CardExample {
    fn update(&mut self, message: self::Message) {
        match message {
            Message::CloseCard | Message::OpenCard => {
                self.card_open = !self.card_open;
            }
        }
    }

    fn view(&self) -> Element<'_, self::Message> {
        let element: Element<'_, Message> = if self.card_open {
            card(
                        Text::new("Head X"),
                        Column::new()
                            .push(Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead."))
                    )
                    .foot(Text::new("Foot"))
                    .style(style::card::primary)
                    .on_close(Message::CloseCard)
                    .into()
        } else {
            Button::new(Text::new("Open card"))
                .on_press(Message::OpenCard)
                .into()
        };

        let content = Scrollable::new(element);

        Container::new(Column::new().push(content).max_width(600))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

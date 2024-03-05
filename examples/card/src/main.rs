use iced::{
    alignment, font,
    theme::Theme,
    widget::{container, text, Button, Column, Container, Scrollable, Text},
    Application, Command, Element, Length, Settings,
};
use iced_aw::{helpers::card, style::CardStyles};

fn main() -> iced::Result {
    CardExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    CloseCard,
    OpenCard,
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum CardExample {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    card_open: bool,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for CardExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CardExample, Command<Message>) {
        (
            CardExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Card example")
    }

    fn update(&mut self, message: self::Message) -> Command<Message> {
        match self {
            CardExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = CardExample::Loaded(State { card_open: false })
                }
            }
            CardExample::Loaded(State { card_open }) => match message {
                Message::CloseCard | Message::OpenCard => {
                    *card_open = !*card_open;
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, self::Message> {
        match self {
            CardExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            CardExample::Loaded(State { card_open }) => {
                let element: Element<'_, Message> = if *card_open {
                    card(
                        Text::new("Head X"),
                        Column::new()
                            .push(Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead."))
                    )
                    .foot(Text::new("Foot"))
                    .style(CardStyles::Primary)
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
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}

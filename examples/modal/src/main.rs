use iced::{
    alignment::{self, Horizontal},
    font,
    widget::{container, pick_list, text, Button, Container, Row, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

use iced_aw::{modal, Card};

fn main() -> iced::Result {
    ModalExample::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    OpenModal,
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
    ChangeTheme(Theme),
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum ModalExample {
    Loading,
    Loaded(State),
}

#[derive(Default, Debug)]
struct State {
    show_modal: bool,
    last_message: Option<Message>,
    theme: Theme,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for ModalExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ModalExample, Command<Message>) {
        (
            ModalExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Modal example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            ModalExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = ModalExample::Loaded(State::default())
                }
            }
            ModalExample::Loaded(state) => {
                match message {
                    Message::OpenModal => state.show_modal = true,
                    Message::CloseModal => state.show_modal = false,
                    Message::CancelButtonPressed => state.show_modal = false,
                    Message::OkButtonPressed => state.show_modal = false,
                    Message::ChangeTheme(ref t) => state.theme = t.clone(),
                    _ => {}
                }

                state.last_message = Some(message)
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            ModalExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            ModalExample::Loaded(state) => {
                let underlay = Container::new(
                    Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(Button::new(Text::new("Open modal!")).on_press(Message::OpenModal))
                        .push(pick_list(
                            Theme::ALL,
                            Some(&state.theme),
                            Message::ChangeTheme,
                        ))
                        .push(Text::new(format!(
                            "Last message: {}",
                            match state.last_message.as_ref() {
                                Some(message) => match message {
                                    Message::OpenModal => "Modal opened",
                                    Message::CloseModal => "Modal closed",
                                    Message::CancelButtonPressed => "Modal canceled",
                                    Message::OkButtonPressed => "Modal accepted",
                                    Message::ChangeTheme(_) => "Changed Theme",
                                    _ => "None",
                                },
                                None => "None",
                            }
                        ))),
                );

                let overlay = if state.show_modal {
                    Some(
                        Card::new(
                            Text::new("My modal"),
                            Text::new("This is a modal!"), //Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit​​, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead.")
                        )
                        .foot(
                            Row::new()
                                .spacing(10)
                                .padding(5)
                                .width(Length::Fill)
                                .push(pick_list(
                                    Theme::ALL,
                                    Some(&state.theme),
                                    Message::ChangeTheme,
                                ))
                                .push(
                                    Button::new(
                                        Text::new("Cancel")
                                            .horizontal_alignment(Horizontal::Center),
                                    )
                                    .width(Length::Fill)
                                    .on_press(Message::CancelButtonPressed),
                                )
                                .push(
                                    Button::new(
                                        Text::new("Ok").horizontal_alignment(Horizontal::Center),
                                    )
                                    .width(Length::Fill)
                                    .on_press(Message::OkButtonPressed),
                                ),
                        )
                        .max_width(500.0)
                        //.width(Length::Shrink)
                        .on_close(Message::CloseModal),
                    )
                } else {
                    None
                };

                modal(underlay, overlay)
                    .backdrop(Message::CloseModal)
                    .on_esc(Message::CloseModal)
                    .align_y(alignment::Vertical::Center)
                    .into()
            }
        }
    }

    fn theme(&self) -> Theme {
        match self {
            ModalExample::Loading => Theme::Light,
            ModalExample::Loaded(state) => state.theme.clone(),
        }
    }
}

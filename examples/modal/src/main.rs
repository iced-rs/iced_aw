use iced::{
    alignment::{Alignment, Horizontal},
    button, Button, Container, Element, Length, Row, Sandbox, Settings, Text,
};

use iced_aw::{modal, Card, Modal};

fn main() -> iced::Result {
    ModalExample::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    OpenModal,
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
}

#[derive(Default)]
struct ModalExample {
    open_state: button::State,
    modal_state: modal::State<ModalState>,
    last_message: Option<Message>,
}

#[derive(Default)]
struct ModalState {
    cancel_state: button::State,
    ok_state: button::State,
}

impl Sandbox for ModalExample {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Modal example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenModal => self.modal_state.show(true),
            Message::CloseModal => self.modal_state.show(false),
            Message::CancelButtonPressed => self.modal_state.show(false),
            Message::OkButtonPressed => self.modal_state.show(false),
        }
        self.last_message = Some(message)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Container::new(
            Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(
                    Button::new(&mut self.open_state, Text::new("Open modal!"))
                        .on_press(Message::OpenModal),
                )
                .push(Text::new(format!(
                    "Last message: {}",
                    match self.last_message.as_ref() {
                        Some(message) => match message {
                            Message::OpenModal => "Modal opened",
                            Message::CloseModal => "Modal closed",
                            Message::CancelButtonPressed => "Modal canceled",
                            Message::OkButtonPressed => "Modal accepted",
                        },
                        None => "None",
                    }
                ))),
        );

        Modal::new(&mut self.modal_state, content, |state| {
            Card::new(
                Text::new("My modal"),
                Text::new("This is a modal!"), //Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit​​, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead.")
            )
            .foot(
                Row::new()
                    .spacing(10)
                    .padding(5)
                    .width(Length::Fill)
                    .push(
                        Button::new(
                            &mut state.cancel_state,
                            Text::new("Cancel").horizontal_alignment(Horizontal::Center),
                        )
                        .width(Length::Fill)
                        .on_press(Message::CancelButtonPressed),
                    )
                    .push(
                        Button::new(
                            &mut state.ok_state,
                            Text::new("Ok").horizontal_alignment(Horizontal::Center),
                        )
                        .width(Length::Fill)
                        .on_press(Message::OkButtonPressed),
                    ),
            )
            .max_width(300)
            //.width(Length::Shrink)
            .on_close(Message::CloseModal)
            .into()
        })
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal)
        .into()
    }
}

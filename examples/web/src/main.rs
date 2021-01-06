use iced::{
    button, scrollable, Button, Column, Container, Element,
    HorizontalAlignment, Length, Sandbox, Scrollable, Settings, Text
};

use iced_aw::{modal, Card, Modal};

mod badge_section;
use badge_section::BadgeSection;

mod floating_button_section;
use floating_button_section::FloatingButtonSection;

mod card_section;
use card_section::CardSection;

mod modal_section;
use modal_section::ModalSection;

mod date_picker_section;
use date_picker_section::DatePickerSection;

mod time_picker_section;
use time_picker_section::TimePickerSection;

//mod picklist_section;
//use picklist_section::PickListSection;

const HEADER_SIZE: u16 = 42;

fn main() -> iced::Result {
    Web::run(Settings::default())
}

struct Web {
    scrollable_state: scrollable::State,
    primary_modal_state: modal::State<PrimaryModalState>,
    badge_section: BadgeSection,
    floating_button_section: FloatingButtonSection,
    card_section: CardSection,
    modal_section: ModalSection,
    date_picker_section: DatePickerSection,
    //picklist_section: PickListSection,
    time_picker_section: TimePickerSection,
}

#[derive(Clone, Debug)]
enum Message {
    ClosePrimaryModal,
    OpenPrimaryModal,
    FloatingButton(floating_button_section::Message),
    Card(card_section::Message),
    DatePicker(date_picker_section::Message),
    //PickList(picklist_section::Message),
    TimePicker(time_picker_section::Message),
}

impl Sandbox for Web {

    type Message = Message;
    
    fn new() -> Self {
        Self {
            scrollable_state: scrollable::State::new(),
            primary_modal_state: modal::State::new(PrimaryModalState::default()),
            badge_section: BadgeSection::new(),
            floating_button_section: FloatingButtonSection::new(),
            card_section: CardSection::new(),
            modal_section: ModalSection::new(),
            date_picker_section: DatePickerSection::new(),
            //picklist_section: PickListSection::new(),
            time_picker_section: TimePickerSection::new(),
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ClosePrimaryModal => self.primary_modal_state.show(false),
            Message::OpenPrimaryModal => self.primary_modal_state.show(true),
            Message::FloatingButton(msg) => self.floating_button_section.update(msg),
            Message::Card(msg) => self.card_section.update(msg),
            Message::DatePicker(msg) => self.date_picker_section.update(msg),
            //Message::PickList(msg) => self.picklist_section.update(msg),
            Message::TimePicker(msg) => self.time_picker_section.update(msg),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Scrollable::new(&mut self.scrollable_state)
            .spacing(20)
            .max_width(600)
            .push(self.badge_section.view())
            .push(self.floating_button_section.view())
            .push(self.card_section.view())
            .push(self.modal_section.view())
            .push(self.date_picker_section.view())
            //.push(self.picklist_section.view())
            .push(self.time_picker_section.view())
            ;

        let container = Container::new(
            // Workaround: https://github.com/hecrj/iced/issues/643
            Column::new()
                .push(content)
                .max_width(600)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        Modal::new(
            &mut self.primary_modal_state,
            container,
            primary_modal,
        )
        .backdrop(Message::ClosePrimaryModal)
        .into()
    }
}

trait Section {
    type Message: 'static;

    fn header(&self) -> String;

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .spacing(10)
            .push(
                Text::new(format!("{}:", self.header()))
                    .size(HEADER_SIZE)
            )
            .push(
                self.content()
            )
            .into()
    }

    fn content(&mut self) -> Element<'_, Self::Message>;
}

#[derive(Default)]
struct PrimaryModalState {
    ok_button: button::State,
}

fn primary_modal<'a>(state: &'a mut PrimaryModalState) -> Element<'a, Message> {
    Card::new(
        Text::new("Modal"),
        Text::new("This is a modal using the Card widget with its primary color style.")
    )
    .foot(
        Button::new(
            &mut state.ok_button,
            Text::new("Ok")
                .horizontal_alignment(HorizontalAlignment::Center)
                .width(Length::Fill)
        )
        .on_press(Message::ClosePrimaryModal)
        //.style(iced_aw::style::button::Secondary)
        .width(Length::Fill)
    )
    .max_width(300)
    .style(iced_aw::style::card::Primary)
    .into()
}

use iced::{pick_list, Column, Length, Text,
    PickList, Align, Element};
use crate::Section;

pub struct PickListSection {
    pick_list: pick_list::State<Language>,
    selected_language: Language,
}

impl PickListSection {
    pub fn new() -> Self {
        Self {
            pick_list: pick_list::State::default(),
            selected_language: Language::Haskell,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::LanguageSelected(language) => self.selected_language = language,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    LanguageSelected(Language),
}

impl Section for PickListSection {
    
    type Message = crate::Message;

    fn header(&self) -> String {
        String::from("Picklist")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let pick_list = PickList::new(
            &mut self.pick_list,
            &Language::ALL[..],
            Some(self.selected_language),
            Message::LanguageSelected,
        );

        let column: Element<'_, Message> = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(5)
            .push(
                Text::new("Which is your favorite language?")
            )
            .push(
                pick_list
            ).into();

        column.map(|msg| crate::Message::PickList(msg))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Javascript,
    Other,
}

impl Language {
    const ALL: [Language; 7] = [
        Language::C,
        Language::Elm,
        Language::Ruby,
        Language::Haskell,
        Language::Rust,
        Language::Javascript,
        Language::Other,
    ];
}

impl Default for Language {
    fn default() -> Language {
        Language::Rust
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Elm => "Elm",
                Language::Ruby => "Ruby",
                Language::Haskell => "Haskell",
                Language::C => "C",
                Language::Javascript => "Javascript",
                Language::Other => "Some other language",
            }
        )
    }
}
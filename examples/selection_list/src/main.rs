use iced::{
    widget::{Column, Container, Space, Text},
    Alignment, Element, Length, Sandbox, Settings,
};
use iced_aw::selection_list::{SelectionList, SelectionListStyles};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    vec: Vec<String>,
    selected_language: String,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageSelected(String),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        let mut vec = Vec::with_capacity(10);

        for i in Language::ALL.iter() {
            vec.push(format!("{:?}", i))
        }

        Self {
            vec,
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("Selection list - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = language.clone();

                if language == "Rust" {
                    self.vec.push("Rusty".into());
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let selection_list = SelectionList::new_with(
            &self.vec[..],
            Message::LanguageSelected,
            12,
            5,
            SelectionListStyles::Default,
        )
        .width(Length::Shrink)
        .height(Length::Units(100));

        let mut content = Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(selection_list)
            .push(Text::new("Which is your favorite language?"))
            .push(Text::new(format!("{:?}", self.selected_language)));

        content = content.push(Space::with_height(Length::Units(600)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
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

use iced::{
    widget::{button, Column, Container, Space, Text},
    Alignment, Element, Font, Length, Sandbox, Settings,
};
use iced_aw::{selection_list::SelectionList, SelectionListStyles};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    vec: Vec<String>,
    selected_language: String,
    selected_index: usize,
    manual_select: Option<usize>,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageSelected((usize, String)),
    AddAtSelection,
    ManualSelection,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        let mut vec = Vec::with_capacity(10);

        for i in Language::ALL.iter() {
            vec.push(format!("{i}"))
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
            Message::LanguageSelected((index, language)) => {
                self.selected_language = language;
                self.selected_index = index;
                self.manual_select = None;

                if self.selected_language == "Rust" {
                    self.vec.push("Rusty".into());
                }
            }
            Message::AddAtSelection => {
                self.vec
                    .insert(self.selected_index, "Java OH NOES!".to_owned());
                self.selected_language.clear();
                self.manual_select = None;
            }
            Message::ManualSelection => {
                if let Some(option) = self.vec.get(0) {
                    self.selected_language = option.to_owned();
                    self.selected_index = 0;
                    self.manual_select = Some(0);
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let selection_list = SelectionList::new_with(
            &self.vec[..],
            Message::LanguageSelected,
            12.0,
            5.0,
            SelectionListStyles::Default,
            self.manual_select,
            Font::default(),
        )
        .width(Length::Shrink)
        .height(Length::Fixed(100.0));

        let mut content = Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(selection_list)
            .push(Text::new("Which is your favorite language?"))
            .push(Text::new(format!("{:?}", self.selected_language)))
            .push(button("press to add at selection").on_press(Message::AddAtSelection))
            .push(button("Manual select Index 2").on_press(Message::ManualSelection));

        content = content.push(Space::with_height(Length::Fixed(800.0)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
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

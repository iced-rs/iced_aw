use iced::{
    widget::{button, Column, Container, Text},
    Alignment, Element, Font, Length,
};
use iced_aw::{selection_list::SelectionList, style::selection_list::primary};

pub fn main() -> iced::Result {
    iced::application("Selection list example", Example::update, Example::view)
        .font(iced_aw::BOOTSTRAP_FONT_BYTES)
        .run()
}

struct Example {
    vec: Vec<String>,
    selected_language: String,
    selected_index: usize,
    manual_select: Option<usize>,
}

impl Default for Example {
    fn default() -> Self {
        let mut vec = Vec::with_capacity(10);

        for i in Language::ALL.iter() {
            vec.push(i.name())
        }

        Self {
            vec,
            selected_language: "".to_string(),
            selected_index: 0,
            manual_select: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    LanguageSelected(usize, String),
    AddAtSelection,
    ManualSelection,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::LanguageSelected(index, language) => {
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
                if let Some(option) = self.vec.get(2) {
                    option.clone_into(&mut self.selected_language);
                    self.selected_index = 2;
                    self.manual_select = Some(2);
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
            primary,
            self.manual_select,
            Font::default(),
        )
        .width(Length::Shrink)
        .height(Length::Fixed(100.0));

        let content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .spacing(10)
            .push(selection_list)
            .push(Text::new("Which is your favorite language?"))
            .push(Text::new(format!("{:?}", self.selected_language)))
            .push(button("press to add at selection").on_press(Message::AddAtSelection))
            .push(button("Manual select Index 2").on_press(Message::ManualSelection));

        //content = content.push(Space::with_height(Length::Fixed(400.0)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
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

    pub fn name(&self) -> String {
        match self {
            Language::Rust => "Rust".to_owned(),
            Language::Elm => "Elm".to_owned(),
            Language::Ruby => "Ruby".to_owned(),
            Language::Haskell => "Haskell".to_owned(),
            Language::C => "C".to_owned(),
            Language::Javascript => "Javascript".to_owned(),
            Language::Other => "Some other language".to_owned(),
        }
    }
}

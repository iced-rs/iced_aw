use iced::{
    widget::{Button, Column, Row, Text, TextInput},
    Alignment, Element, Length, Sandbox, Settings,
};
use iced_aw::{TabBar, TabLabel};

fn main() -> iced::Result {
    TabBarExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(usize),
    TabClosed(usize),
    TabLabelInputChanged(String),
    TabContentInputChanged(String),
    NewTab,
}

struct TabBarExample {
    active_tab: usize,
    new_tab_label: String,
    new_tab_content: String,
    tabs: Vec<(String, String)>,
}

impl Sandbox for TabBarExample {
    type Message = Message;

    fn new() -> Self {
        TabBarExample {
            active_tab: 0,
            new_tab_label: String::new(),
            new_tab_content: String::new(),
            tabs: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("TabBar example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(index) => {
                println!("Tab selected: {}", index);
                self.active_tab = index
            }
            Message::TabClosed(index) => {
                self.tabs.remove(index);
                println!("active tab before: {}", self.active_tab);
                self.active_tab = if self.tabs.is_empty() {
                    0
                } else {
                    usize::max(0, usize::min(self.active_tab, self.tabs.len() - 1))
                };
                println!("active tab after: {}", self.active_tab);
            }
            Message::TabLabelInputChanged(value) => self.new_tab_label = value,
            Message::TabContentInputChanged(value) => self.new_tab_content = value,
            Message::NewTab => {
                println!("New");
                if !self.new_tab_label.is_empty() && !self.new_tab_content.is_empty() {
                    println!("Create");
                    self.tabs.push((
                        self.new_tab_label.to_owned(),
                        self.new_tab_content.to_owned(),
                    ));
                    self.new_tab_label.clear();
                    self.new_tab_content.clear();
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(
                Row::new()
                    .push(
                        TextInput::new("Tab label", &self.new_tab_label)
                            .on_input(Message::TabLabelInputChanged)
                            .size(22)
                            .padding(5.0),
                    )
                    .push(
                        TextInput::new("Tab content", &self.new_tab_content)
                            .on_input(Message::TabContentInputChanged)
                            .size(22)
                            .padding(5.0),
                    )
                    .push(Button::new(Text::new("New")).on_press(Message::NewTab))
                    .align_items(Alignment::Center)
                    .padding(10.0)
                    .spacing(5.0),
            )
            .push(
                self.tabs
                    .iter()
                    .fold(
                        TabBar::new(Message::TabSelected),
                        |tab_bar, (tab_label, _)| {
                            // manually create a new index for the new tab
                            // starting from 0, when there is no tab created yet
                            let idx = tab_bar.size();
                            tab_bar.push(idx, TabLabel::Text(tab_label.to_owned()))
                        },
                    )
                    .on_close(Message::TabClosed)
                    .tab_width(Length::Shrink)
                    .spacing(5.0)
                    .padding(5.0)
                    .text_size(32.0),
            )
            .push(
                if let Some((_, content)) = self.tabs.get(self.active_tab) {
                    Text::new(content)
                } else {
                    Text::new("Please create a new tab")
                }
                .size(25),
            )
            .into()
    }
}

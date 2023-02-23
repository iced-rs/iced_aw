use iced::widget::{button, container, text};
use iced::{Element, Length, Sandbox, Settings};
use iced_aw::{floating_element::Anchor, FloatingElement};

fn main() -> iced::Result {
    FloatingElementAnchorsExample::run(Settings::default())
}

struct FloatingElementAnchorsExample {
    current_anchor: usize,
}

#[derive(Clone, Debug)]
enum Message {
    NextAnchor,
}

const AVAILABLE_ANCHORS: [Anchor; 8] = [
    Anchor::North,
    Anchor::NorthEast,
    Anchor::East,
    Anchor::SouthEast,
    Anchor::South,
    Anchor::SouthWest,
    Anchor::West,
    Anchor::NorthWest,
];

impl Sandbox for FloatingElementAnchorsExample {
    type Message = Message;

    fn new() -> Self {
        Self { current_anchor: 0 }
    }

    fn title(&self) -> String {
        String::from("Floating element anchors")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NextAnchor => {
                self.current_anchor = (self.current_anchor + 1) % 8;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let current_anchor = AVAILABLE_ANCHORS[self.current_anchor];
        let current_anchor_name = match current_anchor {
            Anchor::NorthWest => "North West",
            Anchor::NorthEast => "North East",
            Anchor::SouthWest => "South West",
            Anchor::SouthEast => "South East",
            Anchor::North => "North",
            Anchor::East => "East",
            Anchor::South => "South",
            Anchor::West => "West",
        };

        let content = container(button(text(current_anchor_name)).on_press(Message::NextAnchor))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(8);

        FloatingElement::new(content, || text("Content").into())
            .anchor(current_anchor)
            .hide(false)
            .into()
    }
}

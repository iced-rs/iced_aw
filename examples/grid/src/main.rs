use iced::{
    theme,
    widget::{Button, Column, Container, Scrollable, Text},
    Alignment, Color, Element, Length, Sandbox, Settings,
};

use iced_aw::Grid;

// Number of columns for the grid
const COLUMNS: usize = 2;

fn main() -> iced::Result {
    GridExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    AddElement,
}

struct GridExample {
    element_index: usize,
}

impl Sandbox for GridExample {
    type Message = Message;

    fn new() -> Self {
        GridExample { element_index: 0 }
    }

    fn title(&self) -> String {
        String::from("Grid example")
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::AddElement => {
                self.element_index += 1;
            }
        }
    }

    fn view(&self) -> Element<'_, self::Message> {
        // Creates a grid with two columns
        let mut grid = Grid::with_columns(COLUMNS)
            .push(Text::new("Column 1").style(theme::Text::Color(Color::from_rgb8(255, 0, 0))))
            .push(Text::new("Column 2").style(theme::Text::Color(Color::from_rgb8(255, 0, 0))));

        // Add elements to the grid
        for i in 0..self.element_index {
            grid.insert(Text::new(format!("Row {} Element {}", (i / COLUMNS), i)));
        }

        let add_button: Element<'_, Message> = Button::new(Text::new("Add element"))
            .on_press(Message::AddElement)
            .into();

        let column: Element<'_, Message> = Column::new()
            .spacing(15)
            .max_width(600)
            .align_items(Alignment::Center)
            .push(grid)
            .push(add_button)
            .into();

        let content = Scrollable::new(column);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}

use iced::Length;
use iced_aw::{Menu, MenuBar, menu, menu_items};
use iced_widget::{button, container, text};

#[derive(Default)]
pub struct App;

#[derive(Clone, Copy)]
pub enum Message {
    NoOp,
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}

impl App {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::NoOp => println!("Hello"),
        }
    }

    pub fn title(&self) -> String {
        "hello".into()
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let file_menu = Menu::new(menu_items!(
            (button("New").on_press(Message::NoOp)),
            (button("Open").on_press(Message::NoOp)),
            (button("Exit").on_press(Message::NoOp)),
        ))
        .width(Length::Fill);

        let edit_menu = Menu::new(menu_items!(
            (button("Copy").on_press(Message::NoOp)),
            (button("Paste").on_press(Message::NoOp)),
        ))
        .width(Length::Shrink);

        let about_menu =
            Menu::new(menu_items!((button("About").on_press(Message::NoOp)))).width(Length::Shrink);

        let menu_bar = MenuBar::new(menu_items!(
            (container(text("File")), file_menu),
            (container(text("Edit")), edit_menu),
            (container(text("Help")), about_menu),
        ))
        .draw_path(menu::DrawPath::Backdrop);

        menu_bar.into()
    }
}

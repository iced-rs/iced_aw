//use iced::{
use iced_web::{
    executor, Align, Application, Command, Element, Row, Text
};

use iced_aw::{Badge};

fn main() {
    Web::run(())
}

struct Web {

}

#[derive(Debug, Clone)]
enum Message {

}

impl Application for Web {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Web, Command<Self::Message>) {
        (Web {  }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {

        Row::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(
                Text::new("Hello World!")
            )
            .push(
                Badge::new(Text::new("Default"))
                    .style(iced_aw::style::badge::Success)
            )
            .into()

        
    }
}
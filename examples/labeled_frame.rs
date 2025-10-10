fn view(_: &u8) -> iced::Element<'_, ()> {
    iced::widget::container(
        iced_aw::widget::LabeledFrame::new(
            "Title", 
            "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."
        )
    ).padding(100).center(iced::Fill).into()
}

fn main() {
    iced::application(|| 0, |_: &mut u8, _: ()| {}, view)
        .title(title)
        .theme(theme)
        .run()
        .unwrap()
}

fn title(_: &u8) -> String {
    String::from("labeled_frame example")
}

fn theme(_: &u8) -> iced::Theme {
    iced::Theme::Light
}

use iced::{
    Alignment, Element,
    Length::{Fill, FillPortion, Shrink},
    widget::{button, column, container, row, scrollable, text},
};
use iced_aw::{DropDown, drop_down};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}

#[derive(Default, Debug, Clone)]
enum DropDownItem {
    #[default]
    A,
    B,
    C,
}

impl std::fmt::Display for DropDownItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DropDownItem::A => write!(f, "A"),
            DropDownItem::B => write!(f, "B"),
            DropDownItem::C => write!(f, "C"),
        }
    }
}

impl DropDownItem {
    const ALL: &'static [Self] = &[DropDownItem::A, DropDownItem::B, DropDownItem::C];
}

#[derive(Default)]
struct App {
    is_open: bool,
    selected: DropDownItem,
}

#[derive(Debug, Clone)]
enum Message {
    DropdownOpen,
    DropdownSelect(DropDownItem),
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        row![
            container(column![
                container(
                    column![text("Placeholder").size(18), text("text").size(12)]
                        .spacing(2)
                        .width(Fill)
                        .align_x(Alignment::Center)
                )
                .padding(4),
                column![
                    row![
                        text("Placeholder"),
                        button("scaffold").padding(1).height(Shrink)
                    ],
                    scrollable(text("Lorem")).height(Fill)
                ]
            ])
            .width(FillPortion(1)),
            column![
                row![
                    text("Dropdowns"),
                    prepare_dropdown(self.selected.clone(), self.is_open)
                ]
                .spacing(4)
                .padding(5),
                scrollable("Lorem").height(Fill)
            ]
            .width(FillPortion(2))
        ]
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DropdownOpen => self.is_open = !self.is_open,
            Message::DropdownSelect(drop_down_item) => self.selected = drop_down_item,
        }
    }
}

fn prepare_dropdown<'a>(selected: DropDownItem, expanded: bool) -> DropDown<'a, Message> {
    let underlay = button(
        row![
            text(selected.to_string()).width(90),
            if expanded { text("V") } else { text("<") }
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Message::DropdownOpen);
    let overlay = container(column(DropDownItem::ALL.iter().map(|r| {
        button(text(r.to_string()).size(18).wrapping(text::Wrapping::None))
            .width(Fill)
            .on_press(Message::DropdownSelect(r.clone()))
            .into()
    })));
    DropDown::new(underlay, overlay, expanded)
        .alignment(drop_down::Alignment::Bottom)
        .on_dismiss(Message::DropdownOpen)
}

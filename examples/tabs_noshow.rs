use iced::{
    Element, Length, Task,
    widget::{button, column, container, stack, text},
};
use iced_aw::{
    sidebar::{SidebarPosition, TabLabel},
    widget::SidebarWithContent,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum TabId {
    #[default]
    One,
    Two,
}

impl TabId {
    const ALL: [Self; 2] = [Self::One, Self::Two];
}

impl fmt::Display for TabId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => f.write_str("One"),
            Self::Two => f.write_str("Two"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleError,
    TabSelected(TabId),
}

#[derive(Default)]
struct App {
    active_tab: TabId,
    show_error: bool,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleError => {
                self.show_error = !self.show_error;
            }
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let mut sidebar = SidebarWithContent::new(Message::TabSelected);
        for tab in TabId::ALL {
            let label = TabLabel::Text(tab.to_string());
            let view: Element<'_, Message> = column![
                text(format!("Tab: {tab}")),
                button("Toggle Error").on_press(Message::ToggleError)
            ]
            .spacing(10)
            .padding(10)
            .into();
            sidebar = sidebar.push(tab, label, view);
        }

        let content: Element<'_, Message> = sidebar
            .set_active_tab(&self.active_tab)
            .sidebar_position(SidebarPosition::Start)
            .into();

        if self.show_error {
            let overlay = container(text("Error"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill);
            stack![content, overlay].into()
        } else {
            // This change in root widget type causes a panic in iced_aw::SidebarWithContent
            content
        }
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}

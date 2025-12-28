//! Integration tests for the ColorPicker widget
//!
//! These tests verify the ColorPicker widget's behavior by actually exercising
//! the widget through the iced test framework.
//!
//! Note: ColorPicker has known thread-safety issues with overlays in the test
//! framework. These tests are minimal to avoid segfaults in parallel execution.

use iced::{Color, Element, Settings};
use iced_aw::ColorPicker;
use iced_test::Simulator;
use iced_widget::button;
use iced_widget::text::Text;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Open,
    Cancel,
    Submit(Color),
}

type ViewFn = Box<dyn Fn() -> Element<'static, Message>>;

#[derive(Clone)]
struct App {
    view_fn: std::rc::Rc<ViewFn>,
}

impl App {
    fn new<F>(view_fn: F) -> (Self, iced::Task<Message>)
    where
        F: Fn() -> Element<'static, Message> + 'static,
    {
        (
            App {
                view_fn: std::rc::Rc::new(Box::new(view_fn)),
            },
            iced::Task::none(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        (self.view_fn)()
    }
}

/// Helper to run a test with a given view
fn run_test<F>(view_fn: F)
where
    F: Fn() -> Element<'static, Message> + 'static,
{
    let (app, _) = App::new(view_fn);
    let _ui = Simulator::with_settings(Settings::default(), app.view());
    // The widget is successfully rendered if we get here without panicking
}

#[test]
fn color_picker_closed_renders_underlay_button() {
    let (app, _) = App::new(|| {
        ColorPicker::new(
            false, // Picker closed
            Color::from_rgb(0.5, 0.5, 0.5),
            button(Text::new("Open Picker")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });

    let mut ui = Simulator::with_settings(Settings::default(), app.view());

    // Verify the underlay button is visible when picker is closed
    assert!(
        ui.find("Open Picker").is_ok(),
        "Underlay button should be visible when picker is closed"
    );
}

#[test]
fn color_picker_closed_state_renders() {
    run_test(|| {
        ColorPicker::new(
            false, // Closed
            Color::from_rgb(0.5, 0.5, 0.5),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

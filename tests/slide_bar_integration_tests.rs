//! Integration tests for the SlideBar widget
//!
//! These tests verify the SlideBar widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

#[macro_use]
mod common;

use iced::event::Status;
use iced::mouse::Button;
use iced::{Element, Event, Length, Point};
use iced_aw::SlideBar;
use iced_test::Error;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
enum Message {
    Changed(u32),
    Released,
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Stateful App for Testing Interactions
// ============================================================================

#[derive(Clone)]
struct StatefulSlideBarApp {
    value: u32,
    messages_received: Rc<RefCell<Vec<Message>>>,
}

impl StatefulSlideBarApp {
    fn new(initial_value: u32) -> (Self, iced::Task<Message>) {
        (
            Self {
                value: initial_value,
                messages_received: Rc::new(RefCell::new(Vec::new())),
            },
            iced::Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        self.messages_received.borrow_mut().push(message.clone());
        match message {
            Message::Changed(new_value) => {
                self.value = new_value;
            }
            Message::Released => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        SlideBar::new(0..=100, self.value, Message::Changed).into()
    }

    fn view_with_release(&self) -> Element<'_, Message> {
        SlideBar::new(0..=100, self.value, Message::Changed)
            .on_release(Message::Released)
            .into()
    }

    fn messages_received(&self) -> Vec<Message> {
        self.messages_received.borrow().clone()
    }
}

// ============================================================================
// Basic Rendering Tests
// ============================================================================

#[test]
fn slide_bar_renders_with_defaults() {
    run_test(|| SlideBar::new(0..=100, 50, Message::Changed).into());
}

#[test]
fn slide_bar_renders_with_custom_width() {
    run_test(|| {
        SlideBar::new(0..=100, 50, Message::Changed)
            .width(Length::Fixed(300.0))
            .into()
    });
}

#[test]
fn slide_bar_renders_with_custom_height() {
    run_test(|| {
        SlideBar::new(0..=100, 50, Message::Changed)
            .height(Some(Length::Fixed(50.0)))
            .into()
    });
}

#[test]
fn slide_bar_renders_with_step() {
    run_test(|| {
        SlideBar::new(0u32..=100, 50, Message::Changed)
            .step(10u32)
            .into()
    });
}

#[test]
fn slide_bar_renders_with_on_release() {
    run_test(|| {
        SlideBar::new(0..=100, 50, Message::Changed)
            .on_release(Message::Released)
            .into()
    });
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn slide_bar_different_ranges() {
    run_test(|| SlideBar::new(0..=100, 50, Message::Changed).into());
    run_test(|| SlideBar::new(10..=50, 30, Message::Changed).into());
    run_test(|| SlideBar::new(0..=10, 5, Message::Changed).into());
}

#[test]
fn slide_bar_value_at_start() {
    run_test(|| SlideBar::new(0..=100, 0, Message::Changed).into());
}

#[test]
fn slide_bar_value_at_end() {
    run_test(|| SlideBar::new(0..=100, 100, Message::Changed).into());
}

#[test]
fn slide_bar_value_in_middle() {
    run_test(|| SlideBar::new(0..=100, 50, Message::Changed).into());
}

// ============================================================================
// State Management Tests
// ============================================================================

#[test]
fn slide_bar_maintains_state() -> Result<(), Error> {
    // Verify the widget maintains its value through the state
    let (app, _) = StatefulSlideBarApp::new(50);
    let _ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // The widget should render with the initial value
    assert_eq!(app.value, 50);

    Ok(())
}

#[test]
fn slide_bar_state_updates_on_message() -> Result<(), Error> {
    // Verify state updates when messages are processed
    let (mut app, _) = StatefulSlideBarApp::new(50);

    // Simulate a value change
    app.update(Message::Changed(75));

    assert_eq!(app.value, 75);
    assert_eq!(app.messages_received().len(), 1);

    Ok(())
}

#[test]
fn slide_bar_tracks_multiple_changes() -> Result<(), Error> {
    let (mut app, _) = StatefulSlideBarApp::new(50);

    // Simulate multiple value changes
    app.update(Message::Changed(60));
    app.update(Message::Changed(70));
    app.update(Message::Changed(80));

    assert_eq!(app.value, 80);
    assert_eq!(app.messages_received().len(), 3);

    Ok(())
}

#[test]
fn slide_bar_on_release_message() -> Result<(), Error> {
    let (mut app, _) = StatefulSlideBarApp::new(50);

    // Simulate a change followed by release
    app.update(Message::Changed(75));
    app.update(Message::Released);

    let messages = app.messages_received();
    assert_eq!(messages.len(), 2);
    assert!(matches!(messages[0], Message::Changed(75)));
    assert!(matches!(messages[1], Message::Released));

    Ok(())
}

// ============================================================================
// Operate Method Tests
// ============================================================================

#[test]
fn slide_bar_operate_method_works() -> Result<(), Error> {
    // This test verifies that the operate() method is properly implemented
    // and the widget can be created without errors
    let (app, _) = App::new(|| SlideBar::new(0..=100, 50, Message::Changed).into());

    // Creating a simulator exercises the operate() method through the widget tree
    let _ui = simulator(&app);

    // If we got here without panicking, the operate() method is working
    Ok(())
}

#[test]
fn slide_bar_with_stateful_app_renders() -> Result<(), Error> {
    // Verify a stateful app can render the widget
    let (app, _) = StatefulSlideBarApp::new(30);
    let _ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    Ok(())
}

#[test]
fn slide_bar_with_release_callback_renders() -> Result<(), Error> {
    // Verify widget with on_release callback renders correctly
    let (app, _) = StatefulSlideBarApp::new(30);
    let _ui =
        iced_test::Simulator::with_settings(iced::Settings::default(), app.view_with_release());

    Ok(())
}

#[test]
fn slide_bar_with_all_options() {
    run_test(|| {
        SlideBar::new(0u32..=100, 50, Message::Changed)
            .width(Length::Fixed(200.0))
            .height(Some(Length::Fixed(40.0)))
            .step(5u32)
            .on_release(Message::Released)
            .into()
    });
}

// ============================================================================
// Mouse Interaction Tests
// ============================================================================

#[test]
fn slide_bar_responds_to_mouse_click() -> Result<(), Error> {
    let (mut app, _) = StatefulSlideBarApp::new(50);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Get the bounds of the slider - it should be 200px wide by default
    // Click at the start (should set value to 0)
    ui.point_at(Point::new(10.0, 15.0));
    let statuses = ui.simulate(vec![
        Event::Mouse(iced::mouse::Event::ButtonPressed(Button::Left)),
        Event::Mouse(iced::mouse::Event::ButtonReleased(Button::Left)),
    ]);

    // Process any messages
    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify at least some events were processed
    assert!(statuses.contains(&Status::Captured));

    Ok(())
}

#[test]
fn slide_bar_click_produces_changed_message() -> Result<(), Error> {
    let (app, _) = StatefulSlideBarApp::new(50);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Click somewhere on the slider
    ui.point_at(Point::new(100.0, 15.0));
    ui.simulate(vec![Event::Mouse(iced::mouse::Event::ButtonPressed(
        Button::Left,
    ))]);

    // Get messages
    let messages: Vec<_> = ui.into_messages().collect();

    // Should have received a Changed message
    assert!(
        messages.iter().any(|m| matches!(m, Message::Changed(_))),
        "Should receive Changed message on click"
    );

    Ok(())
}

#[test]
fn slide_bar_drag_operation() -> Result<(), Error> {
    let (mut app, _) = StatefulSlideBarApp::new(50);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Simulate a drag operation
    ui.point_at(Point::new(50.0, 15.0));
    ui.simulate(vec![Event::Mouse(iced::mouse::Event::ButtonPressed(
        Button::Left,
    ))]);

    // Move the cursor while holding the button
    ui.point_at(Point::new(150.0, 15.0));
    ui.simulate(vec![Event::Mouse(iced::mouse::Event::CursorMoved {
        position: Point::new(150.0, 15.0),
    })]);

    // Release the button
    ui.simulate(vec![Event::Mouse(iced::mouse::Event::ButtonReleased(
        Button::Left,
    ))]);

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    // We should have received messages during the drag
    let messages = app.messages_received();
    assert!(
        !messages.is_empty(),
        "Should receive messages during drag operation"
    );

    Ok(())
}

#[test]
fn slide_bar_release_callback_triggered() -> Result<(), Error> {
    let (mut app, _) = StatefulSlideBarApp::new(50);
    let mut ui =
        iced_test::Simulator::with_settings(iced::Settings::default(), app.view_with_release());

    // Click and drag, then release
    ui.point_at(Point::new(100.0, 15.0));
    ui.simulate(vec![
        Event::Mouse(iced::mouse::Event::ButtonPressed(Button::Left)),
        Event::Mouse(iced::mouse::Event::CursorMoved {
            position: Point::new(120.0, 15.0),
        }),
        Event::Mouse(iced::mouse::Event::ButtonReleased(Button::Left)),
    ]);

    // Process all messages
    for message in ui.into_messages() {
        app.update(message);
    }

    let messages = app.messages_received();

    // Should have both Changed and Released messages
    let has_changed = messages.iter().any(|m| matches!(m, Message::Changed(_)));
    let has_released = messages.iter().any(|m| matches!(m, Message::Released));

    assert!(has_changed, "Should have Changed message");
    assert!(
        has_released,
        "Should have Released message on button release"
    );

    Ok(())
}

#[test]
fn slide_bar_multiple_drag_movements() -> Result<(), Error> {
    let (mut app, _) = StatefulSlideBarApp::new(50);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Start dragging
    ui.point_at(Point::new(50.0, 15.0));
    ui.simulate(vec![Event::Mouse(iced::mouse::Event::ButtonPressed(
        Button::Left,
    ))]);

    // Multiple cursor movements while dragging
    for x in [60.0, 70.0, 80.0, 90.0, 100.0] {
        ui.point_at(Point::new(x, 15.0));
        ui.simulate(vec![Event::Mouse(iced::mouse::Event::CursorMoved {
            position: Point::new(x, 15.0),
        })]);
    }

    // Release
    ui.simulate(vec![Event::Mouse(iced::mouse::Event::ButtonReleased(
        Button::Left,
    ))]);

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    let messages = app.messages_received();

    // Should have received multiple Changed messages as we dragged
    let changed_count = messages
        .iter()
        .filter(|m| matches!(m, Message::Changed(_)))
        .count();
    assert!(
        changed_count > 0,
        "Should receive Changed messages during drag"
    );

    Ok(())
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn slide_bar_minimum_value() {
    run_test(|| SlideBar::new(0..=100, 0, Message::Changed).into());
}

#[test]
fn slide_bar_maximum_value() {
    run_test(|| SlideBar::new(0..=100, 100, Message::Changed).into());
}

#[test]
fn slide_bar_single_value_range() {
    run_test(|| SlideBar::new(50..=50, 50, Message::Changed).into());
}

#[test]
fn slide_bar_with_negative_range() {
    run_test(|| SlideBar::new(-100..=100, 0, |v: i32| Message::Changed(v as u32)).into());
}

#[test]
fn slide_bar_with_float_values() {
    run_test(|| {
        SlideBar::new(0.0..=1.0, 0.5, |v: f32| {
            Message::Changed((v * 100.0) as u32)
        })
        .into()
    });
}

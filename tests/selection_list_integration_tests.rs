//! Integration tests for the SelectionList widget
//!
//! These tests verify the SelectionList widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

// Test Notes:
// Simulator API https://raw.githubusercontent.com/iced-rs/iced/master/test/src/simulator.rs

use iced::{Element, Settings};
use iced_aw::SelectionList;
use iced_test::{Error, Simulator};

#[derive(Clone, Debug)]
enum Message {
    Selected(usize, String),
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

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(_, _) => {
                // No state changes in these tests
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        (self.view_fn)()
    }
}

fn simulator(app: &App) -> Simulator<'_, Message> {
    Simulator::with_settings(
        Settings {
            ..Settings::default()
        },
        app.view(),
    )
}

// ============================================================================
// Basic Text Finding Tests
// ============================================================================

#[test]
fn selection_list_can_find_option_text() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
    ]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Option 1").is_ok(),
        "Option 1 text should be findable"
    );
    assert!(
        ui.find("Option 2").is_ok(),
        "Option 2 text should be findable"
    );
    assert!(
        ui.find("Option 3").is_ok(),
        "Option 3 text should be findable"
    );

    Ok(())
}

#[test]
fn selection_list_can_find_all_options_in_list() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
    ]));

    let (app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    // Verify all options are findable
    for option in &["Apple", "Banana", "Cherry", "Date", "Elderberry"] {
        assert!(ui.find(*option).is_ok(), "{} should be findable", option);
    }

    Ok(())
}

// ============================================================================
// Click and Selection Tests
// ============================================================================

#[test]
fn selection_list_clicking_option_produces_selected_message() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "First".to_string(),
        "Second".to_string(),
        "Third".to_string(),
    ]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    // Verify option is findable
    assert!(ui.find("Second").is_ok(), "Second should be findable");

    // Click the option
    ui.click("Second")?;

    // Verify we got a Selected message
    let mut got_selected = false;
    let mut selected_index = None;
    let mut selected_value = None;

    for message in ui.into_messages() {
        let Message::Selected(index, value) = &message;
        got_selected = true;
        selected_index = Some(*index);
        selected_value = Some(value.clone());
        app.update(message);
    }

    assert!(
        got_selected,
        "Clicking option should produce Selected message"
    );
    assert_eq!(selected_index, Some(1), "Selected index should be 1");
    assert_eq!(
        selected_value,
        Some("Second".to_string()),
        "Selected value should be 'Second'"
    );

    Ok(())
}

#[test]
fn selection_list_clicking_first_option_produces_correct_message() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Alpha".to_string(),
        "Beta".to_string(),
        "Gamma".to_string(),
    ]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);
    ui.click("Alpha")?;

    let mut selected_index = None;
    for message in ui.into_messages() {
        let Message::Selected(index, value) = &message;
        selected_index = Some(*index);
        assert_eq!(value, "Alpha");
        app.update(message);
    }

    assert_eq!(selected_index, Some(0), "First option should have index 0");

    Ok(())
}

#[test]
fn selection_list_clicking_last_option_produces_correct_message() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Red".to_string(),
        "Green".to_string(),
        "Blue".to_string(),
    ]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);
    ui.click("Blue")?;

    let mut selected_index = None;
    for message in ui.into_messages() {
        let Message::Selected(index, value) = &message;
        selected_index = Some(*index);
        assert_eq!(value, "Blue");
        app.update(message);
    }

    assert_eq!(selected_index, Some(2), "Last option should have index 2");

    Ok(())
}

// ============================================================================
// Multiple Selection Workflow Tests
// ============================================================================

#[test]
fn selection_list_clicking_different_options_produces_different_messages() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Item A".to_string(),
        "Item B".to_string(),
        "Item C".to_string(),
    ]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    // First click
    let mut ui = simulator(&app);
    ui.click("Item A")?;

    let mut first_selection = None;
    for message in ui.into_messages() {
        let Message::Selected(index, value) = &message;
        first_selection = Some((*index, value.clone()));
        app.update(message);
    }

    assert_eq!(
        first_selection.as_ref().map(|(i, v)| (*i, v.as_str())),
        Some((0, "Item A")),
        "First click should select Item A"
    );

    // Second click
    let mut ui = simulator(&app);
    ui.click("Item C")?;

    let mut second_selection = None;
    for message in ui.into_messages() {
        let Message::Selected(index, value) = &message;
        second_selection = Some((*index, value.clone()));
        app.update(message);
    }

    assert_eq!(
        second_selection.as_ref().map(|(i, v)| (*i, v.as_str())),
        Some((2, "Item C")),
        "Second click should select Item C"
    );

    Ok(())
}

// ============================================================================
// Edge Cases and Special Characters
// ============================================================================

#[test]
fn selection_list_handles_options_with_numbers() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    assert!(ui.find("Item 1").is_ok());
    assert!(ui.find("Item 2").is_ok());
    assert!(ui.find("Item 3").is_ok());

    ui.click("Item 2")?;

    let mut selected = false;
    for message in ui.into_messages() {
        if let Message::Selected(1, ref value) = message {
            selected = true;
            assert_eq!(value, "Item 2");
        }
        app.update(message);
    }

    assert!(selected, "Item 2 should be selectable");

    Ok(())
}

#[test]
fn selection_list_handles_options_with_special_characters() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "Option-A".to_string(),
        "Option_B".to_string(),
        "Option.C".to_string(),
    ]));

    let (app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    assert!(ui.find("Option-A").is_ok());
    assert!(ui.find("Option_B").is_ok());
    assert!(ui.find("Option.C").is_ok());

    Ok(())
}

#[test]
fn selection_list_handles_empty_list() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![]));

    let (app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    // Just verify it doesn't crash with empty list
    // Try to find a non-existent option
    assert!(ui.find("NonExistent").is_err());

    Ok(())
}

#[test]
fn selection_list_handles_single_option() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec!["Only Option".to_string()]));

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    assert!(ui.find("Only Option").is_ok());
    ui.click("Only Option")?;

    let mut selected = false;
    for message in ui.into_messages() {
        if let Message::Selected(0, ref value) = message {
            selected = true;
            assert_eq!(value, "Only Option");
        }
        app.update(message);
    }

    assert!(selected, "Single option should be selectable");

    Ok(())
}

// ============================================================================
// Long List Tests
// ============================================================================

#[test]
fn selection_list_handles_long_list_of_options() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(
        (1..=20)
            .map(|i| format!("Option {}", i))
            .collect::<Vec<String>>()
            .into_boxed_slice(),
    );

    let (mut app, _) = App::new(move || SelectionList::new(options, Message::Selected).into());

    let mut ui = simulator(&app);

    // Verify some options are findable
    assert!(ui.find("Option 1").is_ok());
    assert!(ui.find("Option 10").is_ok());
    assert!(ui.find("Option 20").is_ok());

    // Click an option in the middle
    ui.click("Option 15")?;

    let mut selected = false;
    for message in ui.into_messages() {
        if let Message::Selected(14, ref value) = message {
            selected = true;
            assert_eq!(value, "Option 15");
        }
        app.update(message);
    }

    assert!(selected, "Option 15 should be selectable");

    Ok(())
}

// ============================================================================
// Widget Configuration Tests
// ============================================================================

#[test]
fn selection_list_with_custom_width() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
    ]));

    let (app, _) = App::new(move || {
        SelectionList::new(options, Message::Selected)
            .width(300)
            .into()
    });

    let mut ui = simulator(&app);

    // Verify options are still findable with custom width
    assert!(ui.find("A").is_ok());
    assert!(ui.find("B").is_ok());
    assert!(ui.find("C").is_ok());

    Ok(())
}

#[test]
fn selection_list_with_custom_height() -> Result<(), Error> {
    let options: &'static [String] = Box::leak(Box::new(vec![
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string(),
    ]));

    let (app, _) = App::new(move || {
        SelectionList::new(options, Message::Selected)
            .height(200)
            .into()
    });

    let mut ui = simulator(&app);

    // Verify options are still findable with custom height
    assert!(ui.find("X").is_ok());
    assert!(ui.find("Y").is_ok());
    assert!(ui.find("Z").is_ok());

    Ok(())
}

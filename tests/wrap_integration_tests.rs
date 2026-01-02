//! Integration tests for the Wrap widget
//!
//! These tests verify the Wrap widget's behavior by actually exercising
//! the widget through the iced test framework. The Wrap widget displays
//! its children in multiple horizontal or vertical runs.

#[macro_use]
mod common;

use iced::{Alignment, Element, Length};
use iced_aw::Wrap;
use iced_test::Error;
use iced_widget::{Button, Text};

// Message type for the tests (unused but required by iced)
#[derive(Clone, Debug)]
enum Message {}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Basic Rendering Tests - Horizontal
// ============================================================================

#[test]
fn wrap_horizontal_empty_renders() {
    run_test(|| Wrap::<Message, iced_aw::wrap::direction::Horizontal>::new().into());
}

#[test]
fn wrap_horizontal_single_element_renders() {
    run_test_and_find(|| Wrap::new().push(Text::new("Single")).into(), "Single");
}

#[test]
fn wrap_horizontal_multiple_elements_render() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("First"))
                .push(Text::new("Second"))
                .push(Text::new("Third"))
                .into()
        },
        "First",
    );
}

#[test]
fn wrap_horizontal_with_buttons_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Button::new(Text::new("Button 1")))
                .push(Button::new(Text::new("Button 2")))
                .push(Button::new(Text::new("Button 3")))
                .into()
        },
        "Button 1",
    );
}

// ============================================================================
// Basic Rendering Tests - Vertical
// ============================================================================

#[test]
fn wrap_vertical_empty_renders() {
    run_test(|| Wrap::<Message, iced_aw::wrap::direction::Vertical>::new_vertical().into());
}

#[test]
fn wrap_vertical_single_element_renders() {
    run_test_and_find(
        || Wrap::new_vertical().push(Text::new("Single")).into(),
        "Single",
    );
}

#[test]
fn wrap_vertical_multiple_elements_render() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("First"))
                .push(Text::new("Second"))
                .push(Text::new("Third"))
                .into()
        },
        "First",
    );
}

#[test]
fn wrap_vertical_with_buttons_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Button::new(Text::new("Button 1")))
                .push(Button::new(Text::new("Button 2")))
                .push(Button::new(Text::new("Button 3")))
                .into()
        },
        "Button 1",
    );
}

// ============================================================================
// Spacing Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_spacing_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .push(Text::new("C"))
                .spacing(10.0)
                .into()
        },
        "A",
    );
}

#[test]
fn wrap_vertical_with_spacing_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .push(Text::new("C"))
                .spacing(10.0)
                .into()
        },
        "A",
    );
}

#[test]
fn wrap_horizontal_with_line_spacing_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .push(Text::new("C"))
                .line_spacing(20.0)
                .into()
        },
        "A",
    );
}

#[test]
fn wrap_vertical_with_line_spacing_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .push(Text::new("C"))
                .line_spacing(20.0)
                .into()
        },
        "A",
    );
}

#[test]
fn wrap_horizontal_with_both_spacings_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .push(Text::new("C"))
                .spacing(5.0)
                .line_spacing(15.0)
                .into()
        },
        "A",
    );
}

// ============================================================================
// Alignment Tests
// ============================================================================

#[test]
fn wrap_horizontal_align_start_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Start"))
                .push(Text::new("Aligned"))
                .align_items(Alignment::Start)
                .into()
        },
        "Start",
    );
}

#[test]
fn wrap_horizontal_align_center_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Center"))
                .push(Text::new("Aligned"))
                .align_items(Alignment::Center)
                .into()
        },
        "Center",
    );
}

#[test]
fn wrap_horizontal_align_end_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("End"))
                .push(Text::new("Aligned"))
                .align_items(Alignment::End)
                .into()
        },
        "End",
    );
}

#[test]
fn wrap_vertical_align_start_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Start"))
                .push(Text::new("Aligned"))
                .align_items(Alignment::Start)
                .into()
        },
        "Start",
    );
}

#[test]
fn wrap_vertical_align_center_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Center"))
                .push(Text::new("Aligned"))
                .align_items(Alignment::Center)
                .into()
        },
        "Center",
    );
}

#[test]
fn wrap_vertical_align_end_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("End"))
                .push(Text::new("Aligned"))
                .align_items(Alignment::End)
                .into()
        },
        "End",
    );
}

// ============================================================================
// Padding Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_padding_renders() {
    run_test_and_find(
        || Wrap::new().push(Text::new("Padded")).padding(10).into(),
        "Padded",
    );
}

#[test]
fn wrap_vertical_with_padding_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Padded"))
                .padding(10)
                .into()
        },
        "Padded",
    );
}

#[test]
fn wrap_horizontal_with_large_padding_renders() {
    run_test_and_find(
        || Wrap::new().push(Text::new("Large")).padding(50).into(),
        "Large",
    );
}

#[test]
fn wrap_horizontal_with_zero_padding_renders() {
    run_test_and_find(
        || Wrap::new().push(Text::new("Zero")).padding(0).into(),
        "Zero",
    );
}

// ============================================================================
// Width and Height Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_width_fill_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Fill"))
                .width_items(Length::Fill)
                .into()
        },
        "Fill",
    );
}

#[test]
fn wrap_horizontal_with_width_shrink_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Shrink"))
                .width_items(Length::Shrink)
                .into()
        },
        "Shrink",
    );
}

#[test]
fn wrap_horizontal_with_height_fill_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Fill"))
                .height_items(Length::Fill)
                .into()
        },
        "Fill",
    );
}

#[test]
fn wrap_vertical_with_width_fill_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Fill"))
                .width_items(Length::Fill)
                .into()
        },
        "Fill",
    );
}

#[test]
fn wrap_vertical_with_height_fill_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Fill"))
                .height_items(Length::Fill)
                .into()
        },
        "Fill",
    );
}

// ============================================================================
// Max Width and Max Height Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_max_width_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Item 1"))
                .push(Text::new("Item 2"))
                .push(Text::new("Item 3"))
                .max_width(200.0)
                .into()
        },
        "Item 1",
    );
}

#[test]
fn wrap_horizontal_with_max_height_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Item 1"))
                .push(Text::new("Item 2"))
                .max_height(100.0)
                .into()
        },
        "Item 1",
    );
}

#[test]
fn wrap_vertical_with_max_width_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Item 1"))
                .push(Text::new("Item 2"))
                .max_width(200.0)
                .into()
        },
        "Item 1",
    );
}

#[test]
fn wrap_vertical_with_max_height_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Item 1"))
                .push(Text::new("Item 2"))
                .push(Text::new("Item 3"))
                .max_height(100.0)
                .into()
        },
        "Item 1",
    );
}

// ============================================================================
// Line Minimal Length Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_line_minimal_length_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .line_minimal_length(20.0)
                .into()
        },
        "A",
    );
}

#[test]
fn wrap_vertical_with_line_minimal_length_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("A"))
                .push(Text::new("B"))
                .line_minimal_length(20.0)
                .into()
        },
        "A",
    );
}

// ============================================================================
// Method Chaining Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_all_methods_chained_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("First"))
                .push(Text::new("Second"))
                .push(Text::new("Third"))
                .spacing(5.0)
                .line_spacing(10.0)
                .line_minimal_length(15.0)
                .padding(8)
                .align_items(Alignment::Center)
                .width_items(Length::Fill)
                .height_items(Length::Shrink)
                .max_width(300.0)
                .max_height(200.0)
                .into()
        },
        "First",
    );
}

#[test]
fn wrap_vertical_with_all_methods_chained_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("First"))
                .push(Text::new("Second"))
                .push(Text::new("Third"))
                .spacing(5.0)
                .line_spacing(10.0)
                .line_minimal_length(15.0)
                .padding(8)
                .align_items(Alignment::Center)
                .width_items(Length::Fill)
                .height_items(Length::Shrink)
                .max_width(300.0)
                .max_height(200.0)
                .into()
        },
        "First",
    );
}

// ============================================================================
// Many Elements Test
// ============================================================================

#[test]
fn wrap_horizontal_with_many_elements_renders() {
    run_test_and_find(
        || {
            let mut wrap = Wrap::new().spacing(5.0);
            for i in 0..20 {
                wrap = wrap.push(Text::new(format!("Item {}", i)));
            }
            wrap.into()
        },
        "Item 0",
    );
}

#[test]
fn wrap_vertical_with_many_elements_renders() {
    run_test_and_find(
        || {
            let mut wrap = Wrap::new_vertical().spacing(5.0);
            for i in 0..20 {
                wrap = wrap.push(Text::new(format!("Item {}", i)));
            }
            wrap.into()
        },
        "Item 0",
    );
}

// ============================================================================
// Different Element Types Test
// ============================================================================

#[test]
fn wrap_horizontal_with_mixed_widgets_renders() {
    run_test_and_find(
        || {
            Wrap::new()
                .push(Text::new("Text"))
                .push(Button::new(Text::new("Button")))
                .push(Text::new("More"))
                .into()
        },
        "Text",
    );
}

#[test]
fn wrap_vertical_with_mixed_widgets_renders() {
    run_test_and_find(
        || {
            Wrap::new_vertical()
                .push(Text::new("Text"))
                .push(Button::new(Text::new("Button")))
                .push(Text::new("More"))
                .into()
        },
        "Text",
    );
}

// ============================================================================
// With Elements Constructor Tests
// ============================================================================

#[test]
fn wrap_horizontal_with_elements_constructor_renders() {
    run_test_and_find(
        || {
            let elements: Vec<Element<Message>> = vec![
                Text::new("First").into(),
                Text::new("Second").into(),
                Text::new("Third").into(),
            ];
            Wrap::with_elements(elements).into()
        },
        "First",
    );
}

#[test]
fn wrap_vertical_with_elements_constructor_renders() {
    run_test_and_find(
        || {
            let elements: Vec<Element<Message>> = vec![
                Text::new("First").into(),
                Text::new("Second").into(),
                Text::new("Third").into(),
            ];
            Wrap::with_elements_vertical(elements).into()
        },
        "First",
    );
}

// ============================================================================
// Snapshot Tests
// ============================================================================

#[test]
fn wrap_horizontal_snapshot_test() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        Wrap::new()
            .push(Button::new(Text::new("Button 1")))
            .push(Button::new(Text::new("Button 2")))
            .push(Button::new(Text::new("Button 3")))
            .push(Button::new(Text::new("Button 4")))
            .push(Button::new(Text::new("Button 5")))
            .spacing(5.0)
            .line_spacing(10.0)
            .padding(10)
            .align_items(Alignment::Center)
            .max_width(300.0)
            .into()
    });

    let mut ui = simulator(&app);
    assert_snapshot_matches(&mut ui, "tests/snapshots/wrap_horizontal_snapshot_test")?;

    Ok(())
}

#[test]
fn wrap_vertical_snapshot_test() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        Wrap::new_vertical()
            .push(Button::new(Text::new("Button 1")))
            .push(Button::new(Text::new("Button 2")))
            .push(Button::new(Text::new("Button 3")))
            .push(Button::new(Text::new("Button 4")))
            .push(Button::new(Text::new("Button 5")))
            .spacing(5.0)
            .line_spacing(10.0)
            .padding(10)
            .align_items(Alignment::Center)
            .max_height(200.0)
            .into()
    });

    let mut ui = simulator(&app);
    assert_snapshot_matches(&mut ui, "tests/snapshots/wrap_vertical_snapshot_test")?;

    Ok(())
}

#[test]
fn wrap_horizontal_align_start_snapshot() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        Wrap::new()
            .push(Text::new("Short"))
            .push(Button::new(Text::new("Medium Button")))
            .push(Text::new("A"))
            .spacing(5.0)
            .align_items(Alignment::Start)
            .into()
    });

    let mut ui = simulator(&app);
    assert_snapshot_matches(&mut ui, "tests/snapshots/wrap_horizontal_align_start")?;

    Ok(())
}

#[test]
fn wrap_horizontal_align_center_snapshot() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        Wrap::new()
            .push(Text::new("Short"))
            .push(Button::new(Text::new("Medium Button")))
            .push(Text::new("A"))
            .spacing(5.0)
            .align_items(Alignment::Center)
            .into()
    });

    let mut ui = simulator(&app);
    assert_snapshot_matches(&mut ui, "tests/snapshots/wrap_horizontal_align_center")?;

    Ok(())
}

#[test]
fn wrap_horizontal_align_end_snapshot() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        Wrap::new()
            .push(Text::new("Short"))
            .push(Button::new(Text::new("Medium Button")))
            .push(Text::new("A"))
            .spacing(5.0)
            .align_items(Alignment::End)
            .into()
    });

    let mut ui = simulator(&app);
    assert_snapshot_matches(&mut ui, "tests/snapshots/wrap_horizontal_align_end")?;

    Ok(())
}

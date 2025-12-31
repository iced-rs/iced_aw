# Integration Tests Guide

This directory contains integration tests for `iced_aw` widgets using the `iced_test` simulator framework.

## Quick Start

It is suggested that integration test writers try to use the common test helpers to avoid code duplication:

```rust
//! Integration tests for the MyWidget widget

#[macro_use]
mod common;

use iced::Element;
use iced_aw::MyWidget;

#[derive(Clone, Debug)]
enum Message {
    MyAction,
}

// Generate test helpers for your Message type
test_helpers!(Message);

#[test]
fn my_widget_renders() {
    run_test(|| MyWidget::new().into());
}

#[test]
fn my_widget_shows_text() {
    run_test_and_find(|| MyWidget::new().into(), "Expected Text");
}
```

## Available Helpers

The `test_helpers!` macro generates the following for your Message type:

### Types
- **`App`** - Test application wrapper with `new()`, `update()`, and `view()` methods
- **`ViewFn`** - Type alias for view functions: `Box<dyn Fn() -> Element<'static, Message>>`

### Basic Test Functions
- **`simulator(app: &App) -> Simulator<'_, Message>`**
  Creates a simulator with default settings

- **`run_test<F>(view_fn: F)`**
  Verifies a widget renders without panicking
  ```rust
  run_test(|| MyWidget::new().into());
  ```

- **`run_test_and_find<F>(view_fn: F, text: &str)`**
  Verifies a widget renders and contains specific text
  ```rust
  run_test_and_find(|| MyWidget::new().into(), "Button Label");
  ```

### Mouse and Touch Input Helpers
- **`simulate_mouse_click(ui, position, button)`**
  Simulate a mouse click at a specific Point

- **`simulate_mouse_click_at(ui, x, y, button)`**
  Simulate a mouse click at coordinates

- **`simulate_left_click_at(ui, x, y)`**
  Simulate a left mouse click at coordinates
  ```rust
  simulate_left_click_at(&mut ui, 100.0, 100.0);
  ```

- **`simulate_right_click_at(ui, x, y)`**
  Simulate a right mouse click at coordinates
  ```rust
  simulate_right_click_at(&mut ui, 10.0, 10.0);
  ```

- **`simulate_touch(ui, position)`**
  Simulate a touch event at a specific Point

- **`simulate_touch_at(ui, x, y)`**
  Simulate a touch event at coordinates
  ```rust
  simulate_touch_at(&mut ui, 50.0, 50.0);
  ```

- **`outside_position() -> Point`**
  Returns a position far outside typical widget bounds (1000.0, 1000.0)
  ```rust
  simulate_left_click_at(&mut ui, outside_position().x, outside_position().y);
  ```

### Message Verification Helpers
- **`assert_message_received<F>(ui, app, predicate, error_msg)`**
  Process messages and assert a specific message was received
  ```rust
  assert_message_received(ui, &mut app,
      |msg| matches!(msg, Message::Clicked),
      "Should receive Clicked message");
  ```

- **`check_message_received<F>(ui, app, predicate) -> bool`**
  Process messages and return whether a specific message was received
  ```rust
  let got_message = check_message_received(ui, &mut app,
      |msg| matches!(msg, Message::Expected));
  ```

- **`process_messages(ui, app)`**
  Process all messages from simulator without checking
  ```rust
  process_messages(ui, &mut app);
  ```

- **`collect_messages<F>(ui, app, predicate) -> Vec<Message>`**
  Process messages and return collected messages matching predicate
  ```rust
  let clicks = collect_messages(ui, &mut app,
      |msg| matches!(msg, Message::Clicked));
  assert_eq!(clicks.len(), 3);
  ```

## Advanced Usage

### Stateful Tests

For tests requiring state management, it is recommended to use `App` directly:

```rust
#[test]
fn button_click_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| MyWidget::new().into());
    let mut ui = simulator(&app);

    ui.click("Button")?;

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}
```

### Custom Stateful Apps

For complex state tracking, it is recommended to define a custom app alongside the generated helpers:

```rust
#[derive(Clone)]
struct StatefulTestApp {
    counter: std::rc::Rc<std::cell::RefCell<usize>>,
}

impl StatefulTestApp {
    fn new() -> (Self, iced::Task<Message>) {
        (Self { counter: std::rc::Rc::new(std::cell::RefCell::new(0)) }, iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => *self.counter.borrow_mut() += 1,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        MyWidget::new(*self.counter.borrow()).into()
    }
}
```

## Test Organization

For consistency, it is suggested that you structure your tests with clear section comments:

```rust
// ============================================================================
// Basic Rendering Tests
// ============================================================================

#[test]
fn widget_renders_with_defaults() { /* ... */ }

// ============================================================================
// Interaction Tests
// ============================================================================

#[test]
fn button_click_works() { /* ... */ }
```

## Simulator API

Full API documentation: https://github.com/iced-rs/iced/blob/master/test/src/simulator.rs

Key methods:
- `ui.find(text)` - Locate text in the widget tree
- `ui.click(text)` - Click on element containing text
- `ui.tap_key(key)` - Simulate keyboard input
- `ui.into_messages()` - Consume simulator and get produced messages

## Suggested Best Practices

1. **Use `run_test_and_find` when possible** - Tests both rendering and `operate()` implementation
2. **Keep test names descriptive** - Name should explain what's being tested
3. **Test one thing per test** - Avoid complex multi-step tests
4. **Use `Result<(), Error>` for tests with `?` operator** - Makes error handling cleaner
5. **Keep imports minimal** - Only import what you actually use

## Test Naming Convention

It is suggested that integration test function names begin with the widget name as a prefix.

### Naming Pattern

```rust
fn <widget_name>_<test_description>()
```

### Example test names

**Correct:**
```rust
#[test]
fn badge_renders_with_text() { /* ... */ }

#[test]
fn number_input_increment_button_click_produces_message() { /* ... */ }

#[test]
fn time_picker_displays_12h_format_correctly() { /* ... */ }
```

**Incorrect:**
```rust
#[test]
fn renders_with_text() { /* ... */ }  // Missing widget prefix

#[test]
fn can_increment_number() { /* ... */ }  // Missing widget prefix

#[test]
fn displays_time() { /* ... */ }  // Missing widget prefix
```

This convention helps ensure:
- Tests are clearly associated with their widget
- Test output is easier to read and filter
- grep/search operations are more effective

## Example Test File Structure

```rust
//! Integration tests for the Widget
//!
//! Brief description of what these tests cover.

#[macro_use]
mod common;

use iced::{Element, Length};
use iced_aw::Widget;
use iced_test::{Error, Simulator};

#[derive(Clone, Debug)]
enum Message {
    Action,
}

test_helpers!(Message);

// ============================================================================
// Basic Tests
// ============================================================================

#[test]
fn widget_renders() {
    run_test(|| Widget::new().into());
}

#[test]
fn widget_with_text() {
    run_test_and_find(|| Widget::new().with_label("Test").into(), "Test");
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn widget_with_custom_width() {
    run_test(|| Widget::new().width(Length::Fill).into());
}

// ============================================================================
// Interaction Tests
// ============================================================================

#[test]
fn widget_click_works() -> Result<(), Error> {
    let (mut app, _) = App::new(|| Widget::new().into());
    let mut ui = simulator(&app);

    ui.click("Target")?;

    let mut got_message = false;
    for message in ui.into_messages() {
        got_message = true;
        app.update(message);
    }

    assert!(got_message);
    Ok(())
}
```
## Button Icons Reference

Common icon characters used in widgets (searchable with `ui.find()`):

- `\u{e800}` - cancel (close/cancel buttons)
- `\u{e801}` - down_open (down arrow, dropdowns)
- `\u{e802}` - left_open (left navigation)
- `\u{e803}` - right_open (right navigation)
- `\u{e804}` - up_open (up arrow, increment)
- `\u{e805}` - ok (checkmark/submit)

Or search for their display forms: `" ▲ "`, `" ▼ "`, `" ✓ "`, etc.
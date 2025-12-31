//! Common test utilities for integration tests
//!
//! This module provides shared functionality used across multiple integration tests,
//! reducing code duplication and ensuring consistency in test setup.

/// Macro to define common test helpers for a specific Message type
///
/// This macro generates type-specific versions of TestApp and helper functions,
/// which avoids type inference issues with generic functions.
///
/// # Usage
/// ```ignore
/// test_helpers!(Message);
/// ```
///
/// This generates:
/// - `type ViewFn` - Type alias for view functions
/// - `struct App` - Test application wrapper
/// - `fn simulator(app: &App)` - Creates a simulator
/// - `fn run_test(view_fn)` - Runs a test without checking output
/// - `fn run_test_and_find(view_fn, text)` - Runs a test and finds text
#[macro_export]
macro_rules! test_helpers {
    ($message_type:ty) => {
        type ViewFn = Box<dyn Fn() -> iced::Element<'static, $message_type>>;

        #[derive(Clone)]
        struct App {
            view_fn: std::rc::Rc<ViewFn>,
        }

        impl App {
            fn new<F>(view_fn: F) -> (Self, iced::Task<$message_type>)
            where
                F: Fn() -> iced::Element<'static, $message_type> + 'static,
            {
                (
                    App {
                        view_fn: std::rc::Rc::new(Box::new(view_fn)),
                    },
                    iced::Task::none(),
                )
            }

            #[allow(unused_variables, dead_code)]
            fn update(&mut self, message: $message_type) {
                // Default: no state changes
                // Override this in tests if needed
            }

            fn view(&self) -> iced::Element<'static, $message_type> {
                (self.view_fn)()
            }
        }

        #[allow(dead_code)]
        fn simulator(app: &App) -> iced_test::Simulator<'static, $message_type> {
            iced_test::Simulator::with_settings(iced::Settings::default(), app.view())
        }

        /// Helper to run a test with a given view
        #[allow(dead_code)]
        fn run_test<F>(view_fn: F)
        where
            F: Fn() -> iced::Element<'static, $message_type> + 'static,
        {
            let (app, _) = App::new(view_fn);
            let _ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
            // The widget is successfully rendered if we get here without panicking
        }

        /// Helper to verify text can be found (tests operate() implementation)
        #[allow(dead_code)]
        fn run_test_and_find<F>(view_fn: F, text: &str)
        where
            F: Fn() -> iced::Element<'static, $message_type> + 'static,
        {
            let (app, _) = App::new(view_fn);
            let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
            assert!(
                ui.find(text).is_ok(),
                "Failed to find text '{}' in widget",
                text
            );
        }

        // ====================================================================
        // Mouse and Touch Input Helpers
        // ====================================================================

        /// Simulate a mouse click at a specific position
        #[allow(dead_code)]
        fn simulate_mouse_click(
            ui: &mut iced_test::Simulator<'_, $message_type>,
            position: iced_core::Point,
            button: iced_core::mouse::Button,
        ) {
            use iced::Event;
            use iced_core::mouse;

            ui.simulate([
                Event::Mouse(mouse::Event::CursorMoved { position }),
                Event::Mouse(mouse::Event::ButtonPressed(button)),
            ]);
        }

        /// Simulate a mouse click at coordinates
        #[allow(dead_code)]
        fn simulate_mouse_click_at(
            ui: &mut iced_test::Simulator<'_, $message_type>,
            x: f32,
            y: f32,
            button: iced_core::mouse::Button,
        ) {
            simulate_mouse_click(ui, iced_core::Point::new(x, y), button);
        }

        /// Simulate a left mouse click at coordinates
        #[allow(dead_code)]
        fn simulate_left_click_at(
            ui: &mut iced_test::Simulator<'_, $message_type>,
            x: f32,
            y: f32,
        ) {
            simulate_mouse_click_at(ui, x, y, iced_core::mouse::Button::Left);
        }

        /// Simulate a right mouse click at coordinates
        #[allow(dead_code)]
        fn simulate_right_click_at(
            ui: &mut iced_test::Simulator<'_, $message_type>,
            x: f32,
            y: f32,
        ) {
            simulate_mouse_click_at(ui, x, y, iced_core::mouse::Button::Right);
        }

        /// Simulate a touch event at a specific position
        #[allow(dead_code)]
        fn simulate_touch(
            ui: &mut iced_test::Simulator<'_, $message_type>,
            position: iced_core::Point,
        ) {
            use iced::Event;
            use iced_core::touch;

            ui.simulate([Event::Touch(touch::Event::FingerPressed {
                id: touch::Finger(0),
                position,
            })]);
        }

        /// Simulate a touch event at coordinates
        #[allow(dead_code)]
        fn simulate_touch_at(ui: &mut iced_test::Simulator<'_, $message_type>, x: f32, y: f32) {
            simulate_touch(ui, iced_core::Point::new(x, y));
        }

        /// Common position far outside typical widget bounds (for "click outside" tests)
        #[allow(dead_code)]
        fn outside_position() -> iced_core::Point {
            iced_core::Point::new(1000.0, 1000.0)
        }

        // ====================================================================
        // Message Verification Helpers
        // ====================================================================

        /// Process messages from simulator and check if a specific message was received.
        /// Panics with the provided error message if the message was not received.
        #[allow(dead_code)]
        fn assert_message_received<F>(
            ui: iced_test::Simulator<'_, $message_type>,
            app: &mut App,
            mut predicate: F,
            error_msg: &str,
        ) where
            F: FnMut(&$message_type) -> bool,
        {
            let mut received = false;
            for message in ui.into_messages() {
                if predicate(&message) {
                    received = true;
                }
                app.update(message);
            }
            assert!(received, "{}", error_msg);
        }

        /// Process messages from simulator and return whether a specific message was received
        #[allow(dead_code)]
        fn check_message_received<F>(
            ui: iced_test::Simulator<'_, $message_type>,
            app: &mut App,
            mut predicate: F,
        ) -> bool
        where
            F: FnMut(&$message_type) -> bool,
        {
            let mut received = false;
            for message in ui.into_messages() {
                if predicate(&message) {
                    received = true;
                }
                app.update(message);
            }
            received
        }

        /// Process all messages from simulator without checking
        #[allow(dead_code)]
        fn process_messages(ui: iced_test::Simulator<'_, $message_type>, app: &mut App) {
            for message in ui.into_messages() {
                app.update(message);
            }
        }

        /// Process messages and return collected messages matching predicate
        #[allow(dead_code, unreachable_code)]
        fn collect_messages<F>(
            ui: iced_test::Simulator<'_, $message_type>,
            app: &mut App,
            mut predicate: F,
        ) -> Vec<$message_type>
        where
            F: FnMut(&$message_type) -> bool,
            $message_type: Clone,
        {
            let mut collected = Vec::new();
            for message in ui.into_messages() {
                if predicate(&message) {
                    collected.push(message.clone());
                }
                app.update(message);
            }
            collected
        }
    };
}

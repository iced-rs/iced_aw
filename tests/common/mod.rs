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

            fn view(&self) -> iced::Element<'_, $message_type> {
                (self.view_fn)()
            }
        }

        #[allow(dead_code)]
        fn simulator(app: &App) -> iced_test::Simulator<'_, $message_type> {
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
    };
}

//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*

use super::overlay::time_picker::{self, TimePickerOverlay, TimePickerOverlayButtons};

use chrono::Local;
use iced_core::{
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{
        self,
        tree::{self, Tag, Tree},
    },
};
use iced_widget::{Renderer, button, container, text};

pub use crate::{
    core::time::{Period, Time},
    style::{
        Status, StyleFn,
        time_picker::{Catalog, Style},
    },
};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking times.
///
/// # Example
/// ```ignore
/// # use iced_aw::{TimePicker, time_picker};
/// # use iced_widget::{button, Button, Text};
/// #
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(time_picker::Time),
/// }
///
/// let time_picker = TimePicker::new(
///     true,
///     time_picker::Time::now_hms(true),
///     Button::new(Text::new("Pick time"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct TimePicker<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + button::Catalog,
{
    /// Show the picker.
    show_picker: bool,
    /// The time to show.
    time: Time,
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The message that is send if the cancel button of the [`TimePickerOverlay`] is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`] is pressed.
    on_submit: Box<dyn Fn(Time) -> Message>,
    /// The style of the [`TimePickerOverlay`].
    class: <Theme as Catalog>::Class<'a>,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Theme, Renderer>,
    /// Toggle the use of the 24h clock of the [`TimePickerOverlay`].
    use_24h: bool,
    /// Toggle the use of the seconds of the [`TimePickerOverlay`].
    show_seconds: bool,
}

impl<'a, Message, Theme> TimePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog,
{
    /// Creates a new [`TimePicker`] wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the time picker is visible.
    ///     * the initial time to show.
    ///     * the underlay [`Element`] on which this [`TimePicker`]
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`TimePicker`] is pressed.
    ///     * a function that will be called when the submit button of the [`TimePicker`]
    ///         is pressed, which takes the picked [`Time`] value.
    pub fn new<U, F>(
        show_picker: bool,
        time: impl Into<Time>,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            show_picker,
            time: time.into(),
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            class: <Theme as Catalog>::default(),
            overlay_state: TimePickerOverlayButtons::default().into(),
            use_24h: false,
            show_seconds: false,
        }
    }

    /// Enables the picker to also pick seconds.
    #[must_use]
    pub fn show_seconds(mut self) -> Self {
        self.show_seconds = true;
        self
    }

    /// Sets the style of the [`TimePicker`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Use 24 hour format instead of AM/PM.
    #[must_use]
    pub fn use_24h(mut self) -> Self {
        self.use_24h = true;
        self
    }

    /// Sets the class of the input of the [`TimePicker`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// The state of the [`TimePicker`] / [`TimePickerOverlay`].
#[derive(Debug)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: time_picker::State,
}

impl State {
    /// Creates a new [`State`] with the current time.
    #[must_use]
    pub fn now() -> Self {
        Self {
            overlay_state: time_picker::State::default(),
        }
    }

    /// Creates a new [`State`] with the given time.
    #[must_use]
    pub fn new(time: Time, use_24h: bool, show_seconds: bool) -> Self {
        Self {
            overlay_state: time_picker::State::new(time, use_24h, show_seconds),
        }
    }

    /// Resets the time of the state to the current time.
    pub fn reset(&mut self) {
        self.overlay_state.clock_cache.clear();
        self.overlay_state.time = Local::now().naive_local().time();
    }
}

impl<Message, Theme> Widget<Message, Theme, Renderer> for TimePicker<'_, Message, Theme>
where
    Message: 'static + Clone,
    Theme: Catalog + button::Catalog + text::Catalog + container::Catalog,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.time, self.use_24h, self.show_seconds))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay_state)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget_mut().update(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<()>,
    ) {
        self.underlay
            .as_widget_mut()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let picker_state: &mut State = tree.state.downcast_mut();

        if !self.show_picker {
            return self.underlay.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            TimePickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.class,
                &mut tree.children[1],
                *viewport,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Theme> From<TimePicker<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'static + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog + container::Catalog,
{
    fn from(time_picker: TimePicker<'a, Message, Theme>) -> Self {
        Element::new(time_picker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, Timelike};

    #[derive(Clone, Debug)]
    enum TestMessage {
        Cancel,
        #[allow(dead_code)]
        Submit(Time),
    }

    type TestTimePicker<'a> = TimePicker<'a, TestMessage, iced_widget::Theme>;

    #[test]
    fn state_new_creates_state_with_time() {
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };
        let state = State::new(time, true, false);

        assert_eq!(state.overlay_state.time.hour(), 14);
        assert_eq!(state.overlay_state.time.minute(), 30);
        assert!(state.overlay_state.use_24h);
        assert!(!state.overlay_state.show_seconds);
    }

    #[test]
    fn state_new_with_seconds() {
        let time = Time::Hms {
            hour: 14,
            minute: 30,
            second: 45,
            period: Period::H24,
        };
        let state = State::new(time, true, true);

        assert_eq!(state.overlay_state.time.hour(), 14);
        assert_eq!(state.overlay_state.time.minute(), 30);
        assert_eq!(state.overlay_state.time.second(), 45);
        assert!(state.overlay_state.show_seconds);
    }

    #[test]
    fn state_new_with_12h_format() {
        let time = Time::Hm {
            hour: 2,
            minute: 30,
            period: Period::Pm,
        };
        let state = State::new(time, false, false);

        assert_eq!(state.overlay_state.time.hour(), 14); // 2 PM = 14:00 in 24h
        assert_eq!(state.overlay_state.time.minute(), 30);
        assert!(!state.overlay_state.use_24h);
    }

    #[test]
    fn state_now_creates_state() {
        let state = State::now();
        // Just verify it was created successfully and has valid time
        assert!(state.overlay_state.time.hour() < 24);
        assert!(state.overlay_state.time.minute() < 60);
        assert!(state.overlay_state.time.second() < 60);
    }

    #[test]
    fn state_reset_updates_to_current_time() {
        let old_time = Time::Hm {
            hour: 1,
            minute: 1,
            period: Period::H24,
        };
        let mut state = State::new(old_time, true, false);

        state.reset();

        // After reset, the time should be current
        // We can't easily verify the exact time since it's based on Local::now()
        // but we can verify the state is still valid
        assert!(state.overlay_state.time.hour() < 24);
        assert!(state.overlay_state.time.minute() < 60);
    }

    #[test]
    fn time_picker_new_creates_instance() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            false,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(!time_picker.show_picker);
        match time_picker.time {
            Time::Hm { hour, minute, .. } => {
                assert_eq!(hour, 14);
                assert_eq!(minute, 30);
            }
            Time::Hms { .. } => panic!("Expected Time::Hm variant"),
        }
        assert!(!time_picker.use_24h); // Default is 12h
        assert!(!time_picker.show_seconds); // Default is no seconds
    }

    #[test]
    fn time_picker_show_picker_true() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            true,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(time_picker.show_picker);
    }

    #[test]
    fn time_picker_use_24h_sets_value() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            false,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        )
        .use_24h();

        assert!(time_picker.use_24h);
    }

    #[test]
    fn time_picker_show_seconds_sets_value() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            false,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        )
        .show_seconds();

        assert!(time_picker.show_seconds);
    }

    #[test]
    fn time_picker_chained_configuration() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            true,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        )
        .use_24h()
        .show_seconds();

        assert!(time_picker.show_picker);
        assert!(time_picker.use_24h);
        assert!(time_picker.show_seconds);
    }

    #[test]
    fn time_picker_tag_returns_state_tag() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            false,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let tag = Widget::<TestMessage, iced_widget::Theme, Renderer>::tag(&time_picker);
        assert_eq!(tag, widget::tree::Tag::of::<State>());
    }

    #[test]
    fn time_picker_has_two_children() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            false,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let children = Widget::<TestMessage, iced_widget::Theme, Renderer>::children(&time_picker);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn time_picker_size_matches_underlay() {
        let underlay = iced_widget::text::Text::new("Pick a time");
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        let time_picker = TestTimePicker::new(
            false,
            time,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let size = Widget::<TestMessage, iced_widget::Theme, Renderer>::size(&time_picker);
        assert_eq!(size.width, Length::Shrink);
        assert_eq!(size.height, Length::Shrink);
    }

    #[test]
    fn time_hm_creates_correct_time() {
        let time = Time::Hm {
            hour: 14,
            minute: 30,
            period: Period::H24,
        };

        match time {
            Time::Hm { hour, minute, .. } => {
                assert_eq!(hour, 14);
                assert_eq!(minute, 30);
            }
            Time::Hms { .. } => panic!("Expected Time::Hm variant"),
        }
    }

    #[test]
    fn time_hms_creates_correct_time() {
        let time = Time::Hms {
            hour: 14,
            minute: 30,
            second: 45,
            period: Period::H24,
        };

        match time {
            Time::Hms {
                hour,
                minute,
                second,
                ..
            } => {
                assert_eq!(hour, 14);
                assert_eq!(minute, 30);
                assert_eq!(second, 45);
            }
            Time::Hm { .. } => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn time_12h_am_conversion() {
        let time = Time::Hm {
            hour: 9,
            minute: 30,
            period: Period::Am,
        };

        let naive_time: NaiveTime = time.into();
        assert_eq!(naive_time.hour(), 9); // 9 AM = 9:00
    }

    #[test]
    fn time_12h_pm_conversion() {
        let time = Time::Hm {
            hour: 2,
            minute: 30,
            period: Period::Pm,
        };

        let naive_time: NaiveTime = time.into();
        assert_eq!(naive_time.hour(), 14); // 2 PM = 14:00
    }

    #[test]
    fn time_12h_noon_conversion() {
        let time = Time::Hm {
            hour: 12,
            minute: 0,
            period: Period::Pm,
        };

        let naive_time: NaiveTime = time.into();
        assert_eq!(naive_time.hour(), 12); // 12 PM = noon = 12:00
    }

    #[test]
    fn time_12h_midnight_conversion() {
        let time = Time::Hm {
            hour: 12,
            minute: 0,
            period: Period::Am,
        };

        let naive_time: NaiveTime = time.into();
        assert_eq!(naive_time.hour(), 0); // 12 AM = midnight = 00:00
    }

    #[test]
    fn time_now_hm_creates_current_time() {
        let time = Time::now_hm(true);
        match time {
            Time::Hm { hour, minute, .. } => {
                assert!(hour < 24);
                assert!(minute < 60);
            }
            Time::Hms { .. } => panic!("Expected Time::Hm variant"),
        }
    }

    #[test]
    fn time_now_hms_creates_current_time() {
        let time = Time::now_hms(true);
        match time {
            Time::Hms {
                hour,
                minute,
                second,
                ..
            } => {
                assert!(hour < 24);
                assert!(minute < 60);
                assert!(second < 60);
            }
            Time::Hm { .. } => panic!("Expected Time::Hms variant"),
        }
    }

    #[test]
    fn different_times_with_24h_format() {
        let test_times = [
            (0, 0, Period::H24),   // Midnight
            (6, 30, Period::H24),  // Morning
            (12, 0, Period::H24),  // Noon
            (18, 45, Period::H24), // Evening
            (23, 59, Period::H24), // End of day
        ];

        for (expected_hour, expected_minute, period) in test_times {
            let time = Time::Hm {
                hour: expected_hour,
                minute: expected_minute,
                period,
            };
            let underlay = iced_widget::text::Text::new("Pick a time");

            let time_picker = TestTimePicker::new(
                false,
                time,
                underlay,
                TestMessage::Cancel,
                TestMessage::Submit,
            );

            match time_picker.time {
                Time::Hm { hour, minute, .. } => {
                    assert_eq!(hour, expected_hour);
                    assert_eq!(minute, expected_minute);
                }
                Time::Hms { .. } => panic!("Expected Time::Hm variant"),
            }
        }
    }
}

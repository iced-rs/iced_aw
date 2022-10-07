//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use chrono::Local;
use iced_graphics::{Backend, Renderer};
use iced_native::widget::button;
use iced_native::widget::tree::{self, Tag};
use iced_native::widget::Tree;
use iced_native::{event, mouse, Clipboard, Event, Layout, Point, Rectangle};
use iced_native::{overlay, Shell};
use iced_native::{Element, Widget};

use super::overlay::time_picker::{self, TimePickerOverlay, TimePickerOverlayButtons};

pub use crate::core::time::{Period, Time};

pub use crate::style::time_picker::{Appearance, StyleSheet};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking times.
///
/// # Example
/// ```ignore
/// # use iced_aw::time_picker;
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::{button, Button, Text};
/// #
/// # pub type TimePicker<'a, Message> = iced_aw::TimePicker<'a, Message, Null>;
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
pub struct TimePicker<'a, Message, B, Theme>
where
    Message: Clone,
    B: Backend + iced_graphics::backend::Text,
    Theme: StyleSheet + button::StyleSheet,
{
    /// Show the picker.
    show_picker: bool,
    /// The time to show.
    time: Time,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer<B, Theme>>,
    /// The message that is send if the cancel button of the [`TimePickerOverlay`](TimePickerOverlay) is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`](TimePickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Time) -> Message>,
    /// The style of the [`TimePickerOverlay`](TimePickerOverlay).
    style: <Theme as StyleSheet>::Style,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Renderer<B, Theme>>,
    /// Toggle the use of the 24h clock of the [`TimePickerOverlay`](TimePickerOverlay).
    use_24h: bool,
    /// Toggle the use of the seconds of the [`TimePickerOverlay`](TimePickerOverlay).
    show_seconds: bool,
}

impl<'a, Message, B, Theme> TimePicker<'a, Message, B, Theme>
where
    Message: 'a + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: 'a + StyleSheet + button::StyleSheet + iced_style::text::StyleSheet,
{
    /// Creates a new [`TimePicker`](TimePicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the time picker is visible.
    ///     * the initial time to show.
    ///     * the underlay [`Element`](iced_native::Element) on which this [`TimePicker`](TimePicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`TimePicker`](TimePicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`TimePicker`](TimePicker)
    ///         is pressed, which takes the picked [`Time`](crate::time_picker::Time) value.
    pub fn new<U, F>(
        show_picker: bool,
        time: impl Into<Time>,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer<B, Theme>>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            show_picker,
            time: time.into(),
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style: <Theme as StyleSheet>::Style::default(),
            overlay_state: TimePickerOverlayButtons::default().into(),
            use_24h: false,
            show_seconds: false,
        }
    }

    /// Use 24 hour format instead of AM/PM.
    #[must_use]
    pub fn use_24h(mut self) -> Self {
        self.use_24h = true;
        self
    }

    /// Enables the picker to also pick seconds.
    #[must_use]
    pub fn show_seconds(mut self) -> Self {
        self.show_seconds = true;
        self
    }

    /// Sets the style of the [`TimePicker`](TimePicker).
    #[must_use]
    pub fn style(mut self, style: <Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

/// The state of the [`TimePicker`](TimePicker) / [`TimePickerOverlay`](TimePickerOverlay).
#[derive(Debug)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: time_picker::State,
}

impl State {
    /// Creates a new [`State`](State) with the current time.
    #[must_use]
    pub fn now() -> Self {
        Self {
            overlay_state: time_picker::State::default(),
        }
    }

    /// Creates a new [`State`](State) with the given time.
    #[must_use]
    pub fn new(time: Time) -> Self {
        Self {
            overlay_state: time_picker::State::new(time),
        }
    }

    /// Resets the time of the state to the current time.
    pub fn reset(&mut self) {
        self.overlay_state.clock_cache.clear();
        self.overlay_state.time = Local::now().naive_local().time();
        self.overlay_state.use_24h = false;
        self.overlay_state.show_seconds = false;
    }
}

impl<'a, Message, B, Theme> Widget<Message, Renderer<B, Theme>>
    for TimePicker<'a, Message, B, Theme>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: StyleSheet
        + button::StyleSheet
        + iced_style::text::StyleSheet
        + iced_style::container::StyleSheet,
{
    fn tag(&self) -> iced_native::widget::tree::Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        tree::State::new(State::new(self.time))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay_state)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn width(&self) -> iced_native::Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> iced_native::Length {
        self.underlay.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer<B, Theme>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer<B, Theme>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer<B, Theme>,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer<B, Theme>,
        theme: &Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer<B, Theme>,
    ) -> Option<overlay::Element<'b, Message, Renderer<B, Theme>>> {
        let picker_state: &mut State = state.state.downcast_mut();

        if !self.show_picker {
            return self
                .underlay
                .as_widget()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            TimePickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                self.style,
                &mut state.children[1],
            )
            .overlay(),
        )
    }
}

impl<'a, Message, B, Theme> From<TimePicker<'a, Message, B, Theme>>
    for Element<'a, Message, Renderer<B, Theme>>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: 'a
        + StyleSheet
        + iced_style::button::StyleSheet
        + iced_style::text::StyleSheet
        + iced_style::container::StyleSheet,
{
    fn from(time_picker: TimePicker<'a, Message, B, Theme>) -> Self {
        Element::new(time_picker)
    }
}

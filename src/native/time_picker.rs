//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use std::hash::Hash;

use chrono::Local;
use iced_graphics::{Backend, Renderer};
use iced_native::{
    event, mouse, widget::button, Clipboard, Element, Event, Layout, Point, Rectangle, Widget,
};
use iced_native::{overlay, Shell};

use super::overlay::time_picker::{self, Focus, TimePickerOverlay};

pub use crate::core::time::{Period, Time};

pub use crate::style::time_picker::{Style, StyleSheet};

/// An input element for picking times.
///
/// # Example
/// ```
/// # use iced_aw::time_picker;
/// # use iced_native::{Button, Text, button, renderer::Null};
/// #
/// # pub type TimePicker<'a, Message> = iced_aw::native::TimePicker<'a, Message, Null>;
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(time_picker::Time),
/// }
///
/// let mut button_state = button::State::new();
/// let mut state = time_picker::State::now();
/// state.show(true);
///
/// let time_picker = TimePicker::new(
///     &mut state,
///     Button::new(&mut button_state, Text::new("Pick time"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct TimePicker<'a, Message, B>
where
    Message: Clone,
    B: Backend,
{
    /// The state of the [`TimePicker`](TimePicker).
    state: &'a mut State,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer<B>>,
    /// The message that is send if the cancel button of the [`TimePickerOverlay`](TimePickerOverlay) is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`](TimePickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Time) -> Message>,
    /// The style of the [`TimePickerOverlay`](TimePickerOverlay).
    style_sheet: Box<dyn StyleSheet + 'a>,
}

impl<'a, Message, B> TimePicker<'a, Message, B>
where
    Message: Clone,
    B: Backend,
{
    /// Creates a new [`TimePicker`](TimePicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * a mutable reference to the [`TimePicker`](TimePicker)'s [`State`](State).
    ///     * the underlay [`Element`](iced_native::Element) on which this [`TimePicker`](TimePicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`TimePicker`](TimePicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`TimePicker`](TimePicker)
    ///         is pressed, which takes the picked [`Time`](crate::time_picker::Time) value.
    pub fn new<U, F>(state: &'a mut State, underlay: U, on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message, Renderer<B>>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            //use_24h: false,
            //show_seconds: false,
            style_sheet: Default::default(),
        }
    }

    /// Use 24 hour format instead of AM/PM.
    pub fn use_24h(mut self) -> Self {
        self.state.overlay_state.use_24h = true;
        self
    }

    /// Enables the picker to also pick seconds.
    pub fn show_seconds(mut self) -> Self {
        self.state.overlay_state.show_seconds = true;
        self
    }

    /// Sets the style of the [`TimePicker`](TimePicker).
    pub fn style<S>(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

/// The state of the [`TimePicker`](TimePicker) / [`TimePickerOverlay`](TimePickerOverlay).
#[derive(Debug)]
pub struct State {
    /// The visibility of the overlay.
    pub(crate) show: bool,
    /// The state of the overlay.
    pub(crate) overlay_state: time_picker::State,
    /// The state of the cancel button.
    pub(crate) cancel_button: button::State,
    /// The state of the submit button.
    pub(crate) submit_button: button::State,
}

impl State {
    /// Creates a new [`State`](State) with the current time.
    #[must_use]
    pub fn now() -> Self {
        Self {
            show: false,
            cancel_button: button::State::new(),
            submit_button: button::State::new(),
            overlay_state: time_picker::State::default(),
        }
    }

    /// Sets the visibility of the [`TimePickerOverlay`](TimePickerOverlay).
    pub fn show(&mut self, b: bool) {
        self.overlay_state.focus = if b { Focus::Overlay } else { Focus::None };
        self.show = b;
    }

    /// Resets the time of the state to the current time.
    pub fn reset(&mut self) {
        self.overlay_state.clock_cache.clear();
        self.overlay_state.time = Local::now().naive_local().time();
        self.overlay_state.use_24h = false;
        self.overlay_state.show_seconds = false;
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for TimePicker<'a, Message, B>
where
    Message: Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn width(&self) -> iced_native::Length {
        self.underlay.width()
    }

    fn height(&self) -> iced_native::Length {
        self.underlay.height()
    }

    fn layout(
        &self,
        renderer: &Renderer<B>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer<B>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        self.underlay
            .on_event(event, layout, cursor_position, renderer, clipboard, shell)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer<B>,
    ) -> mouse::Interaction {
        self.underlay
            .mouse_interaction(layout, cursor_position, viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) {
        self.underlay
            .draw(renderer, style, layout, cursor_position, viewport);
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.underlay.hash_layout(state);
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer<B>,
    ) -> Option<overlay::Element<'_, Message, Renderer<B>>> {
        if !self.state.show {
            return self.underlay.overlay(layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            TimePickerOverlay::new(
                self.state,
                self.on_cancel.clone(),
                &self.on_submit,
                //self.use_24h,
                //self.show_seconds,
                position,
                &self.style_sheet,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, B> From<TimePicker<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
    Message: 'a + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn from(time_picker: TimePicker<'a, Message, B>) -> Self {
        Element::new(time_picker)
    }
}

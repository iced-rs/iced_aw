//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use std::hash::Hash;

use chrono::Local;
use iced_native::{
    button, column, container, event, overlay, row, text, Clipboard, Element, Event, Layout, Point,
    Widget,
};

pub use super::overlay::time_picker::Renderer;
use super::{
    icon_text,
    overlay::time_picker::{self, Focus, TimePickerOverlay},
};

pub use crate::core::time::{Period, Time};
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
pub struct TimePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: time_picker::Renderer + button::Renderer,
{
    /// The state of the [`TimePicker`](TimePicker).
    state: &'a mut State,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The message that is send if the cancel button of the [`TimePickerOverlay`](TimePickerOverlay) is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`](TimePickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Time) -> Message>,
    /// The style of the [`TimePickerOverlay`](TimePickerOverlay).
    style: <Renderer as time_picker::Renderer>::Style,
}

impl<'a, Message, Renderer> TimePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: time_picker::Renderer + button::Renderer,
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
        U: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            //use_24h: false,
            //show_seconds: false,
            style: <Renderer as time_picker::Renderer>::Style::default(),
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
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<<Renderer as time_picker::Renderer>::Style>,
    {
        self.style = style.into();
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for TimePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: time_picker::Renderer
        + button::Renderer
        + column::Renderer
        + container::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer,
{
    fn width(&self) -> iced_native::Length {
        self.underlay.width()
    }

    fn height(&self) -> iced_native::Length {
        self.underlay.height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        self.underlay.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) -> Renderer::Output {
        self.underlay
            .draw(renderer, defaults, layout, cursor_position, viewport)
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.underlay.hash_layout(state);
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        if !self.state.show {
            return self.underlay.overlay(layout);
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
                &self.style,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Renderer> From<TimePicker<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a
        + time_picker::Renderer
        + button::Renderer
        + column::Renderer
        + container::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer,
{
    fn from(time_picker: TimePicker<'a, Message, Renderer>) -> Self {
        Element::new(time_picker)
    }
}

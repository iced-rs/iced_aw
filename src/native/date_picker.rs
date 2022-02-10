//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*
use std::hash::Hash;

use chrono::Local;
use iced_native::{
    event, mouse, widget::button, Clipboard, Element, Event, Layout, Point, Rectangle, Shell,
    Widget,
};

use super::overlay::date_picker::{self, DatePickerOverlay, Focus};

pub use crate::core::date::Date;

pub use crate::style::date_picker::{Style, StyleSheet};

/// An input element for picking dates.
///
/// # Example
/// ```
/// # use iced_aw::date_picker;
/// # use iced_native::{Button, Text, button, renderer::Null};
/// #
/// # pub type DatePicker<'a, Message> = iced_aw::native::DatePicker<'a, Message, Null>;
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(date_picker::Date),
/// }
///
/// let mut button_state = button::State::new();
/// let mut state = date_picker::State::now();
/// state.show(true);
///
/// let date_picker = DatePicker::new(
///     &mut state,
///     Button::new(&mut button_state, Text::new("Pick date"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct DatePicker<'a, Message: Clone, Renderer: iced_native::Renderer> {
    /// The state of the [`DatePicker`](DatePicker).
    state: &'a mut State,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The message that is send if the cancel button of the [`DatePickerOverlay`](DatePickerOverlay) is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`DatePickerOverlay`](DatePickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Date) -> Message>,
    /// The style of the [`DatePickerOverlay`](DatePickerOverlay).
    style_sheet: Box<dyn StyleSheet>,
    //button_style: <Renderer as button::Renderer>::Style, // clone not satisfied
}

impl<'a, Message: Clone, Renderer: iced_native::Renderer> DatePicker<'a, Message, Renderer> {
    /// Creates a new [`DatePicker`](DatePicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * a mutable reference to the [`DatePicker`](DatePicker)'s [`State`](State).
    ///     * the underlay [`Element`](iced_native::Element) on which this [`DatePicker`](DatePicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`DatePicker`](DatePicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`DatePicker`](DatePicker)
    ///         is pressed, which takes the picked [`Date`](crate::date_picker::Date) value.
    pub fn new<U, F>(state: &'a mut State, underlay: U, on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(Date) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style_sheet: std::boxed::Box::default(),
            //button_style: <Renderer as button::Renderer>::Style::default(),
        }
    }

    /// Sets the style of the [`DatePicker`](DatePicker).
    pub fn style<S>(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        //self.button_style = style.into();
        self
    }
}

/// The state of the [`DatePicker`](DatePicker) / [`DatePickerOverlay`](DatePickerOverlay).
#[derive(Debug)]
pub struct State {
    /// The visibility of the overlay.
    pub(crate) show: bool,
    /// The state of the overlay.
    pub(crate) overlay_state: date_picker::State,
    /// The state of the cancel button.
    pub(crate) cancel_button: button::State,
    /// The state of the submit button.
    pub(crate) submit_button: button::State,
}

impl State {
    /// Creates a new [`State`](State) with the current date.
    #[must_use]
    pub fn now() -> Self {
        Self {
            show: false,
            overlay_state: date_picker::State::default(),
            cancel_button: button::State::new(),
            submit_button: button::State::new(),
        }
    }

    /// Sets the visibility of the [`DatePickerOverlay`](DatePickerOverlay).
    pub fn show(&mut self, b: bool) {
        self.overlay_state.focus = if b { Focus::Overlay } else { Focus::None };
        self.show = b;
    }

    /// Resets the date of the state to the current date.
    pub fn reset(&mut self) {
        self.overlay_state.date = Local::today().naive_local();
    }

    /// Set the date of the state to the given value.
    pub fn set_date(&mut self, year: i32, month: u32, day: u32) {
        self.overlay_state.date = chrono::NaiveDate::from_ymd(year, month, day);
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DatePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
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
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay
            .mouse_interaction(layout, cursor_position, viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
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

        self.state.show.hash(state);
        self.underlay.hash_layout(state);
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'_, Message, Renderer>> {
        if !self.state.show {
            return self.underlay.overlay(layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            DatePickerOverlay::new(
                self.state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.style_sheet,
                //self.button_style, // Clone not satisfied
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Renderer> From<DatePicker<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    fn from(date_picker: DatePicker<'a, Message, Renderer>) -> Self {
        Element::new(date_picker)
    }
}

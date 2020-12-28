//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
use std::hash::Hash;

use chrono::{Local, NaiveDate};
use iced_native::{Clipboard, Element, Event, Layout, Point, Widget, button, column, container, event, overlay, row, text};

use super::{icon_text, overlay::date_picker::{self, DatePickerOverlay}};
pub use super::overlay::date_picker::Renderer;

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
///     // Year, Month, Day
///     Submit(i32, u32, u32),
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
#[allow(missing_debug_implementations)]
pub struct DatePicker<'a, Message: Clone, Renderer: date_picker::Renderer + button::Renderer> {
    state: &'a mut State,
    underlay: Element<'a, Message, Renderer>,
    on_cancel: Message,
    on_submit: Box<dyn Fn(i32, u32, u32) -> Message>,
    style: <Renderer as date_picker::Renderer>::Style,
    //button_style: <Renderer as button::Renderer>::Style, // clone not satisfied
}

impl<'a, Message: Clone, Renderer: date_picker::Renderer + button::Renderer> DatePicker<'a, Message, Renderer> {
    /// Creates a new [`DatePicker`](DatePicker) wrapping around the given underlay.
    pub fn new<U, F>(
        state: &'a mut State,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(i32, u32, u32) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style: <Renderer as date_picker::Renderer>::Style::default(),
            //button_style: <Renderer as button::Renderer>::Style::default(),
        }
    }

    /// Sets the style of the [`DatePicker`](DatePicker).
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<<Renderer as date_picker::Renderer>::Style> // + Clone + Into<<Renderer as button::Renderer>::Style>,
    {
        self.style = style.into();
        //self.button_style = style.into();
        self
    }
}

/// The state of the [`DatePicker`](DatePicker) / [`DatePickerOverlay`](DatePickerOverlay).
#[derive(Debug)]
pub struct State {
    pub(crate) show: bool,
    pub(crate) date: NaiveDate,
    pub(crate) cancel_button: button::State,
    pub(crate) submit_button: button::State,
}

impl State {
    /// Creates a new [`State`](State) with the current date.
    pub fn now() -> Self {
        State {
            show: false,
            date: Local::today().naive_local(),
            cancel_button: button::State::new(),
            submit_button: button::State::new(),
        }
    }

    /// Sets the visibility of the [`DatePickerOverlay`](DatePickerOverlay).
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// Resets the date of the state to the current date.
    pub fn reset(&mut self) {
        self.date = Local::today().naive_local();
    }
}

impl <'a, Message, Renderer> Widget<Message, Renderer>
    for DatePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: date_picker::Renderer + button::Renderer + column::Renderer
        + container::Renderer + icon_text::Renderer + row::Renderer + text::Renderer,
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
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        self.underlay.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard
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
        self.underlay.draw(
            renderer,
            defaults,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.state.show.hash(state);
        self.underlay.hash_layout(state);
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        if !self.state.show { return self.underlay.overlay(layout); }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some (
            DatePickerOverlay::new(
                &mut self.state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.style,
                //self.button_style, // Clone not satisfied
            )
            .overlay()
        )
    }
}

impl<'a, Message, Renderer> From<DatePicker<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + date_picker::Renderer + button::Renderer + column::Renderer
        + container::Renderer + icon_text::Renderer + row::Renderer + text::Renderer,
{
    fn from(date_picker: DatePicker<'a, Message, Renderer>) -> Self {
        Element::new(date_picker)
    }
}
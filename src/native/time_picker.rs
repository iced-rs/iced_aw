//! TODO

use std::hash::Hash;

use chrono::{Local, NaiveTime};
use iced_native::{Clipboard, Element, Event, Layout, Point, Widget, button, column, container, event, overlay, row, text};
use iced_graphics::canvas;

use super::{icon_text, overlay::time_picker::{self, TimePickerOverlay}};
pub use super::overlay::time_picker::Renderer;

pub mod time;
pub use time::{Time, Period};
/// TODO
#[allow(missing_debug_implementations)]
pub struct TimePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: time_picker::Renderer + button::Renderer,
{
    state: &'a mut State,
    underlay: Element<'a, Message, Renderer>,
    on_cancel: Message,
    on_submit: Box<dyn Fn(Time) -> Message>,
    use_24h: bool,
    show_seconds: bool,
    style: <Renderer as time_picker::Renderer>::Style,
}

impl<'a, Message, Renderer> TimePicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: time_picker::Renderer + button::Renderer,
{
    /// TODO
    pub fn new<U, F>(
        state: &'a mut State,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            use_24h: false,
            show_seconds: false,
            style: <Renderer as time_picker::Renderer>::Style::default(),
        }
    }

    /// TODO
    pub fn use_24h(mut self) -> Self {
        self.use_24h = true;
        self
    }

    /// TODO
    pub fn show_seconds(mut self) -> Self {
        self.show_seconds = true;
        self
    }

    /// Sets the style of the [`TimePicker`](TimePicker).
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<<Renderer as time_picker::Renderer>::Style>
    {
        self.style = style.into();
        self
    }
}

/// TODO
#[derive(Debug)]
pub struct State {
    pub(crate) show: bool,
    pub(crate) time: NaiveTime,
    pub(crate) cancel_button: button::State,
    pub(crate) submit_button: button::State,
    pub(crate) clock_cache_needs_clearance: bool,
    pub(crate) clock_cache: canvas::Cache,
}

impl State {
    /// TODO
    pub fn now() -> Self {
        State {
            show: false,
            time: Local::now().naive_local().time(),
            cancel_button: button::State::new(),
            submit_button: button::State::new(),
            clock_cache_needs_clearance: false,
            clock_cache: canvas::Cache::new(),
        }
    }

    /// TODO
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// TODO
    pub fn reset(&mut self) {
        self.clock_cache.clear();
        self.time = Local::now().naive_local().time();
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for TimePicker<'a, Message, Renderer>
where 
    Message: Clone,
    Renderer: time_picker::Renderer + button::Renderer + column::Renderer + container::Renderer + icon_text::Renderer + row::Renderer + text::Renderer,
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
            clipboard,
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

        self.underlay.hash_layout(state);
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        if !self.state.show { return self.underlay.overlay(layout); }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some (
            TimePickerOverlay::new(
                &mut self.state,
                self.on_cancel.clone(),
                &self.on_submit,
                self.use_24h,
                self.show_seconds,
                position,
                &self.style,
            )
            .overlay()
        )
    }
}

impl<'a, Message, Renderer> From<TimePicker<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where 
    Message: 'a + Clone,
    Renderer: 'a + time_picker::Renderer + button::Renderer + column::Renderer + container::Renderer + icon_text::Renderer + row::Renderer + text::Renderer,
{
    fn from(time_picker: TimePicker<'a, Message, Renderer>) -> Self {
        Element::new(time_picker)
    }
}
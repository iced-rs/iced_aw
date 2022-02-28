//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*
use color_picker::ColorBarDragged;
use iced_graphics::{Backend, Renderer};
use iced_native::{
    event,
    layout::{Limits, Node},
    mouse, overlay, renderer,
    widget::button,
    Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

use super::overlay::color_picker::{self, ColorPickerOverlay, Focus};
pub use crate::style::color_picker::{Style, StyleSheet};

/// An input element for picking colors.
///
/// # Example
/// ```
/// # use iced_aw::color_picker;
/// # use iced_native::{Button, Color, Text, button, renderer::Null};
/// #
/// # pub type ColorPicker<'a, Message> = iced_aw::native::ColorPicker<'a, Message, Null>;
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(Color),
/// }
///
/// let mut button_state = button::State::new();
/// let mut state = color_picker::State::new();
/// state.show(true);
///
/// let color_picker = ColorPicker::new(
///     &mut state,
///     Button::new(&mut button_state, Text::new("Pick color"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ColorPicker<'a, Message, B>
where
    Message: Clone,
    B: Backend,
{
    /// The state of the [`ColorPicker`](ColorPicker).
    state: &'a mut State,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer<B>>,
    /// The message that is send if the cancel button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_cancel: Message,
    /// The function thet produces a message when the submit button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Color) -> Message>,
    /// The style of the [`ColorPickerOverlay`](ColorPickerOverlay).
    style_sheet: Box<dyn StyleSheet + 'a>,
}

impl<'a, Message, B> ColorPicker<'a, Message, B>
where
    Message: Clone,
    B: Backend,
{
    /// Creates a new [`ColorPicker`](ColorPicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * a mutable reference to the [`ColorPicker`](ColorPicker)'s [`State`](State).
    ///     * the underlay [`Element`](iced_native::Element) on which this [`ColorPicker`](ColorPicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`ColorPicker`](ColorPicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`ColorPicker`](ColorPicker)
    ///         is pressed, which takes the picked [`Color`](iced_native::Color) value.
    pub fn new<U, F>(state: &'a mut State, underlay: U, on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message, Renderer<B>>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style_sheet: std::boxed::Box::default(),
        }
    }

    /// Sets the style of the [`ColorPicker`](ColorPicker).
    #[must_use]
    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

/// The state of the [`ColorPicker`](ColorPicker).
#[derive(Debug, Default)]
pub struct State {
    /// The visibility of the overlay.
    pub(crate) show: bool,
    /// The state of the overlay.
    pub(crate) overlay_state: color_picker::State,
    /// The state of the cancel button.
    pub(crate) cancel_button: button::State,
    /// The state of the submit button.
    pub(crate) submit_button: button::State,
}

impl State {
    /// Creates a new [`State`](State).
    #[must_use]
    pub fn new() -> Self {
        Self {
            show: false,
            overlay_state: color_picker::State::default(),
            cancel_button: button::State::new(),
            submit_button: button::State::new(),
        }
    }

    /// Sets the visibility of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub fn show(&mut self, b: bool) {
        self.overlay_state.focus = if b { Focus::Overlay } else { Focus::None };
        self.show = b;
    }

    /// Resets the color of the state.
    pub fn reset(&mut self) {
        self.overlay_state.color = Color::from_rgb(0.5, 0.25, 0.25);
        self.overlay_state.color_bar_dragged = ColorBarDragged::None;
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for ColorPicker<'a, Message, B>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn width(&self) -> Length {
        self.underlay.width()
    }

    fn height(&self) -> Length {
        self.underlay.height()
    }

    fn layout(&self, renderer: &Renderer<B>, limits: &Limits) -> Node {
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
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) {
        self.underlay
            .draw(renderer, style, layout, cursor_position, viewport);
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
            ColorPickerOverlay::new(
                self.state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.style_sheet,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, B> From<ColorPicker<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
{
    fn from(color_picker: ColorPicker<'a, Message, B>) -> Self {
        Element::new(color_picker)
    }
}

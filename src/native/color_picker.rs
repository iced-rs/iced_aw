//! Use a color picker as an input element for picking colors.
//! 
//! *This API requires the following crate features to be activated: color_picker*
use std::hash::Hash;

use color_picker::ColorBarFocussed;
use iced_graphics::canvas;
use iced_native::{Clipboard, Color, Element, Event, Layout, Point, Widget, button, column, event, overlay, row, text_input};

pub use super::overlay::color_picker::Renderer;
use super::{icon_text, overlay::color_picker::{self, ColorPickerOverlay}};

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
pub struct ColorPicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: color_picker::Renderer,
{
    state: &'a mut State,
    underlay: Element<'a, Message, Renderer>,
    on_cancel: Message,
    on_submit: Box<dyn Fn(Color) -> Message>,
    style: <Renderer as color_picker::Renderer>::Style,
}

impl<'a, Message, Renderer> ColorPicker<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: color_picker::Renderer,
{
    /// Creates a new [`ColorPicker`](ColorPicker) wrapping around the given underlay.
    pub fn new<U, F>(state: &'a mut State, underlay: U, on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            state,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style: <Renderer as color_picker::Renderer>::Style::default(),
        }
    }

    /// Sets the style of the [`ColorPicker`](ColorPicker).
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<<Renderer as color_picker::Renderer>::Style>,
    {
        self.style = style.into();
        self
    }
}

/// The state of the [`ColorPicker`](ColorPicker) / [`ColorPickerOverlay`](ColorPickerOverlay).
#[derive(Debug)]
pub struct State {
    pub(crate) show: bool,
    pub(crate) color: Color,
    pub(crate) sat_value_canvas_cache: canvas::Cache,
    pub(crate) hue_canvas_cache: canvas::Cache,
    //pub(crate) color_hex: String,
    pub(crate) text_input: text_input::State,
    pub(crate) cancel_button: button::State,
    pub(crate) submit_button: button::State,
    pub(crate) color_bar_focussed: ColorBarFocussed,
}

impl State {
    /// Creates a new [`State`](State).
    pub fn new() -> Self {
        State {
            show: false,
            //color: Color::default(),
            color: Color::from_rgb(0.5, 0.25, 0.25),
            sat_value_canvas_cache: canvas::Cache::new(),
            hue_canvas_cache: canvas::Cache::new(),
            //color_hex: State::color_as_string(&Color::default()),
            text_input: text_input::State::new(),
            cancel_button: button::State::new(),
            submit_button: button::State::new(),
            color_bar_focussed: ColorBarFocussed::None,
        }
    }

    /// Sets the visibility of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// Resets the color of the state.
    pub fn reset(&mut self) {
        self.color = Color::from_rgb(0.5, 0.25, 0.25);
        self.color_bar_focussed = ColorBarFocussed::None;
    }

}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ColorPicker<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: color_picker::Renderer + button::Renderer + column::Renderer + icon_text::Renderer
        + row::Renderer + text_input::Renderer,
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
        self.underlay.draw(renderer, defaults, layout, cursor_position, viewport)
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
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
            ColorPickerOverlay::new(
                &mut self.state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.style,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Renderer> From<ColorPicker<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'a + color_picker::Renderer + button::Renderer + column::Renderer + icon_text::Renderer
        + row::Renderer + text_input::Renderer,
{
    fn from(color_picker: ColorPicker<'a, Message, Renderer>) -> Self {
        Element::new(color_picker)
    }
}
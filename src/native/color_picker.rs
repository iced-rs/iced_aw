//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use iced_widget::{
    button,
    canvas::{self, LineCap, Path, Stroke, Style},
    core::{
        alignment::{Horizontal, Vertical},
        event, keyboard,
        layout::{self, Limits, Node},
        mouse::{self, Cursor},
        overlay, renderer, text, touch,
        widget::{
            self,
            tree::{self, Tag, Tree},
        },
        Alignment, Clipboard, Color, Element, Event, Font, Layout, Length, Overlay, Padding, Point,
        Rectangle, Shell, Size, Vector, Widget,
    },
    graphics, Button, Column, Row, Text,
};

pub use crate::style::color_picker::{Appearance, StyleSheet};

use super::overlay::color_picker::{
    self, ColorBarDragged, ColorPickerOverlay, ColorPickerOverlayButtons,
};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking colors.
///
/// # Example
/// ```ignore
/// # use iced_aw::pure::color_picker;
/// # use iced_native::{Color, renderer::Null};
/// # use iced_pure::widget::{button, Button, Text};
/// #
/// # pub type ColorPicker<'a, Message> = iced_aw::pure::ColorPicker<'a, Message, Null>;
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(Color),
/// }
///
/// let color_picker = ColorPicker::new(
///     true,
///     Color::default(),
///     Button::new(Text::new("Pick color"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ColorPicker<'a, Message, Renderer = crate::Renderer>
where
    Message: Clone,
    Renderer: graphics::geometry::Renderer,
    Renderer::Theme: StyleSheet + button::StyleSheet,
{
    /// Show the picker.
    show_picker: bool,
    /// The color to show.
    color: Color,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The message that is send if the cancel button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_cancel: Message,
    /// The function thet produces a message when the submit button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Color) -> Message>,
    /// The style of the [`ColorPickerOverlay`](ColorPickerOverlay).
    style: <Renderer::Theme as StyleSheet>::Style,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> ColorPicker<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + graphics::geometry::Renderer + text::Renderer,
    Renderer::Theme: 'a + StyleSheet + button::StyleSheet,
{
    /// Creates a new [`ColorPicker`](ColorPicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the color picker is visible.
    ///     * the initial color to show.
    ///     * the underlay [`Element`](iced_pure::Element) on which this [`ColorPicker`](ColorPicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`ColorPicker`](ColorPicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`ColorPicker`](ColorPicker)
    ///         is pressed, which takes the picked [`Color`](iced_native::Color) value.
    pub fn new<U, F>(
        show_picker: bool,
        color: Color,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            show_picker,
            color,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            overlay_state: ColorPickerOverlayButtons::default().into(),
        }
    }

    /// Sets the style of the [`ColorPicker`](ColorPicker).
    #[must_use]
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

/// The state of the [`ColorPicker`](ColorPicker).
#[derive(Debug, Default)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: color_picker::State,
}

impl State {
    /// Creates a new [`State`](State).
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self {
            overlay_state: color_picker::State::new(color),
        }
    }

    /// Resets the color of the state.
    pub fn reset(&mut self) {
        self.overlay_state.color = Color::from_rgb(0.5, 0.25, 0.25);
        self.overlay_state.color_bar_dragged = ColorBarDragged::None;
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ColorPicker<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'a + graphics::geometry::Renderer + text::Renderer,
    Renderer::Theme: 'a + StyleSheet + button::StyleSheet + widget::text::StyleSheet,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.color))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay_state)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
        )
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

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
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
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let picker_state: &mut State = state.state.downcast_mut();

        if !self.show_picker {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            ColorPickerOverlay::new(
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

impl<'a, Message, Renderer> From<ColorPicker<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'a + graphics::geometry::Renderer + text::Renderer,
    Renderer::Theme: 'a + StyleSheet + button::StyleSheet + widget::text::StyleSheet,
{
    fn from(color_picker: ColorPicker<'a, Message, Renderer>) -> Self {
        Element::new(color_picker)
    }
}

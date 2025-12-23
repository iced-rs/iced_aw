//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use self::style::{Status, StyleFn};

use super::overlay::color_picker::{
    self, ColorBarDragged, ColorPickerOverlay, ColorPickerOverlayButtons,
};

use iced_core::{
    Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Vector, Widget,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::tree::{self, Tag, Tree},
};
use iced_widget::Renderer;

pub use crate::style::{self, color_picker::Style};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking colors.
///
/// # Example
/// ```ignore
/// # use iced_aw::ColorPicker;
/// # use iced::{Color, widget::{button, Button, Text}};
/// #
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
pub struct ColorPicker<'a, Message, Theme = iced_widget::Theme>
where
    Message: Clone,
    Theme: style::color_picker::Catalog + iced_widget::button::Catalog,
{
    /// Show the picker.
    show_picker: bool,
    /// The color to show.
    color: Color,
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The message that is sent if the cancel button of the [`ColorPickerOverlay`] is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`ColorPickerOverlay`] is pressed.
    on_submit: Box<dyn Fn(Color) -> Message>,
    /// The style of the [`ColorPickerOverlay`].
    class: <Theme as style::color_picker::Catalog>::Class<'a>,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme> ColorPicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog,
{
    /// Creates a new [`ColorPicker`] wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the color picker is visible.
    ///     * the initial color to show.
    ///     * the underlay [`Element`] on which this [`ColorPicker`]
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`ColorPicker`]
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`ColorPicker`]
    ///         is pressed, which takes the picked [`Color`] value.
    pub fn new<U, F>(
        show_picker: bool,
        color: Color,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            show_picker,
            color,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            class: <Theme as style::color_picker::Catalog>::default(),
            overlay_state: ColorPickerOverlayButtons::default().into(),
        }
    }

    /// Sets the style of the [`ColorPicker`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as style::color_picker::Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`ColorPicker`].
    #[must_use]
    pub fn class(
        mut self,
        class: impl Into<<Theme as style::color_picker::Catalog>::Class<'a>>,
    ) -> Self {
        self.class = class.into();
        self
    }
}

/// The state of the [`ColorPicker`].
#[derive(Debug, Default)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: color_picker::State,
    /// Was overlay shown during the previous render?
    pub(crate) old_show_picker: bool,
}

impl State {
    /// Creates a new [`State`].
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self {
            overlay_state: color_picker::State::new(color),
            old_show_picker: false,
        }
    }

    /// Resets the color of the state.
    pub fn reset(&mut self) {
        self.overlay_state.color = Color::from_rgb(0.5, 0.25, 0.25);
        self.overlay_state.color_bar_dragged = ColorBarDragged::None;
    }

    /// Synchronize with the provided color if it was changed or picker was reopened
    ///
    /// Keep the overlay state in sync. While overlay is open, it "owns" the value
    /// (there is no other way the user can update its value). When it is reopened,
    /// reset the color to the initial one.
    fn synchronize(&mut self, show_picker: bool, color: Color) {
        if show_picker && (!self.old_show_picker || self.overlay_state.initial_color != color) {
            self.overlay_state.force_synchronize(color);
        }
        self.old_show_picker = show_picker;
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for ColorPicker<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog,
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
        let color_picker_state = tree.state.downcast_mut::<State>();

        color_picker_state.synchronize(self.show_picker, self.color);

        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn size(&self) -> iced_core::Size<Length> {
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

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
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
            ColorPickerOverlay::new(
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

impl<'a, Message, Theme> From<ColorPicker<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'static + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog,
{
    fn from(color_picker: ColorPicker<'a, Message, Theme>) -> Self {
        Element::new(color_picker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    enum TestMessage {
        Cancel,
        Submit(Color),
    }

    type TestColorPicker<'a> = ColorPicker<'a, TestMessage, iced_widget::Theme>;

    fn create_test_button() -> iced_widget::Button<'static, TestMessage, iced_widget::Theme> {
        iced_widget::button(iced_widget::text::Text::new("Pick"))
    }

    #[test]
    fn color_picker_new_with_picker_hidden() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            color,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(!picker.show_picker);
        assert_eq!(picker.color, color);
    }

    #[test]
    fn color_picker_new_with_picker_shown() {
        let color = Color::from_rgb(0.3, 0.6, 0.9);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            true,
            color,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(picker.show_picker);
        assert_eq!(picker.color, color);
    }

    #[test]
    fn color_picker_with_black_color() {
        let black = Color::from_rgb(0.0, 0.0, 0.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            black,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, black);
    }

    #[test]
    fn color_picker_with_white_color() {
        let white = Color::from_rgb(1.0, 1.0, 1.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            white,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, white);
    }

    #[test]
    fn color_picker_with_rgba_color() {
        let semi_transparent = Color::from_rgba(1.0, 0.0, 0.0, 0.5);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            semi_transparent,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, semi_transparent);
    }

    #[test]
    fn color_picker_default_show_picker_false() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            color,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(!picker.show_picker);
    }

    #[test]
    fn color_picker_with_various_colors() {
        let colors = vec![
            Color::from_rgb(0.1, 0.2, 0.3),
            Color::from_rgb(0.5, 0.5, 0.5),
            Color::from_rgb(0.9, 0.8, 0.7),
            Color::from_rgba(0.5, 0.5, 0.5, 0.5),
        ];

        for color in colors {
            let button = create_test_button();
            let picker = TestColorPicker::new(
                false,
                color,
                button,
                TestMessage::Cancel,
                TestMessage::Submit,
            );

            assert_eq!(picker.color, color);
        }
    }

    #[test]
    fn color_picker_state_new() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let state = State::new(color);

        assert!(!state.old_show_picker);
    }

    #[test]
    fn color_picker_state_default() {
        let state = State::default();

        assert!(!state.old_show_picker);
    }

    #[test]
    fn color_picker_state_reset() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let mut state = State::new(color);

        state.reset();
        // State should still be valid after reset
        assert!(!state.old_show_picker);
    }

    #[test]
    fn color_picker_with_red_color() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            red,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, red);
    }

    #[test]
    fn color_picker_with_green_color() {
        let green = Color::from_rgb(0.0, 1.0, 0.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            green,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, green);
    }

    #[test]
    fn color_picker_with_blue_color() {
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            blue,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, blue);
    }

    #[test]
    fn color_picker_precision_colors() {
        let precise_color = Color::from_rgb(0.123456, 0.789012, 0.345678);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            precise_color,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, precise_color);
    }

    #[test]
    fn color_picker_fully_transparent() {
        let transparent = Color::from_rgba(0.0, 0.0, 0.0, 0.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            transparent,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, transparent);
    }

    #[test]
    fn color_picker_fully_opaque() {
        let opaque = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
        let button = create_test_button();

        let picker = TestColorPicker::new(
            false,
            opaque,
            button,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert_eq!(picker.color, opaque);
    }

    #[test]
    fn color_picker_show_picker_toggle() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let button1 = create_test_button();
        let button2 = create_test_button();

        let picker_hidden = TestColorPicker::new(
            false,
            color,
            button1,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let picker_shown = TestColorPicker::new(
            true,
            color,
            button2,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(!picker_hidden.show_picker);
        assert!(picker_shown.show_picker);
    }
}

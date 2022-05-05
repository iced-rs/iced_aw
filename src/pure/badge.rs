//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use crate::native::badge;
use iced_native::{
    event, layout, mouse, renderer, Alignment, Clipboard, Color, Event, Layout, Length, Point,
    Rectangle, Shell,
};
use iced_pure::{widget::Tree, Element, Widget};

pub use crate::style::badge::{Style, StyleSheet};

/// The ratio of the border radius.
const BORDER_RADIUS_RATIO: f32 = 34.0 / 15.0;

/// A badge for color highlighting small information.
///
/// # Example
/// ```
/// # use iced_aw::style::badge;
/// # use iced_native::{widget::Text, renderer::Null};
/// #
/// # pub type Badge<'a, Message> = iced_aw::native::Badge<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
/// }
///
/// let badge = Badge::<Message>::new(Text::new("Text"));
/// ```
#[allow(missing_debug_implementations)]
pub struct Badge<'a, Message, Renderer> {
    /// The padding of the [`Badge`].
    padding: u16,
    /// The width of the [`Badge`].
    width: Length,
    /// The height of the [`Badge`].
    height: Length,
    /// The horizontal alignment of the [`Badge`](Badge).
    horizontal_alignment: Alignment,
    /// The vertical alignment of the [`Badge`](Badge).
    vertical_alignment: Alignment,
    /// The style of the [`Badge`](Badge).
    style_sheet: Box<dyn StyleSheet + 'a>,
    /// The content [`Element`](iced_native::Element) of the [`Badge`](Badge).
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Badge<'a, Message, Renderer> {
    /// Creates a new [`Badge`](Badge) with the given content.
    ///
    /// It expects:
    ///     * the content [`Element`](iced_native::Element) to display in the [`Badge`](Badge).
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Badge {
            padding: 7,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: Alignment::Center,
            vertical_alignment: Alignment::Center,
            style_sheet: std::boxed::Box::default(),
            content: content.into(),
        }
    }

    /// Sets the padding of the [`Badge`](Badge).
    #[must_use]
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Badge`](Badge).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Badge`](Badge).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the horizontal alignment of the content of the [`Badge`](Badge).
    #[must_use]
    pub fn align_x(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the content of the [`Badge`](Badge).
    #[must_use]
    pub fn align_y(mut self, alignment: Alignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the style of the [`Badge`](Badge).
    #[must_use]
    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Badge<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        badge::layout(
            renderer,
            limits,
            self.width,
            self.height,
            self.padding.into(),
            self.horizontal_alignment,
            self.vertical_alignment,
            |renderer, limits| self.content.as_widget().layout(renderer, limits),
        )
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout
                .children()
                .next()
                .expect("Native: Layout should have a children layout for a badge."),
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
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &iced_pure::widget::Tree,
        renderer: &mut Renderer,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let is_mouse_over = bounds.contains(cursor_position);
        let style_sheet = if is_mouse_over {
            self.style_sheet.hovered()
        } else {
            self.style_sheet.active()
        };

        //println!("height: {}", bounds.height);
        // 34 15
        //  x
        let border_radius = style_sheet
            .border_radius
            .unwrap_or(bounds.height as f32 / BORDER_RADIUS_RATIO);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius,
                border_width: style_sheet.border_width,
                border_color: style_sheet.border_color.unwrap_or(Color::BLACK),
            },
            style_sheet.background,
        );

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            &renderer::Style {
                text_color: style_sheet.text_color,
            },
            children
                .next()
                .expect("Graphics: Layout should have a children layout for Badge"),
            cursor_position,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<Badge<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer,
    Message: 'a,
{
    fn from(badge: Badge<'a, Message, Renderer>) -> Self {
        Element::new(badge)
    }
}

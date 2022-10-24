//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use iced_native::{
    event, layout, mouse,
    renderer::{self},
    widget::Tree,
    Alignment, Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

pub use crate::style::badge::{Appearance, StyleSheet};

/// The ratio of the border radius.
const BORDER_RADIUS_RATIO: f32 = 34.0 / 15.0;

/// A badge for color highlighting small information.
///
/// # Example
/// ```
/// # use iced_aw::style::badge;
/// # use iced_native::{widget::Text, renderer::Null};
/// #
/// # pub type Badge<'a, Message> = iced_aw::Badge<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
/// }
///
/// let badge = Badge::<Message>::new(Text::new("Text"));
/// ```
#[allow(missing_debug_implementations)]
pub struct Badge<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
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
    style: <Renderer::Theme as StyleSheet>::Style,
    /// The content [`Element`](iced_native::Element) of the [`Badge`](Badge).
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Badge<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
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
            style: <Renderer::Theme as StyleSheet>::Style::default(),
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
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Badge<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
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
        let padding = self.padding.into();
        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let mut content = self.content.as_widget().layout(renderer, &limits.loose());
        let size = limits.resolve(content.size());

        content.move_to(Point::new(f32::from(padding.left), f32::from(padding.top)));
        content.align(self.horizontal_alignment, self.vertical_alignment, size);

        layout::Node::with_children(size.pad(padding), vec![content])
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
        tree: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let is_mouse_over = bounds.contains(cursor_position);
        let style_sheet = if is_mouse_over {
            theme.hovered(self.style)
        } else {
            theme.active(self.style)
        };

        //println!("height: {}", bounds.height);
        // 34 15
        //  x
        let border_radius = style_sheet
            .border_radius
            .unwrap_or(bounds.height / BORDER_RADIUS_RATIO);

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
            &tree.children[0],
            renderer,
            theme,
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
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(badge: Badge<'a, Message, Renderer>) -> Self {
        Self::new(badge)
    }
}

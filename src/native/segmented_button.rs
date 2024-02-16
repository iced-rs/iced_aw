//! Create choices using `segnmented_button` buttons.
use iced::{
    self,
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Clipboard,
        Layout,
        Shell,
        Widget,
    },
    event, mouse::{self, Cursor}, touch,
    Alignment, Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle,
    Shadow, Size,
};

pub use crate::style::segmented_button::StyleSheet;

/// A  `segnmented_button` for color highlighting small information.
///
/// # Example
/// ```ignore
/// # use iced::widget::Text;
/// # use iced_aw::SegmentedButton;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
/// }
///
/// let segmented_button = SegmentedButton::<Message>::new(Text::new("Text"));
/// ```
#[allow(missing_debug_implementations)]
pub struct SegmentedButton<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: StyleSheet,
{
    is_selected: bool,
    on_click: Message,
    /// The padding of the [`SegmentedButton`].
    padding: Padding,
    /// The width of the [`SegmentedButton`].
    width: Length,
    /// The height of the [`SegmentedButton`].
    height: Length,
    /// The horizontal alignment of the [`SegmentedButton`]
    horizontal_alignment: Alignment,
    /// The vertical alignment of the [`SegmentedButton`]
    vertical_alignment: Alignment,
    /// The style of the [`SegmentedButton`]
    style: <Theme as StyleSheet>::Style,
    /// The content [`Element`] of the [`SegmentedButton`]
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> SegmentedButton<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: StyleSheet,
{
    /// Creates a new [`SegmentedButton`](SegmentedButton) with the given content.
    ///
    /// It expects:
    ///     * the content [`Element`] to display in the [`SegmentedButton`](SegmentedButton).
    pub fn new<T, F, V>(content: T, value: V, selected: Option<V>, f: F) -> Self
    where
        T: Into<Element<'a, Message, Theme, Renderer>>,
        V: Eq + Copy,
        F: FnOnce(V) -> Message,
    {
        SegmentedButton {
            is_selected: Some(value) == selected,
            on_click: f(value),
            padding: Padding::new(3.0),
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: Alignment::Center,
            vertical_alignment: Alignment::Center,
            style: <Theme as StyleSheet>::Style::default(),
            content: content.into(),
        }
    }

    /// Sets the padding of the [`SegmentedButton`](SegmentedButton).
    #[must_use]
    pub fn padding(mut self, units: Padding) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`SegmentedButton`](SegmentedButton).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`SegmentedButton`](SegmentedButton).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the horizontal alignment of the content of the [`SegmentedButton`](SegmentedButton).
    #[must_use]
    pub fn align_x(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the content of the [`SegmentedButton`](SegmentedButton).
    #[must_use]
    pub fn align_y(mut self, alignment: Alignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the style of the [`SegmentedButton`](SegmentedButton).
    #[must_use]
    pub fn style(mut self, style: <Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for SegmentedButton<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = self.padding;
        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height)
            .shrink(padding);

        let mut content =
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, &limits.loose());
        let size = limits.resolve(self.width, self.height, content.size());

        content.move_to_mut(Point::new(padding.left, padding.top));
        content.align_mut(self.horizontal_alignment, self.vertical_alignment, size);

        Node::with_children(size.expand(padding), vec![content])
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish(self.on_click.clone());

                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let is_mouse_over = bounds.contains(cursor.position().unwrap_or_default());
        let style_sheet = if is_mouse_over {
            theme.hovered(&self.style)
        } else {
            theme.active(&self.style)
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: 2.0.into(),
                    width: style_sheet.border_width,
                    color: style_sheet.border_color.unwrap_or(Color::BLACK),
                },
                shadow: Shadow::default(),
            },
            style_sheet.background,
        );
        if self.is_selected {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y,
                        width: bounds.width,
                        height: bounds.height,
                    },
                    border: Border {
                        radius: 2.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                },
                style_sheet.selected_color,
            );
        }
        //just for the testing as of now needs to clearup and make styling based of basecolor
        if is_mouse_over && !self.is_selected {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y,
                        width: bounds.width,
                        height: bounds.height,
                    },
                    border: Border {
                        radius: 2.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                },
                Background::Color([0.0, 0.0, 0.0, 0.5].into()),
            );
        }

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style_sheet.text_color,
            },
            children
                .next()
                .expect("Graphics: Layout should have a children layout for SegmentedButton"),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<SegmentedButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a + StyleSheet,
{
    fn from(segmented_button: SegmentedButton<'a, Message, Theme, Renderer>) -> Self {
        Self::new(segmented_button)
    }
}

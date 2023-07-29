//! Text widget for rendering icons.
//!
//! Nearly a complete copy of the `iced_widget::Text` widget, but with the
//! icons font as a default font. Maybe I'll find a better way in the future.
//!
//! //! *This API requires the following crate features to be activated: `icon_text`*
use iced_widget::{
    core::{
        self,
        alignment::{Horizontal, Vertical},
        layout::{Limits, Node},
        mouse::Cursor,
        renderer,
        widget::Tree,
        Color, Element, Layout, Length, Rectangle, Widget,
    },
    text::LineHeight,
};

use crate::graphics::icons::ICON_FONT;
/// Text widget with icon font.
#[allow(missing_debug_implementations)]
pub struct IconText<Renderer: core::text::Renderer<Font = core::Font> = crate::Renderer> {
    /// The content of the [`IconText`](IconText).
    content: String,
    /// The optional size of the [`IconText`](IconText).
    size: Option<f32>,
    /// The optional color of the [`IconText`](IconText).
    color: Option<Color>,
    /// The optional font of the [`IconText`](IconText).
    font: Option<Renderer::Font>,
    /// The width of the [`IconText`](IconText).
    width: Length,
    /// The height of the [`IconText`](IconText).
    height: Length,
    /// The horizontal alignment of the [`IconText`](IconText).
    horizontal_alignment: Horizontal,
    /// The vertical alignment of the [`IconText`](IconText).
    vertical_alignment: Vertical,
}

impl<Renderer: core::text::Renderer<Font = core::Font>> IconText<Renderer> {
    /// Creates a new [`IconText`](IconText) with the given icon label.
    ///
    /// It expects:
    ///     * the label to be displayed as an icon on the [`IconText`](IconText).
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            content: label.into(),
            size: None,
            color: None,
            font: None,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        }
    }

    /// Sets the size of the [`IconText`](IconText).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the [`Color`](core::Color) of the [`IconText`](IconText).
    #[must_use]
    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the [`Font`](core::Font) of the [`IconText`](IconText).
    #[must_use]
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the width of the [`IconText`](IconText) boundaries.
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`IconText`](IconText) boundaries.
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the [`Horizontal `](core::alignment::Horizontal )
    /// of the [`IconText`](IconText).
    #[must_use]
    pub fn horizontal_alignment(mut self, alignment: Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`Vertical `](core::alignment::Vertical )
    /// of the [`IconText`](IconText).
    #[must_use]
    pub fn vertical_alignment(mut self, alignment: Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for IconText<Renderer>
where
    Renderer: core::Renderer + core::text::Renderer<Font = core::Font>,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let bounds = limits.max();

        let size = renderer.measure(
            &self.content,
            size,
            LineHeight::Relative(1.3),
            self.font.unwrap_or_default(),
            bounds,
            iced_widget::text::Shaping::Advanced,
        );

        let size = limits.resolve(size);

        Node::new(size)
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let x = match self.horizontal_alignment {
            Horizontal::Left => bounds.x,
            Horizontal::Center => bounds.center_x(),
            Horizontal::Right => bounds.x + bounds.width,
        };

        let y = match self.vertical_alignment {
            Vertical::Top => bounds.y,
            Vertical::Center => bounds.center_y(),
            Vertical::Bottom => bounds.y + bounds.height,
        };

        renderer.fill_text(core::text::Text {
            content: &self.content,
            bounds: Rectangle { x, y, ..bounds },
            size: self.size.unwrap_or_else(|| renderer.default_size()),
            color: self.color.unwrap_or(style.text_color),
            font: self.font.unwrap_or(ICON_FONT),
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
            line_height: LineHeight::Relative(1.3),
            shaping: iced_widget::text::Shaping::Advanced,
        });
    }
}

impl<'a, Message, Renderer> From<IconText<Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: core::Renderer + core::text::Renderer<Font = core::Font> + 'a,
{
    fn from(icon: IconText<Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(icon)
    }
}

impl<Renderer: core::text::Renderer<Font = core::Font>> Clone for IconText<Renderer> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            size: self.size,
            color: self.color,
            font: self.font,
            width: self.width,
            height: self.height,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
        }
    }
}

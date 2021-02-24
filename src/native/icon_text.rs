//! Text widget for rendering icons.
//!
//! Nearly a complete copy of the `iced_native::Text` widget, but with the
//! icons font as a default font. Maybe I'll find a better way in the future.
//!
//! //! *This API requires the following crate features to be activated: `icon_text`*
use std::hash::Hash;

use iced_native::{
    Color, Element, HorizontalAlignment, Length, Rectangle, Size, VerticalAlignment, Widget,
};

/// Text widget with icon font.
#[allow(missing_debug_implementations)]
pub struct IconText<Renderer: self::Renderer> {
    content: String,
    size: Option<u16>,
    color: Option<Color>,
    font: Option<Renderer::Font>,
    width: Length,
    height: Length,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<Renderer: self::Renderer> IconText<Renderer> {
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
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        }
    }

    /// Sets the size of the [`IconText`](IconText).
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the [`Color`](iced_native::Color) of the [`IconText`](IconText).
    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the [`Font`](iced_native::Font) of the [`IconText`](IconText).
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the width of the [`IconText`](IconText) boundaries.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`IconText`](IconText) boundaries.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the [`HorizontalAlignment`](iced_native::HorizontalAlignment)
    /// of the [`IconText`](IconText).
    pub fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`VerticalAlignment`](iced_native::VerticalAlignment)
    /// of the [`IconText`](IconText).
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for IconText<Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let bounds = limits.max();

        let (width, height) = renderer.measure(
            &self.content,
            size,
            self.font.unwrap_or_else(|| renderer.default_font()),
            bounds,
        );

        let size = limits.resolve(Size::new(width, height));

        iced_native::layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        _cursor_position: iced_graphics::Point,
        _viewport: &iced_graphics::Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            layout.bounds(),
            &self.content,
            self.size.unwrap_or_else(|| renderer.default_size()),
            self.font,
            self.color,
            self.horizontal_alignment,
            self.vertical_alignment,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.content.hash(state);
        self.size.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

/// The renderer of an [`IconText`](IconText).
///
/// Your renderer will need to implement this trait before being
/// able to use an [`IconText`](IconText) in your user interface.
pub trait Renderer: iced_native::Renderer {
    /// The font type used for [`IconText`](IconText).
    type Font: Default + Copy;

    /// Returns the default size of [`IconText`](IconText).
    fn default_size(&self) -> u16;

    /// Returns the default font of [`IconText`](IconText).
    fn default_font(&self) -> Self::Font;

    /// Measures the [`IconText`](IconText) in the given bounds and returns the
    /// minimum boundaries that can fit the contents.
    fn measure(&self, content: &str, size: u16, font: Self::Font, bounds: Size) -> (f32, f32);

    /// Draws an [`IconText`](IconText).
    #[allow(clippy::too_many_arguments)]
    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        content: &str,
        size: u16,
        font: Option<Self::Font>,
        color: Option<Color>,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<IconText<Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer + 'a,
{
    fn from(icon: IconText<Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(icon)
    }
}

impl<Renderer: self::Renderer> Clone for IconText<Renderer> {
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

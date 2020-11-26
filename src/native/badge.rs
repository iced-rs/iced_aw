//! Use a badge for color highlighting important information.
//! 
//! *This API requires the following crate features to be activated: badge*
use std::hash::Hash;

use iced_native::{Align, Clipboard, Element, Event, Layout, Length, Point, Rectangle, Widget, event, layout};

/// A badge for color highlighting small information.
/// 
/// # Example
/// ```
/// # use iced_aw::style::badge;
/// # use iced_native::{Text, renderer::Null};
/// # 
/// # pub type Badge<'a, Message> = iced_aw::native::Badge<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
/// }
/// 
/// let badge = Badge::<Message>::new(Text::new("Text"));
/// ```
#[allow(missing_debug_implementations)]
pub struct Badge<'a, Message, Renderer: self::Renderer> {
    padding: u16,
    width: Length,
    height: Length,
    horizontal_alignment: Align,
    vertical_alignment: Align,
    style: Renderer::Style,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Badge<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// Creates a new [`Badge`](Badge) with the given content.
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Badge {
            padding: 7,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: Align::Center,
            vertical_alignment: Align::Center,
            style: Renderer::Style::default(),
            content: content.into(),
        }
    }

    /// Sets the padding of the [`Badge`](Badge).
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Badge`](Badge).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Badge`](Badge).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the horizontal alignment of the content of the [`Badge`](Badge).
    pub fn align_x(mut self, alignment: Align) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the content of the [`Badge`](Badge).
    pub fn align_y(mut self, alignment: Align) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the style of the [`Badge`](Badge).
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Badge<'a, Message, Renderer>
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
        let padding = f32::from(self.padding);

        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let mut content = self.content.layout(renderer, &limits.loose());
        let size = limits.resolve(content.size());

        content.move_to(Point::new(padding, padding));
        content.align(self.horizontal_alignment, self.vertical_alignment, size);

        layout::Node::with_children(size.pad(padding), vec![content])
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
        self.content.on_event(
            event,
            layout.children().next().unwrap(),
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
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            cursor_position,
            &self.style,
            &self.content,
            layout,
            viewport,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.padding.hash(state);
        self.width.hash(state);
        self.height.hash(state);

        self.content.hash_layout(state);
    }
}

/// The renderer of a [`Badge`](Badge).
/// 
/// Your renderer will need to implement this trait before being
/// able to use a [`Badge`](Badge) in your user interface.
pub trait Renderer: iced_native::Renderer {

    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Badge`](Badge).
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        style_sheet: &Self::Style,
        content: &Element<'_, Message, Self>,
        layout: Layout<'_>,
        viewport: &Rectangle,
    ) -> Self::Output;
}

impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        _cursor_position: Point,
        _style_sheet: &Self::Style,
        _content: &Element<'_, Message, Self>,
        _layout: Layout<'_>,
        _viewport: &Rectangle,
    ) -> Self::Output {}
}

impl<'a, Message, Renderer> From<Badge<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(badge: Badge<'a, Message, Renderer>) -> Self {
        Element::new(badge)
    }
}
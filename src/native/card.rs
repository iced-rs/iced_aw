//! Displays a [`Card`](Card).
//!
//! *This API requires the following crate features to be activated: card*
use std::hash::Hash;

use iced_native::{event, touch, Clipboard, Element, Event, Layout, Length, Point, Size, Widget};
use iced_native::{mouse, Align};

use crate::core::renderer::DrawEnvironment;

/// A card consisting of a head, body and optional foot.
///
/// # Example
/// ```
/// # use iced_native::{renderer::Null, Text};
/// #
/// # pub type Card<'a, Message> = iced_aw::native::Card<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     ClosingCard,
/// }
///
/// let card = Card::new(
///     Text::new("Head"),
///     Text::new("Body")
/// )
/// .foot(Text::new("Foot"))
/// .on_close(Message::ClosingCard);
///
/// ```
#[allow(missing_debug_implementations)]
pub struct Card<'a, Message, Renderer: self::Renderer> {
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    padding_head: f32,
    padding_body: f32,
    padding_foot: f32,
    close_size: Option<f32>,
    on_close: Option<Message>,
    head: Element<'a, Message, Renderer>,
    body: Element<'a, Message, Renderer>,
    foot: Option<Element<'a, Message, Renderer>>,
    style: <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> Card<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// Creates a new [`Card`](Card) containing the given head and body.
    ///
    /// It expects:
    ///     * the head [`Element`](iced_native::Element) to display at the top of
    ///         the [`Card`](Card).
    ///     * the body [`Element`](iced_native::Element) to display at the middle
    ///         of the [`Card`](Card).
    pub fn new<H, B>(head: H, body: B) -> Self
    where
        H: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
    {
        Card {
            width: Length::Fill,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            padding_head: <Renderer as self::Renderer>::DEFAULT_PADDING,
            padding_body: <Renderer as self::Renderer>::DEFAULT_PADDING,
            padding_foot: <Renderer as self::Renderer>::DEFAULT_PADDING,
            close_size: None,
            on_close: None,
            head: head.into(),
            body: body.into(),
            foot: None,
            style: <Renderer as self::Renderer>::Style::default(),
        }
    }

    /// Sets the [`Element`](iced_native::Element) of the foot of the
    /// [`Card`](Card).
    pub fn foot<F>(mut self, foot: F) -> Self
    where
        F: Into<Element<'a, Message, Renderer>>,
    {
        self.foot = Some(foot.into());
        self
    }

    /// Sets the width of the [`Card`](Card).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Card`](Card).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Card`](Card).
    pub fn max_width(mut self, width: u32) -> Self {
        self.max_width = width;
        self
    }

    /// Sets the maximum height of the [`Card`](Card).
    pub fn max_height(mut self, height: u32) -> Self {
        self.max_height = height;
        self
    }

    /// Sets the padding of the [`Card`](Card).
    ///
    /// This will set the padding of the head, body and foot to the
    /// same value.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding_head = padding;
        self.padding_body = padding;
        self.padding_foot = padding;
        self
    }

    /// Sets the padding of the head of the [`Card`](Card).
    pub fn padding_head(mut self, padding: f32) -> Self {
        self.padding_head = padding;
        self
    }

    /// Sets the padding of the body of the [`Card`](Card).
    pub fn padding_body(mut self, padding: f32) -> Self {
        self.padding_body = padding;
        self
    }

    /// Sets the padding of the foot of the [`Card`](Card).
    pub fn padding_foot(mut self, padding: f32) -> Self {
        self.padding_foot = padding;
        self
    }

    /// Sets the size of the close icon of the [`Card`](Card).
    pub fn close_size(mut self, size: f32) -> Self {
        self.close_size = Some(size);
        self
    }

    /// Sets the message that will be produced when the close icon of the
    /// [`Card`](Card) is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the [`Card`](Card).
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    /// Sets the style of the [`Card`](Card).
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Card<'a, Message, Renderer>
where
    Message: Clone,
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
        let limits = limits
            .clone()
            .max_width(self.max_width)
            .max_height(self.max_height);

        let head_node = head_node(
            renderer,
            &limits,
            &self.head,
            self.padding_head,
            self.width,
            self.on_close.is_some(),
            self.close_size,
        );

        let mut body_node = body_node(renderer, &limits, &self.body, self.padding_body, self.width);

        body_node.move_to(Point::new(
            body_node.bounds().x,
            body_node.bounds().y + head_node.bounds().height,
        ));

        let mut foot_node = self
            .foot
            .as_ref()
            .map_or_else(iced_native::layout::Node::default, |foot| {
                foot_node(renderer, &limits, foot, self.padding_foot, self.width)
            });

        foot_node.move_to(Point::new(
            foot_node.bounds().x,
            foot_node.bounds().y + head_node.bounds().height + body_node.bounds().height,
        ));

        iced_native::layout::Node::with_children(
            Size::new(
                body_node.size().width,
                head_node.size().height + body_node.size().height + foot_node.size().height,
            ),
            vec![head_node, body_node, foot_node],
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut children = layout.children();

        let head_layout = children.next().unwrap();
        let mut head_children = head_layout.children();
        let head_status = self.head.on_event(
            event.clone(),
            head_children.next().unwrap(),
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let close_status = head_children
            .next()
            .map_or(event::Status::Ignored, |close_layout| {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. }) => self
                        .on_close
                        .to_owned()
                        // TODO: `let` expressions in this position are experimental
                        // see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
                        .filter(|_| close_layout.bounds().contains(cursor_position))
                        .map_or(event::Status::Ignored, |on_close| {
                            messages.push(on_close);
                            event::Status::Captured
                        }),
                    _ => event::Status::Ignored,
                }
            });

        let body_layout = children.next().unwrap();
        let mut body_children = body_layout.children();
        let body_status = self.body.on_event(
            event.clone(),
            body_children.next().unwrap(),
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let foot_layout = children.next().unwrap();
        let mut foot_children = foot_layout.children();
        let foot_status = self.foot.as_mut().map_or(event::Status::Ignored, |foot| {
            foot.on_event(
                event,
                foot_children.next().unwrap(),
                cursor_position,
                messages,
                renderer,
                clipboard,
            )
        });

        head_status
            .merge(close_status)
            .merge(body_status)
            .merge(foot_status)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &self.style,
                viewport: Some(viewport),
                focus: (),
            },
            &self.head,
            &self.body,
            &self.foot,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);
        self.head.hash_layout(state);
        self.body.hash_layout(state);
        if let Some(foot) = self.foot.as_ref() {
            foot.hash_layout(state)
        };
    }
}

fn head_node<'a, Message, Renderer>(
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    head: &Element<'a, Message, Renderer>,
    padding: f32,
    width: Length,
    on_close: bool,
    close_size: Option<f32>,
) -> iced_native::layout::Node
where
    Renderer: self::Renderer,
{
    let mut limits = limits
        .loose()
        .width(width)
        .height(head.height())
        .pad(padding);

    let close_size = close_size.unwrap_or_else(|| renderer.default_size());
    let mut close = if on_close {
        limits = limits.shrink(Size::new(close_size, 0.0));
        Some(iced_native::layout::Node::new(Size::new(
            close_size, close_size,
        )))
    } else {
        None
    };

    let mut head = head.layout(renderer, &limits);
    let mut size = limits.resolve(head.size());

    head.move_to(Point::new(padding, padding));
    head.align(Align::Start, Align::Center, head.size());

    if let Some(node) = close.as_mut() {
        size = Size::new(size.width + close_size, size.height);

        node.move_to(Point::new(size.width - padding, padding));
        node.align(Align::End, Align::Center, node.size())
    }

    iced_native::layout::Node::with_children(
        size.pad(padding),
        match close {
            Some(node) => vec![head, node],
            None => vec![head],
        },
    )
}

fn body_node<'a, Message, Renderer>(
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    body: &Element<'a, Message, Renderer>,
    padding: f32,
    width: Length,
) -> iced_native::layout::Node
where
    Renderer: self::Renderer,
{
    let limits = limits
        .clone()
        .loose()
        .width(width)
        .height(body.height())
        .pad(padding);

    let mut body = body.layout(renderer, &limits);
    let size = limits.resolve(body.size());

    body.move_to(Point::new(padding, padding));
    body.align(Align::Start, Align::Start, size);

    iced_native::layout::Node::with_children(size.pad(padding), vec![body])
}

fn foot_node<'a, Message, Renderer>(
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    foot: &Element<'a, Message, Renderer>,
    padding: f32,
    width: Length,
) -> iced_native::layout::Node
where
    Renderer: self::Renderer,
{
    let limits = limits
        .clone()
        .loose()
        .width(width)
        .height(foot.height())
        .pad(padding);

    let mut foot = foot.layout(renderer, &limits);
    let size = limits.resolve(foot.size());

    foot.move_to(Point::new(padding, padding));
    foot.align(Align::Start, Align::Center, size);

    iced_native::layout::Node::with_children(size.pad(padding), vec![foot])
}

/// The renderer of a [`Card`](Card).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Cary`](Card) in your user interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// The default padding of a [`Card`](Card).
    const DEFAULT_PADDING: f32;

    /// The default text size of a [`Card`](Card).
    fn default_size(&self) -> f32;

    /// Draws a [`Card`](Card).
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        head: &Element<'_, Message, Self>,
        body: &Element<'_, Message, Self>,
        foot: &Option<Element<'_, Message, Self>>,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    const DEFAULT_PADDING: f32 = 0.0;

    fn default_size(&self) -> f32 {
        0.0
    }

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        _head: &Element<'_, Message, Self>,
        _body: &Element<'_, Message, Self>,
        _foot: &Option<Element<'_, Message, Self>>,
    ) -> Self::Output {
    }
}

impl<'a, Message, Renderer> From<Card<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(card: Card<'a, Message, Renderer>) -> Self {
        Element::new(card)
    }
}

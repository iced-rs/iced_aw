//! Displays a [`Card`].
//!
//! *This API requires the following crate features to be activated: card*

use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        text::LineHeight,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    alignment::{Horizontal, Vertical},
    event,
    mouse::{self, Cursor},
    touch,
    widget::text::Wrapping,
    Alignment, Border, Color, Element, Event, Length, Padding, Pixels, Point, Rectangle, Shadow,
    Size, Vector,
};
use iced_fonts::{
    required::{icon_to_string, RequiredIcons},
    REQUIRED_FONT,
};

pub use crate::style::{
    card::{Catalog, Style},
    status::{Status, StyleFn},
};

/// The default padding of a [`Card`].
const DEFAULT_PADDING: Padding = Padding::new(10.0);

/// A card consisting of a head, body and optional foot.
///
/// # Example
/// ```ignore
/// # use iced::widget::Text;
/// # use iced_aw::Card;
/// #
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
pub struct Card<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// The width of the [`Card`].
    width: Length,
    /// The height of the [`Card`].
    height: Length,
    /// The maximum width of the [`Card`].
    max_width: f32,
    /// The maximum height of the [`Card`].
    max_height: f32,
    /// The padding of the head of the [`Card`].
    padding_head: Padding,
    /// The padding of the body of the [`Card`].
    padding_body: Padding,
    /// The padding of the foot of the [`Card`].
    padding_foot: Padding,
    /// The optional size of the close icon of the [`Card`].
    close_size: Option<f32>,
    /// The optional message that is send if the close icon of the [`Card`] is pressed.
    on_close: Option<Message>,
    /// The head [`Element`] of the [`Card`].
    head: Element<'a, Message, Theme, Renderer>,
    /// The body [`Element`] of the [`Card`].
    body: Element<'a, Message, Theme, Renderer>,
    /// The optional foot [`Element`] of the [`Card`].
    foot: Option<Element<'a, Message, Theme, Renderer>>,
    /// The style of the [`Card`].
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Card<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// Creates a new [`Card`] containing the given head and body.
    ///
    /// It expects:
    ///     * the head [`Element`] to display at the top of the [`Card`].
    ///     * the body [`Element`] to display at the middle of the [`Card`].
    pub fn new<H, B>(head: H, body: B) -> Self
    where
        H: Into<Element<'a, Message, Theme, Renderer>>,
        B: Into<Element<'a, Message, Theme, Renderer>>,
    {
        Card {
            width: Length::Fill,
            height: Length::Shrink,
            max_width: u32::MAX as f32,
            max_height: u32::MAX as f32,
            padding_head: DEFAULT_PADDING,
            padding_body: DEFAULT_PADDING,
            padding_foot: DEFAULT_PADDING,
            close_size: None,
            on_close: None,
            head: head.into(),
            body: body.into(),
            foot: None,
            class: Theme::default(),
        }
    }

    /// Sets the [`Element`] of the foot of the [`Card`].
    #[must_use]
    pub fn foot<F>(mut self, foot: F) -> Self
    where
        F: Into<Element<'a, Message, Theme, Renderer>>,
    {
        self.foot = Some(foot.into());
        self
    }

    /// Sets the size of the close icon of the [`Card`].
    #[must_use]
    pub fn close_size(mut self, size: f32) -> Self {
        self.close_size = Some(size);
        self
    }

    /// Sets the height of the [`Card`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum height of the [`Card`].
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Sets the maximum width of the [`Card`].
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Sets the message that will be produced when the close icon of the
    /// [`Card`] is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the [`Card`].
    #[must_use]
    pub fn on_close(mut self, msg: Message) -> Self {
        self.on_close = Some(msg);
        self
    }

    /// Sets the padding of the [`Card`].
    ///
    /// This will set the padding of the head, body and foot to the
    /// same value.
    #[must_use]
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding_head = padding;
        self.padding_body = padding;
        self.padding_foot = padding;
        self
    }

    /// Sets the padding of the head of the [`Card`].
    #[must_use]
    pub fn padding_head(mut self, padding: Padding) -> Self {
        self.padding_head = padding;
        self
    }

    /// Sets the padding of the body of the [`Card`].
    #[must_use]
    pub fn padding_body(mut self, padding: Padding) -> Self {
        self.padding_body = padding;
        self
    }

    /// Sets the padding of the foot of the [`Card`].
    #[must_use]
    pub fn padding_foot(mut self, padding: Padding) -> Self {
        self.padding_foot = padding;
        self
    }

    /// Sets the style of the [`Card`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`Card`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the width of the [`Card`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Card<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog,
{
    fn children(&self) -> Vec<Tree> {
        self.foot.as_ref().map_or_else(
            || vec![Tree::new(&self.head), Tree::new(&self.body)],
            |foot| {
                vec![
                    Tree::new(&self.head),
                    Tree::new(&self.body),
                    Tree::new(foot),
                ]
            },
        )
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(foot) = self.foot.as_ref() {
            tree.diff_children(&[&self.head, &self.body, foot]);
        } else {
            tree.diff_children(&[&self.head, &self.body]);
        }
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.max_width(self.max_width).max_height(self.max_height);

        let head_node = head_node(
            renderer,
            &limits,
            &self.head,
            self.padding_head,
            self.width,
            self.on_close.is_some(),
            self.close_size,
            tree,
        );

        let limits = limits.shrink(Size::new(0.0, head_node.size().height));

        let mut foot_node = self.foot.as_ref().map_or_else(Node::default, |foot| {
            foot_node(renderer, &limits, foot, self.padding_foot, self.width, tree)
        });
        let limits = limits.shrink(Size::new(0.0, foot_node.size().height));
        let mut body_node = body_node(
            renderer,
            &limits,
            &self.body,
            self.padding_body,
            self.width,
            tree,
        );
        let body_bounds = body_node.bounds();
        body_node = body_node.move_to(Point::new(
            body_bounds.x,
            body_bounds.y + head_node.bounds().height,
        ));

        let foot_bounds = foot_node.bounds();

        foot_node = foot_node.move_to(Point::new(
            foot_bounds.x,
            foot_bounds.y + head_node.bounds().height + body_node.bounds().height,
        ));

        Node::with_children(
            Size::new(
                body_node.size().width,
                head_node.size().height + body_node.size().height + foot_node.size().height,
            ),
            vec![head_node, body_node, foot_node],
        )
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
        viewport: &Rectangle,
    ) -> event::Status {
        let mut children = layout.children();

        let head_layout = children
            .next()
            .expect("widget: Layout should have a head layout");
        let mut head_children = head_layout.children();
        let head_status = self.head.as_widget_mut().on_event(
            &mut state.children[0],
            event.clone(),
            head_children
                .next()
                .expect("widget: Layout should have a head content layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let close_status = head_children
            .next()
            .map_or(event::Status::Ignored, |close_layout| {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. }) => self
                        .on_close
                        .clone()
                        // TODO: `let` expressions in this position are experimental
                        // see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
                        .filter(|_| {
                            close_layout
                                .bounds()
                                .contains(cursor.position().unwrap_or_default())
                        })
                        .map_or(event::Status::Ignored, |on_close| {
                            shell.publish(on_close);
                            event::Status::Captured
                        }),
                    _ => event::Status::Ignored,
                }
            });

        let body_layout = children
            .next()
            .expect("widget: Layout should have a body layout");
        let mut body_children = body_layout.children();
        let body_status = self.body.as_widget_mut().on_event(
            &mut state.children[1],
            event.clone(),
            body_children
                .next()
                .expect("widget: Layout should have a body content layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let foot_layout = children
            .next()
            .expect("widget: Layout should have a foot layout");
        let mut foot_children = foot_layout.children();
        let foot_status = self.foot.as_mut().map_or(event::Status::Ignored, |foot| {
            foot.as_widget_mut().on_event(
                &mut state.children[2],
                event,
                foot_children
                    .next()
                    .expect("widget: Layout should have a foot content layout"),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        });

        head_status
            .merge(close_status)
            .merge(body_status)
            .merge(foot_status)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();

        let head_layout = children
            .next()
            .expect("widget: Layout should have a head layout");
        let mut head_children = head_layout.children();

        let head = head_children
            .next()
            .expect("widget: Layout should have a head layout");
        let close_layout = head_children.next();

        let is_mouse_over_close = close_layout.is_some_and(|layout| {
            let bounds = layout.bounds();
            bounds.contains(cursor.position().unwrap_or_default())
            });

        let mouse_interaction = if is_mouse_over_close {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        };

        let body_layout = children
            .next()
            .expect("widget: Layout should have a body layout");
        let mut body_children = body_layout.children();

        let foot_layout = children
            .next()
            .expect("widget: Layout should have a foot layout");
        let mut foot_children = foot_layout.children();

        mouse_interaction
            .max(self.head.as_widget().mouse_interaction(
                &state.children[0],
                head,
                cursor,
                viewport,
                renderer,
            ))
            .max(
                self.body.as_widget().mouse_interaction(
                    &state.children[1],
                    body_children
                        .next()
                        .expect("widget: Layout should have a body content layout"),
                    cursor,
                    viewport,
                    renderer,
                ),
            )
            .max(
                self.foot
                    .as_ref()
                    .map_or_else(mouse::Interaction::default, |foot| {
                        foot.as_widget().mouse_interaction(
                            &state.children[2],
                            foot_children
                                .next()
                                .expect("widget: Layout should have a foot content layout"),
                            cursor,
                            viewport,
                            renderer,
                        )
                    }),
            )
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let mut children = layout.children();
        let head_layout = children.next().expect("Missing Head Layout");
        let body_layout = children.next().expect("Missing Body Layout");
        let foot_layout = children.next().expect("Missing Footer Layout");

        self.head
            .as_widget()
            .operate(&mut state.children[0], head_layout, renderer, operation);
        self.body
            .as_widget()
            .operate(&mut state.children[1], body_layout, renderer, operation);

        if let Some(footer) = &self.foot {
            footer
                .as_widget()
                .operate(&mut state.children[2], foot_layout, renderer, operation);
        };
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let style_sheet = theme.style(&self.class, Status::Active);

        if bounds.intersects(viewport) {
            // Background
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet.border_radius.into(),
                        width: style_sheet.border_width,
                        color: style_sheet.border_color,
                    },
                    shadow: Shadow::default(),
                },
                style_sheet.background,
            );

            // Border
            renderer.fill_quad(
                // TODO: fill not necessary
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet.border_radius.into(),
                        width: style_sheet.border_width,
                        color: style_sheet.border_color,
                    },
                    shadow: Shadow::default(),
                },
                Color::TRANSPARENT,
            );
        }

        // ----------- Head ----------------------
        let head_layout = children
            .next()
            .expect("Graphics: Layout should have a head layout");
        draw_head(
            &state.children[0],
            renderer,
            &self.head,
            head_layout,
            cursor,
            viewport,
            theme,
            &style_sheet,
            self.close_size,
        );

        // ----------- Body ----------------------
        let body_layout = children
            .next()
            .expect("Graphics: Layout should have a body layout");
        draw_body(
            &state.children[1],
            renderer,
            &self.body,
            body_layout,
            cursor,
            viewport,
            theme,
            &style_sheet,
        );

        // ----------- Foot ----------------------
        let foot_layout = children
            .next()
            .expect("Graphics: Layout should have a foot layout");
        draw_foot(
            state.children.get(2),
            renderer,
            self.foot.as_ref(),
            foot_layout,
            cursor,
            viewport,
            theme,
            &style_sheet,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = vec![&mut self.head, &mut self.body];
        if let Some(foot) = &mut self.foot {
            children.push(foot);
        }
        let children = children
            .into_iter()
            .zip(&mut tree.children)
            .zip(layout.children())
            .filter_map(|((child, state), layout)| {
                layout.children().next().and_then(|child_layout| {
                    child
                        .as_widget_mut()
                        .overlay(state, child_layout, renderer, translation)
                })
            })
            .collect::<Vec<_>>();

        (!children.is_empty())
            .then(|| iced::advanced::overlay::Group::with_children(children).overlay())
    }
}

/// Calculates the layout of the head.
#[allow(clippy::too_many_arguments)]
fn head_node<Message, Theme, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    head: &Element<'_, Message, Theme, Renderer>,
    padding: Padding,
    width: Length,
    on_close: bool,
    close_size: Option<f32>,
    tree: &mut Tree,
) -> Node
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    let header_size = head.as_widget().size();

    let mut limits = limits
        .loose()
        .width(width)
        .height(header_size.height)
        .shrink(padding);

    let close_size = close_size.unwrap_or_else(|| renderer.default_size().0);

    if on_close {
        limits = limits.shrink(Size::new(close_size, 0.0));
    }

    let mut head = head
        .as_widget()
        .layout(&mut tree.children[0], renderer, &limits);
    let mut size = limits.resolve(width, header_size.height, head.size());

    head = head.move_to(Point::new(padding.left, padding.top));
    let head_size = head.size();
    head = head.align(Alignment::Start, Alignment::Center, head_size);

    let close = if on_close {
        let node = Node::new(Size::new(close_size + 1.0, close_size + 1.0));
        let node_size = node.size();

        size = Size::new(size.width + close_size, size.height);

        Some(
            node.move_to(Point::new(size.width - padding.right, padding.top))
                .align(Alignment::End, Alignment::Center, node_size),
        )
    } else {
        None
    };

    Node::with_children(
        size.expand(padding),
        match close {
            Some(node) => vec![head, node],
            None => vec![head],
        },
    )
}

/// Calculates the layout of the body.
fn body_node<Message, Theme, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    body: &Element<'_, Message, Theme, Renderer>,
    padding: Padding,
    width: Length,
    tree: &mut Tree,
) -> Node
where
    Renderer: renderer::Renderer,
{
    let body_size = body.as_widget().size();

    let limits = limits
        .loose()
        .width(width)
        .height(body_size.height)
        .shrink(padding);

    let mut body = body
        .as_widget()
        .layout(&mut tree.children[1], renderer, &limits);
    let size = limits.resolve(width, body_size.height, body.size());

    body = body.move_to(Point::new(padding.left, padding.top)).align(
        Alignment::Start,
        Alignment::Start,
        size,
    );

    Node::with_children(size.expand(padding), vec![body])
}

/// Calculates the layout of the foot.
fn foot_node<Message, Theme, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    foot: &Element<'_, Message, Theme, Renderer>,
    padding: Padding,
    width: Length,
    tree: &mut Tree,
) -> Node
where
    Renderer: renderer::Renderer,
{
    let foot_size = foot.as_widget().size();

    let limits = limits
        .loose()
        .width(width)
        .height(foot_size.height)
        .shrink(padding);

    let mut foot = foot
        .as_widget()
        .layout(&mut tree.children[2], renderer, &limits);
    let size = limits.resolve(width, foot_size.height, foot.size());

    foot = foot.move_to(Point::new(padding.left, padding.right)).align(
        Alignment::Start,
        Alignment::Center,
        size,
    );

    Node::with_children(size.expand(padding), vec![foot])
}

/// Draws the head of the card.
#[allow(clippy::too_many_arguments)]
fn draw_head<Message, Theme, Renderer>(
    state: &Tree,
    renderer: &mut Renderer,
    head: &Element<'_, Message, Theme, Renderer>,
    layout: Layout<'_>,
    cursor: Cursor,
    viewport: &Rectangle,
    theme: &Theme,
    style: &Style,
    close_size: Option<f32>,
) where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog,
{
    let mut head_children = layout.children();
    let bounds = layout.bounds();
    let border_radius = style.border_radius;

    // Head background
    if bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: border_radius.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            },
            style.head_background,
        );
    }

    // cover rounded button of header
    let button_bounds = Rectangle {
        x: bounds.x,
        y: bounds.y + bounds.height - border_radius,
        width: bounds.width,
        height: border_radius,
    };
    if button_bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: button_bounds,
                border: Border {
                    radius: (0.0).into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            },
            style.head_background,
        );
    }

    head.as_widget().draw(
        state,
        renderer,
        theme,
        &renderer::Style {
            text_color: style.head_text_color,
        },
        head_children
            .next()
            .expect("Graphics: Layout should have a head content layout"),
        cursor,
        viewport,
    );

    if let Some(close_layout) = head_children.next() {
        let close_bounds = close_layout.bounds();
        let is_mouse_over_close = close_bounds.contains(cursor.position().unwrap_or_default());

        renderer.fill_text(
            iced::advanced::text::Text {
                content: icon_to_string(RequiredIcons::X),
                bounds: Size::new(close_bounds.width, close_bounds.height),
                size: Pixels(
                    close_size.unwrap_or_else(|| renderer.default_size().0)
                        + if is_mouse_over_close { 1.0 } else { 0.0 },
                ),
                font: REQUIRED_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(close_bounds.center_x(), close_bounds.center_y()),
            style.close_color,
            close_bounds,
        );
    }
}

/// Draws the body of the card.
#[allow(clippy::too_many_arguments)]
fn draw_body<Message, Theme, Renderer>(
    state: &Tree,
    renderer: &mut Renderer,
    body: &Element<'_, Message, Theme, Renderer>,
    layout: Layout<'_>,
    cursor: Cursor,
    viewport: &Rectangle,
    theme: &Theme,
    style: &Style,
) where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog,
{
    let mut body_children = layout.children();
    let bounds = layout.bounds();

    // Body background
    if bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: (0.0).into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            },
            style.body_background,
        );
    }

    body.as_widget().draw(
        state,
        renderer,
        theme,
        &renderer::Style {
            text_color: style.body_text_color,
        },
        body_children
            .next()
            .expect("Graphics: Layout should have a body content layout"),
        cursor,
        viewport,
    );
}

/// Draws the foot of the card.
#[allow(clippy::too_many_arguments)]
fn draw_foot<Message, Theme, Renderer>(
    state: Option<&Tree>,
    renderer: &mut Renderer,
    foot: Option<&Element<'_, Message, Theme, Renderer>>,
    layout: Layout<'_>,
    cursor: Cursor,
    viewport: &Rectangle,
    theme: &Theme,
    style: &Style,
) where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog,
{
    let mut foot_children = layout.children();
    let bounds = layout.bounds();

    // Foot background
    if bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: style.border_radius.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            },
            style.foot_background,
        );
    }

    if let Some((foot, state)) = foot.as_ref().zip(state) {
        foot.as_widget().draw(
            state,
            renderer,
            theme,
            &renderer::Style {
                text_color: style.foot_text_color,
            },
            foot_children
                .next()
                .expect("Graphics: Layout should have a foot content layout"),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Card<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a + Catalog,
    Message: Clone + 'a,
{
    fn from(card: Card<'a, Message, Theme, Renderer>) -> Self {
        Element::new(card)
    }
}

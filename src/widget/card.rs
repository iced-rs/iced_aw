//! Displays a [`Card`].
//!
//! *This API requires the following crate features to be activated: card*

use crate::iced_aw_font::advanced_text::cancel;
pub use crate::style::{
    card::{Catalog, Style},
    status::{Status, StyleFn},
};
use iced_core::{
    Alignment, Border, Clipboard, Color, Element, Event, Layout, Length, Padding, Point, Rectangle,
    Shadow, Shell, Size, Vector, Widget,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree},
};
use iced_widget::button;

/// The default padding of a [`Card`].
const DEFAULT_PADDING: Padding = Padding::new(10.0);

/// The default size of the close button icon
const DEFAULT_CLOSE_SIZE: f32 = 16.0;

/// Extra space around the close button for click area
const CLOSE_BUTTON_SPACING: f32 = 1.0;

/// A card consisting of a head, body and optional foot.
///
/// # Example
/// ```ignore
/// # use iced_widget::Text;
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
pub struct Card<'a, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer>
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
    /// The optional close button [`Element`] of the [`Card`].
    close_button: Option<Element<'a, Message, Theme, Renderer>>,
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
            close_button: None,
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
    pub fn on_close(mut self, msg: Message) -> Self
    where
        Message: Clone + 'a,
        Renderer: iced_core::text::Renderer<Font = iced_core::Font> + 'a,
        Theme: iced_widget::text::Catalog + iced_widget::button::Catalog + 'a,
        <Theme as iced_widget::button::Catalog>::Class<'a>:
            From<iced_widget::button::StyleFn<'a, Theme>>,
    {
        self.on_close = Some(msg.clone());
        self.close_button = Some(self.create_close_button(msg));
        self
    }

    /// Creates a close button element.
    fn create_close_button(&self, msg: Message) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Renderer: iced_core::text::Renderer<Font = iced_core::Font> + 'a,
        Theme: iced_widget::text::Catalog + iced_widget::button::Catalog + 'a,
        <Theme as iced_widget::button::Catalog>::Class<'a>:
            From<iced_widget::button::StyleFn<'a, Theme>>,
    {
        let (content, font, shaping) = cancel();
        let size = self.close_size.unwrap_or(DEFAULT_CLOSE_SIZE);

        let text_widget = iced_widget::text(content)
            .font(font)
            .size(size)
            .shaping(shaping);

        button(text_widget)
            .padding(0)
            .style(|theme: &Theme, _status| {
                let card_style = <Theme as Catalog>::style(
                    theme,
                    &<Theme as Catalog>::default(),
                    crate::style::status::Status::Active,
                );
                iced_widget::button::Style {
                    background: None,
                    text_color: card_style.close_color,
                    border: iced_core::Border::default(),
                    shadow: iced_core::Shadow::default(),
                    snap: false,
                }
            })
            .on_press(msg)
            .into()
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
    Renderer: 'a + renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: Catalog,
{
    fn children(&self) -> Vec<Tree> {
        let mut children = vec![Tree::new(&self.head), Tree::new(&self.body)];

        if let Some(foot) = &self.foot {
            children.push(Tree::new(foot));
        }

        if let Some(close_button) = &self.close_button {
            children.push(Tree::new(close_button));
        }

        children
    }

    fn diff(&self, tree: &mut Tree) {
        match (&self.foot, &self.close_button) {
            (Some(foot), Some(close_button)) => {
                tree.diff_children(&[&self.head, &self.body, foot, close_button]);
            }
            (Some(foot), None) => {
                tree.diff_children(&[&self.head, &self.body, foot]);
            }
            (None, Some(close_button)) => {
                tree.diff_children(&[&self.head, &self.body, close_button]);
            }
            (None, None) => {
                tree.diff_children(&[&self.head, &self.body]);
            }
        }
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.max_width(self.max_width).max_height(self.max_height);

        let close_button_tree_index = 2 + if self.foot.is_some() { 1 } else { 0 };

        let head_node = head_node(
            renderer,
            &limits,
            &mut self.head,
            self.padding_head,
            self.width,
            self.close_button.as_mut(),
            self.close_size,
            tree,
            close_button_tree_index,
        );

        let limits = limits.shrink(Size::new(0.0, head_node.size().height));

        let mut foot_node = self.foot.as_mut().map_or_else(Node::default, |foot| {
            foot_node(renderer, &limits, foot, self.padding_foot, self.width, tree)
        });
        let limits = limits.shrink(Size::new(0.0, foot_node.size().height));
        let mut body_node = body_node(
            renderer,
            &limits,
            &mut self.body,
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

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let head_layout = children
            .next()
            .expect("widget: Layout should have a head layout");
        let mut head_children = head_layout.children();

        self.head.as_widget_mut().update(
            &mut state.children[0],
            event,
            head_children
                .next()
                .expect("widget: Layout should have a head content layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        // Update close button if present
        if let Some(close_layout) = head_children.next() {
            if let Some(close_button) = self.close_button.as_mut() {
                let close_button_tree_index = 2 + if self.foot.is_some() { 1 } else { 0 };
                close_button.as_widget_mut().update(
                    &mut state.children[close_button_tree_index],
                    event,
                    close_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );
            }
        }

        let body_layout = children
            .next()
            .expect("widget: Layout should have a body layout");

        self.body.as_widget_mut().update(
            &mut state.children[1],
            event,
            body_layout
                .children()
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

        if let Some(foot) = self.foot.as_mut() {
            foot.as_widget_mut().update(
                &mut state.children[2],
                event,
                foot_layout
                    .children()
                    .next()
                    .expect("widget: Layout should have a foot content layout"),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
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
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let mut children = layout.children();
        let head_layout = children.next().expect("Missing Head Layout");
        let body_layout = children.next().expect("Missing Body Layout");
        let foot_layout = children.next().expect("Missing Footer Layout");

        // Operate on head and close button
        let mut head_children = head_layout.children();
        if let Some(head_content_layout) = head_children.next() {
            self.head.as_widget_mut().operate(
                &mut state.children[0],
                head_content_layout,
                renderer,
                operation,
            );
        }

        // Operate on close button if present (second child of head_layout)
        if let Some(close_layout) = head_children.next() {
            if let Some(close_button) = self.close_button.as_mut() {
                let close_button_tree_index = 2 + if self.foot.is_some() { 1 } else { 0 };
                close_button.as_widget_mut().operate(
                    &mut state.children[close_button_tree_index],
                    close_layout,
                    renderer,
                    operation,
                );
            }
        }

        // Operate on body (body_layout contains a child with the actual body content)
        if let Some(body_content_layout) = body_layout.children().next() {
            self.body.as_widget_mut().operate(
                &mut state.children[1],
                body_content_layout,
                renderer,
                operation,
            );
        }

        // Operate on footer if present (foot_layout contains a child with the actual foot content)
        if let Some(footer) = &mut self.foot {
            if let Some(foot_content_layout) = foot_layout.children().next() {
                footer.as_widget_mut().operate(
                    &mut state.children[2],
                    foot_content_layout,
                    renderer,
                    operation,
                );
            }
        }
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
                    snap: false,
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
                    snap: false,
                },
                Color::TRANSPARENT,
            );
        }

        // ----------- Head ----------------------
        let head_layout = children
            .next()
            .expect("Graphics: Layout should have a head layout");
        let close_button_tree_index = 2 + if self.foot.is_some() { 1 } else { 0 };
        draw_head(
            &state.children[0],
            renderer,
            &self.head,
            head_layout,
            cursor,
            viewport,
            theme,
            &style_sheet,
            self.close_button.as_ref(),
            state.children.get(close_button_tree_index),
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
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
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
                    child.as_widget_mut().overlay(
                        state,
                        child_layout,
                        renderer,
                        viewport,
                        translation,
                    )
                })
            })
            .collect::<Vec<_>>();

        if children.is_empty() {
            None
        } else {
            Some(overlay::Group::with_children(children).overlay())
        }
    }
}

/// Calculates the layout of the head.
#[allow(clippy::too_many_arguments)]
fn head_node<Message, Theme, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    head: &mut Element<'_, Message, Theme, Renderer>,
    padding: Padding,
    width: Length,
    close_button: Option<&mut Element<'_, Message, Theme, Renderer>>,
    close_size: Option<f32>,
    tree: &mut Tree,
    close_button_tree_index: usize,
) -> Node
where
    Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
{
    let header_size = head.as_widget().size();

    let mut limits = limits
        .loose()
        .width(width)
        .height(header_size.height)
        .shrink(padding);

    let close_size = close_size.unwrap_or_else(|| renderer.default_size().0);

    if close_button.is_some() {
        limits = limits.shrink(Size::new(close_size, 0.0));
    }

    let mut head = head
        .as_widget_mut()
        .layout(&mut tree.children[0], renderer, &limits);
    let mut size = limits.resolve(width, header_size.height, head.size());

    head = head.move_to(Point::new(padding.left, padding.top));
    let head_size = head.size();
    head = head.align(Alignment::Start, Alignment::Center, head_size);

    let close = if let Some(close_btn) = close_button {
        let button_size = close_size + CLOSE_BUTTON_SPACING;
        let button_limits = limits
            .loose()
            .width(Length::Fixed(button_size))
            .height(Length::Fixed(button_size));
        let mut close_node = close_btn.as_widget_mut().layout(
            &mut tree.children[close_button_tree_index],
            renderer,
            &button_limits,
        );
        let node_size = close_node.size();

        size = Size::new(size.width + close_size, size.height);

        close_node = close_node
            .move_to(Point::new(size.width - padding.right, padding.top))
            .align(Alignment::End, Alignment::Center, node_size);

        Some(close_node)
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
    body: &mut Element<'_, Message, Theme, Renderer>,
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
        .as_widget_mut()
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
    foot: &mut Element<'_, Message, Theme, Renderer>,
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
        .as_widget_mut()
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
    close_button: Option<&Element<'_, Message, Theme, Renderer>>,
    close_button_state: Option<&Tree>,
) where
    Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
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
                snap: false,
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
                snap: false,
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

    // Draw close button if present
    if let Some(close_layout) = head_children.next() {
        if let Some((close_btn, close_state)) = close_button.zip(close_button_state) {
            close_btn.as_widget().draw(
                close_state,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style.close_color,
                },
                close_layout,
                cursor,
                viewport,
            );
        }
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
    Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
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
                snap: false,
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
    Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
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
                snap: false,
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
    Renderer: 'a + renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: 'a + Catalog,
    Message: Clone + 'a,
{
    fn from(card: Card<'a, Message, Theme, Renderer>) -> Self {
        Element::new(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum TestMessage {
        Close,
    }

    type TestCard<'a> = Card<'a, TestMessage, iced_widget::Theme, iced_widget::Renderer>;

    #[test]
    fn card_new_has_default_values() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        );

        assert_eq!(card.width, Length::Fill);
        assert_eq!(card.height, Length::Shrink);
        assert_eq!(card.max_width, u32::MAX as f32);
        assert_eq!(card.max_height, u32::MAX as f32);
        assert_eq!(card.padding_head, DEFAULT_PADDING);
        assert_eq!(card.padding_body, DEFAULT_PADDING);
        assert_eq!(card.padding_foot, DEFAULT_PADDING);
        assert!(card.close_size.is_none());
        assert!(card.on_close.is_none());
        assert!(card.foot.is_none());
        assert!(card.close_button.is_none());
    }

    #[test]
    fn card_width_sets_value() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .width(400);

        assert_eq!(card.width, Length::Fixed(400.0));
    }

    #[test]
    fn card_height_sets_value() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .height(300);

        assert_eq!(card.height, Length::Fixed(300.0));
    }

    #[test]
    fn card_max_width_sets_value() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .max_width(500.0);

        assert_eq!(card.max_width, 500.0);
    }

    #[test]
    fn card_max_height_sets_value() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .max_height(400.0);

        assert_eq!(card.max_height, 400.0);
    }

    #[test]
    fn card_padding_sets_all_sections() {
        let padding = Padding::new(15.0);
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .padding(padding);

        assert_eq!(card.padding_head, padding);
        assert_eq!(card.padding_body, padding);
        assert_eq!(card.padding_foot, padding);
    }

    #[test]
    fn card_padding_head_sets_value() {
        let padding = Padding::new(20.0);
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .padding_head(padding);

        assert_eq!(card.padding_head, padding);
        assert_eq!(card.padding_body, DEFAULT_PADDING);
        assert_eq!(card.padding_foot, DEFAULT_PADDING);
    }

    #[test]
    fn card_padding_body_sets_value() {
        let padding = Padding::new(25.0);
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .padding_body(padding);

        assert_eq!(card.padding_head, DEFAULT_PADDING);
        assert_eq!(card.padding_body, padding);
        assert_eq!(card.padding_foot, DEFAULT_PADDING);
    }

    #[test]
    fn card_padding_foot_sets_value() {
        let padding = Padding::new(10.0);
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .padding_foot(padding);

        assert_eq!(card.padding_head, DEFAULT_PADDING);
        assert_eq!(card.padding_body, DEFAULT_PADDING);
        assert_eq!(card.padding_foot, padding);
    }

    #[test]
    fn card_foot_adds_footer() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .foot(iced_widget::text::Text::new("Footer"));

        assert!(card.foot.is_some());
    }

    #[test]
    fn card_on_close_sets_message() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .on_close(TestMessage::Close);

        assert!(card.on_close.is_some());
    }

    #[test]
    fn card_close_size_sets_value() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .close_size(24.0);

        assert_eq!(card.close_size, Some(24.0));
    }

    #[test]
    fn card_size_method_returns_correct_size() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .width(500)
        .height(400);

        let size = Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&card);
        assert_eq!(size.width, Length::Fixed(500.0));
        assert_eq!(size.height, Length::Fixed(400.0));
    }

    #[test]
    fn card_chaining_methods() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .foot(iced_widget::text::Text::new("Footer"))
        .width(600)
        .height(450)
        .max_width(800.0)
        .max_height(600.0)
        .padding(Padding::new(15.0))
        .on_close(TestMessage::Close)
        .close_size(20.0);

        assert!(card.foot.is_some());
        assert_eq!(card.width, Length::Fixed(600.0));
        assert_eq!(card.height, Length::Fixed(450.0));
        assert_eq!(card.max_width, 800.0);
        assert_eq!(card.max_height, 600.0);
        assert_eq!(card.padding_head, Padding::new(15.0));
        assert_eq!(card.padding_body, Padding::new(15.0));
        assert_eq!(card.padding_foot, Padding::new(15.0));
        assert!(card.on_close.is_some());
        assert_eq!(card.close_size, Some(20.0));
    }

    #[test]
    fn card_default_padding_value() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        );

        assert_eq!(card.padding_head, Padding::new(10.0));
        assert_eq!(card.padding_body, Padding::new(10.0));
        assert_eq!(card.padding_foot, Padding::new(10.0));
    }

    #[test]
    fn card_individual_padding_overrides() {
        let general_padding = Padding::new(10.0);
        let head_padding = Padding::new(20.0);

        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .padding(general_padding)
        .padding_head(head_padding);

        assert_eq!(card.padding_head, head_padding);
        assert_eq!(card.padding_body, general_padding);
        assert_eq!(card.padding_foot, general_padding);
    }

    #[test]
    fn card_without_footer_has_none() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        );

        assert!(card.foot.is_none());
    }

    #[test]
    fn card_without_on_close_has_none() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        );

        assert!(card.on_close.is_none());
    }

    #[test]
    fn card_close_size_without_on_close() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .close_size(24.0);

        assert_eq!(card.close_size, Some(24.0));
        assert!(card.on_close.is_none());
    }

    #[test]
    fn card_default_max_dimensions() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        );

        assert_eq!(card.max_width, u32::MAX as f32);
        assert_eq!(card.max_height, u32::MAX as f32);
    }

    #[test]
    fn card_length_fill() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        assert_eq!(card.width, Length::Fill);
        assert_eq!(card.height, Length::Fill);
    }

    #[test]
    fn card_length_shrink() {
        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .width(Length::Shrink)
        .height(Length::Shrink);

        assert_eq!(card.width, Length::Shrink);
        assert_eq!(card.height, Length::Shrink);
    }

    #[test]
    fn card_asymmetric_padding() {
        let head_padding = Padding {
            top: 5.0,
            right: 10.0,
            bottom: 15.0,
            left: 20.0,
        };
        let body_padding = Padding {
            top: 10.0,
            right: 20.0,
            bottom: 30.0,
            left: 40.0,
        };
        let foot_padding = Padding {
            top: 2.0,
            right: 4.0,
            bottom: 6.0,
            left: 8.0,
        };

        let card = TestCard::new(
            iced_widget::text::Text::new("Head"),
            iced_widget::text::Text::new("Body"),
        )
        .padding_head(head_padding)
        .padding_body(body_padding)
        .padding_foot(foot_padding);

        assert_eq!(card.padding_head, head_padding);
        assert_eq!(card.padding_body, body_padding);
        assert_eq!(card.padding_foot, foot_padding);
    }
}

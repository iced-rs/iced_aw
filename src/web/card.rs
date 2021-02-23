//! Displays a [`Card`](Card).
//!
//! *This API requires the following crate features to be activated: card*
use dodrio::bumpalo;
use iced_web::{css, Background, Bus, Color, Css, Element, Length, Widget};

pub use crate::style::card::{Style, StyleSheet};

const DEFAULT_PADDING: f32 = 10.0;

/// A card consisting of a head, body and optional foot.
///
/// # Example
/// ```
/// # use iced_aw::Card;
/// # use iced_web::Text;
/// #[derive(Clone, Debug)]
/// enum Message {
///    ClosingCard,
/// }
///
/// let card = Card::new(
///     Text::new("Head"),
///     Text::new("Body")
/// )
/// .foot(Text::new("Foot"))
/// .on_close(Message::ClosingCard);
/// ```
#[allow(missing_debug_implementations)]
pub struct Card<'a, Message> {
    width: Length,
    height: Length,
    max_width: u16,
    max_height: u16,
    padding_head: f32,
    padding_body: f32,
    padding_foot: f32,
    close_size: Option<f32>,
    on_close: Option<Message>,
    head: Element<'a, Message>,
    body: Element<'a, Message>,
    foot: Option<Element<'a, Message>>,
    style: Box<dyn StyleSheet>,
}

impl<'a, Message> Card<'a, Message> {
    /// Creates a new [`Card`](Card) containing the given head and body.
    pub fn new<H, B>(head: H, body: B) -> Self
    where
        H: Into<Element<'a, Message>>,
        B: Into<Element<'a, Message>>,
    {
        Card {
            width: Length::Fill,
            height: Length::Shrink,
            max_width: u16::MAX,
            max_height: u16::MAX,
            padding_head: DEFAULT_PADDING,
            padding_body: DEFAULT_PADDING,
            padding_foot: DEFAULT_PADDING,
            close_size: None,
            on_close: None,
            head: head.into(),
            body: body.into(),
            foot: None,
            style: Default::default(),
        }
    }

    /// Sets the [`Element`](iced_native::Element) of the foot of the
    /// [`Card`](Card).
    pub fn foot<F>(mut self, foot: F) -> Self
    where
        F: Into<Element<'a, Message>>,
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
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Sets the maximum height of the [`Card`](Card).
    pub fn max_height(mut self, height: u16) -> Self {
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
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Card<'a, Message>
where
    Message: 'static + Clone,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let style = self.style.active();

        let head_node = head_node(
            &self.head,
            self.padding_head,
            self.width,
            &self.on_close,
            self.close_size,
            &style,
            bump,
            bus,
            style_sheet,
        );

        let body_node = body_node(
            &self.body,
            self.padding_body,
            self.width,
            &style,
            bump,
            bus,
            style_sheet,
        );

        let foot_node = self.foot.as_ref().map(|foot| {
            foot_node(
                foot,
                self.padding_foot,
                self.width,
                &style,
                bump,
                bus,
                style_sheet,
            )
        });

        let border = div(bump)
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "position: absolute; top: 0; bottom: 0; left: 0; right: 0; \
                    border-radius: {}px; border: {}px solid {}; pointer-events:none;",
                    style.border_radius,
                    style.border_width,
                    css::color(style.border_color)
                )
                .into_bump_str(),
            )
            .finish();

        let node = div(bump)
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "background: {}; border-radius: {}px; width: {}; height: {}; \
                    position: relative; max-width: {}px; max-height: {}px;",
                    match style.background {
                        Background::Color(color) => css::color(color),
                    },
                    style.border_radius,
                    css::length(self.width),
                    css::length(self.height),
                    self.max_width,
                    self.max_height
                )
                .into_bump_str(),
            )
            .children(match foot_node {
                Some(foot_node) => vec![head_node, body_node, foot_node, border],
                None => vec![head_node, body_node, border],
            });

        node.finish()
    }
}

fn head_node<'a, 'b, Message: 'static + Clone>(
    my_head: &Element<'a, Message>,
    padding: f32,
    _width: Length,
    on_close: &Option<Message>,
    close_size: Option<f32>,
    my_style: &Style,
    bump: &'b bumpalo::Bump,
    bus: &Bus<Message>,
    style_sheet: &mut Css<'b>,
) -> dodrio::Node<'b> {
    use dodrio::builder::*;

    let head_padding_class = style_sheet.insert(bump, css::Rule::Padding(padding as u16)); // TODO: will change in the future

    let head_class = {
        use dodrio::bumpalo::collections::String;

        String::from_str_in(&head_padding_class, bump).into_bump_str()
    };

    let head_content = div(bump)
        .attr("style", "width: 100%")
        .children(vec![my_head.node(bump, bus, style_sheet)])
        .finish();

    let close = on_close.clone().map(|on_close| {
        let event_bus = bus.clone();
        div(bump)
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "font-size: {}px; width: auto; height: auto; \
                    text-align: center; color: {}; cursor: pointer;",
                    close_size.unwrap_or(20.0),
                    css::color(my_style.close_color)
                )
                .into_bump_str(),
            )
            .children(vec![text(
                close_icon(my_style.close_color, bump).into_bump_str(),
            )])
            .on("click", move |_root, _vdom, _event| {
                event_bus.publish(on_close.clone());
            })
            .finish()
    });

    let my_head = div(bump)
        .attr("class", head_class)
        .attr(
            "style",
            bumpalo::format!(
                in bump,
                "background: {}; border-radius: {}px; width: 100%; \
                color: {}; display: flex;",
                match my_style.head_background {
                    Background::Color(color) => css::color(color),
                },
                my_style.border_radius,
                css::color(my_style.head_text_color)
            )
            .into_bump_str(),
        )
        .children(match close {
            Some(close) => vec![head_content, close],
            None => vec![head_content],
        })
        .finish();

    my_head
}

fn body_node<'a, 'b, Message>(
    my_body: &Element<'a, Message>,
    padding: f32,
    _width: Length,
    my_style: &Style,
    bump: &'b bumpalo::Bump,
    bus: &Bus<Message>,
    style_sheet: &mut Css<'b>,
) -> dodrio::Node<'b> {
    use dodrio::builder::*;

    let body_padding_class = style_sheet.insert(bump, css::Rule::Padding(padding as u16)); // TODO: same

    let body_class = {
        use dodrio::bumpalo::collections::String;

        String::from_str_in(&body_padding_class, bump).into_bump_str()
    };

    let my_body = div(bump)
        .attr("class", body_class)
        .attr(
            "style",
            bumpalo::format!(
                in bump,
                "background: {}; width: 100%; color: {};",
                match my_style.body_background {
                    Background::Color(color) => css::color(color),
                },
                css::color(my_style.body_text_color)
            )
            .into_bump_str(),
        )
        .children(vec![my_body.node(bump, bus, style_sheet)])
        .finish();

    my_body
}

fn foot_node<'a, 'b, Message>(
    my_foot: &Element<'a, Message>,
    padding: f32,
    _width: Length,
    my_style: &Style,
    bump: &'b bumpalo::Bump,
    bus: &Bus<Message>,
    style_sheet: &mut Css<'b>,
) -> dodrio::Node<'b> {
    use dodrio::builder::*;

    let foot_padding_class = style_sheet.insert(bump, css::Rule::Padding(padding as u16)); // TODO: same

    let foot_class = {
        use dodrio::bumpalo::collections::String;

        String::from_str_in(&foot_padding_class, bump).into_bump_str()
    };

    let foot_node = div(bump)
        .attr("class", foot_class)
        .attr(
            "style",
            bumpalo::format!(
                in bump,
                "background: {}; border-radius: {}px; width: 100% \
                color: {}",
                match my_style.foot_background {
                    Background::Color(color) => css::color(color),
                },
                my_style.border_radius,
                css::color(my_style.foot_text_color)
            )
            .into_bump_str(),
        )
        .children(vec![my_foot.node(bump, bus, style_sheet)])
        .finish();

    foot_node
}

impl<'a, Message> From<Card<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(card: Card<'a, Message>) -> Element<'a, Message> {
        Element::new(card)
    }
}

// TODO: move this into own module
/*fn close_icon<'b>(color: Color, bump: &'b bumpalo::Bump) -> dodrio::bumpalo::collections::String<'b> {
    use dodrio::bumpalo::collections::String;

    String::from_str_in(
        &format!(
            r#"<svg width="1em" height="1em" viewBox="0 0 16 16" class="bi bi-x" fill="{}" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
            </svg>"#,
            css::color(color)
        ),
        bump,
    )
}*/

fn close_icon<'b>(
    _color: Color,
    bump: &'b bumpalo::Bump,
) -> dodrio::bumpalo::collections::String<'b> {
    use dodrio::bumpalo::collections::String;

    String::from_str_in(r#"×"#, bump)
}

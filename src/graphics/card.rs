//! Displays a [`Card`](Card).
//!
//! *This API requires the following crate features to be activated: card*
use iced_graphics::Renderer;

use crate::native::card;
pub use crate::style::card::{Style, StyleSheet};

/// A card consisting of a head, body and optional foot.
///
/// This is an alias of an `iced_native` Card with an `iced_wgpu::Renderer`.
pub type Card<'a, Message, Backend> = card::Card<'a, Message, Renderer<Backend>>;

/*
impl<B> card::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_PADDING: f32 = 10.0;

    fn default_size(&self) -> f32 {
        f32::from(self.backend().default_size())
    }

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        head: &iced_native::Element<'_, Message, Self>,
        body: &iced_native::Element<'_, Message, Self>,
        foot: &Option<iced_native::Element<'_, Message, Self>>,
    ) -> Self::Output {
        let bounds = env.layout.bounds();
        let mut children = env.layout.children();
        let style = env.style_sheet.active();

        let mouse_interaction = mouse::Interaction::default();

        let background = Primitive::Quad {
            bounds,
            background: style.background,
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        };

        let border = Primitive::Quad {
            bounds,
            background: Color::TRANSPARENT.into(),
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        };

        // ----------- Head ----------------------
        let head_layout = children
            .next()
            .expect("Graphics: Layout should have a head layout");
        let (head, head_mouse_interaction) = draw_head(
            self,
            head,
            head_layout,
            env.cursor_position,
            env.viewport.expect("A viewport should exist for Card"),
            &style,
        );

        // ----------- Body ----------------------
        let body_layout = children
            .next()
            .expect("Graphics: Layout should have a body layout");
        let (body, body_mouse_interaction) = draw_body(
            self,
            body,
            body_layout,
            env.cursor_position,
            env.viewport.expect("A viewport should exist for Card"),
            &style,
        );

        // ----------- Foot ----------------------
        let foot_layout = children
            .next()
            .expect("Graphics: Layout should have a foot layout");
        let (foot, foot_mouse_interaction) = draw_foot(
            self,
            foot,
            foot_layout,
            env.cursor_position,
            env.viewport.expect("A viewport should exist for Card"),
            &style,
        );

        (
            Primitive::Group {
                primitives: vec![background, border, head, body, foot],
            },
            mouse_interaction
                .max(head_mouse_interaction)
                .max(body_mouse_interaction)
                .max(foot_mouse_interaction),
        )
    }
}

/// Draws the head of the card.
fn draw_head<Message, B>(
    renderer: &mut Renderer<B>,
    head: &Element<'_, Message, Renderer<B>>,
    layout: Layout<'_>,
    cursor_position: Point,
    viewport: &Rectangle,
    style: &Style,
) -> (Primitive, mouse::Interaction)
where
    B: Backend + backend::Text,
{
    let mut head_children = layout.children();
    let head_background = Primitive::Quad {
        bounds: layout.bounds(),
        background: style.head_background,
        border_radius: style.border_radius,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    };

    let (head, head_mouse_interaction) = head.draw(
        renderer,
        &Defaults {
            text: defaults::Text {
                color: style.head_text_color,
            },
        },
        head_children
            .next()
            .expect("Graphics: Layout should have a head content layout"),
        cursor_position,
        viewport,
    );

    let (close, close_mouse_interaction) = head_children.next().map_or(
        (Primitive::None, mouse::Interaction::default()),
        |close_layout| {
            let close_bounds = close_layout.bounds();
            let is_mouse_over_close = close_bounds.contains(cursor_position);

            (
                Primitive::Text {
                    content: super::icons::Icon::X.into(),
                    font: super::icons::ICON_FONT,
                    size: close_layout.bounds().height
                        + if is_mouse_over_close { 5.0 } else { 0.0 },
                    bounds: Rectangle {
                        x: close_bounds.center_x(),
                        y: close_bounds.center_y(),
                        ..close_bounds
                    },
                    color: style.close_color,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                },
                if is_mouse_over_close {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                },
            )
        },
    );

    (
        Primitive::Group {
            primitives: vec![head_background, head, close],
        },
        head_mouse_interaction.max(close_mouse_interaction),
    )
}

/// Draws the body of the card.
fn draw_body<Message, B>(
    renderer: &mut Renderer<B>,
    body: &Element<'_, Message, Renderer<B>>,
    layout: Layout<'_>,
    cursor_position: Point,
    viewport: &Rectangle,
    style: &Style,
) -> (Primitive, mouse::Interaction)
where
    B: Backend + backend::Text,
{
    let mut body_children = layout.children();
    let body_background = Primitive::Quad {
        bounds: layout.bounds(),
        background: style.body_background,
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    };

    let (body, mouse_interaction) = body.draw(
        renderer,
        &Defaults {
            text: defaults::Text {
                color: style.body_text_color,
            },
        },
        body_children
            .next()
            .expect("Graphics: Layout should have a body content layout"),
        cursor_position,
        viewport,
    );

    (
        Primitive::Group {
            primitives: vec![body_background, body],
        },
        mouse_interaction,
    )
}

/// Draws the foot of the card.
fn draw_foot<Message, B>(
    renderer: &mut Renderer<B>,
    foot: &Option<Element<'_, Message, Renderer<B>>>,
    layout: Layout<'_>,
    cursor_position: Point,
    viewport: &Rectangle,
    style: &Style,
) -> (Primitive, mouse::Interaction)
where
    B: Backend + backend::Text,
{
    let mut foot_children = layout.children();
    let foot_background = Primitive::Quad {
        bounds: layout.bounds(),
        background: style.foot_background,
        border_radius: style.border_radius,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    };

    let (foot, foot_mouse_interaction) = foot.as_ref().map_or_else(
        || (Primitive::None, mouse::Interaction::default()),
        |foot| {
            foot.draw(
                renderer,
                &Defaults {
                    text: defaults::Text {
                        color: style.foot_text_color,
                    },
                },
                foot_children
                    .next()
                    .expect("Graphics: Layout should have a foot content layout"),
                cursor_position,
                viewport,
            )
        },
    );

    (
        Primitive::Group {
            primitives: vec![foot_background, foot],
        },
        foot_mouse_interaction,
    )
}
*/

//! Displays a [`Card`](Card).
//!
//! *This API requires the following crate features to be activated: card*
use iced_graphics::{
    backend, defaults, Backend, Color, Defaults, HorizontalAlignment, Primitive, Rectangle,
    Renderer, VerticalAlignment,
};
use iced_native::mouse;

pub use crate::style::card::{Style, StyleSheet};
use crate::{core::renderer::DrawEnvironment, native::card};

/// A card consisting of a head, body and optional foot.
///
/// This is an alias of an `iced_native` Card with an `iced_wgpu::Renderer`.
pub type Card<'a, Message, Backend> = card::Card<'a, Message, Renderer<Backend>>;

impl<B> card::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_PADDING: f32 = 10.0;

    fn default_size(&self) -> f32 {
        self.backend().default_size() as f32
    }

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style>,
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
            border_radius: style.border_radius as u16, // TODO: will change in the future
            border_width: style.border_width as u16,   // TODO: same
            border_color: style.border_color,
        };

        let border = Primitive::Quad {
            bounds,
            background: Color::TRANSPARENT.into(),
            border_radius: style.border_radius as u16, // TODO: same
            border_width: style.border_width as u16,   // TODO: same
            border_color: style.border_color,
        };

        let head_layout = children.next().unwrap();
        let mut head_children = head_layout.children();
        let head_background = Primitive::Quad {
            bounds: head_layout.bounds(),
            background: style.head_background,
            border_radius: style.border_radius as u16, // TODO: same,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        let (head, new_mouse_interaction) = head.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: style.head_text_color,
                },
            },
            head_children.next().unwrap(),
            env.cursor_position,
            env.viewport.unwrap(),
        );

        let mouse_interaction = mouse_interaction.max(new_mouse_interaction);

        let (close, new_mouse_interaction) = head_children.next().map_or(
            (Primitive::None, mouse::Interaction::default()),
            |close_layout| {
                let close_bounds = close_layout.bounds();
                let is_mouse_over_close = close_bounds.contains(env.cursor_position);

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
                        horizontal_alignment: HorizontalAlignment::Center,
                        vertical_alignment: VerticalAlignment::Center,
                    },
                    if is_mouse_over_close {
                        mouse::Interaction::Pointer
                    } else {
                        mouse::Interaction::default()
                    },
                )
            },
        );

        let mouse_interaction = mouse_interaction.max(new_mouse_interaction);

        let body_layout = children.next().unwrap();
        let mut body_children = body_layout.children();
        let body_background = Primitive::Quad {
            bounds: body_layout.bounds(),
            background: style.body_background,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        let (body, new_mouse_interaction) = body.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: style.body_text_color,
                },
            },
            body_children.next().unwrap(),
            env.cursor_position,
            env.viewport.unwrap(),
        );

        let mouse_interaction = mouse_interaction.max(new_mouse_interaction);

        let foot_layout = children.next().unwrap();
        let mut foot_children = foot_layout.children();
        let foot_background = Primitive::Quad {
            bounds: foot_layout.bounds(),
            background: style.foot_background,
            border_radius: style.border_radius as u16, // TODO: same
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        let (foot, new_mouse_interaction) = if let Some(foot) = foot.as_ref() {
            foot.draw(
                self,
                &Defaults {
                    text: defaults::Text {
                        color: style.foot_text_color,
                    },
                },
                foot_children.next().unwrap(),
                env.cursor_position,
                env.viewport.unwrap(),
            )
        } else {
            (Primitive::None, mouse::Interaction::default())
        };

        let mouse_interaction = mouse_interaction.max(new_mouse_interaction);

        (
            Primitive::Group {
                primitives: vec![
                    background,
                    head_background,
                    body_background,
                    foot_background,
                    border,
                    head,
                    close,
                    body,
                    foot,
                ],
            },
            mouse_interaction,
        )
    }
}

//! A menu bar.
//!
//! *This API requires the following crate features to be activated: `menu`*
use std::collections::HashMap;

use iced_graphics::{
    backend, defaults, Backend, Background, Color, Defaults, Primitive, Rectangle, Renderer, Vector,
};
use iced_native::mouse;

pub use crate::native::menu::{Entry, Section, State};
use crate::{
    core::{menu::get_entry_list, renderer::DrawEnvironment},
    native::menu,
    style::{
        menu::{Style, StyleSheet},
        style_state::StyleState,
    },
};

use super::icons::{Icon, ICON_FONT};

/// A menu bar.
///
/// This is an alias of the `iced_native` `Menu` with an `iced_wgpu::Renderer`.
pub type Menu<'a, Message, Backend> = menu::Menu<'a, Message, Renderer<Backend>>;

impl<B> menu::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        sections: &[menu::Section<'_, Message, Self>],
    ) -> Self::Output {
        let bounds = env.layout.bounds();
        let children = env.layout.children();

        let mut style: HashMap<StyleState, Style> = HashMap::new();
        let _ = style.insert(StyleState::Active, env.style_sheet.active());
        let _ = style.insert(StyleState::Selected, env.style_sheet.selected());
        let _ = style.insert(StyleState::Hovered, env.style_sheet.hovered());
        let _ = style.insert(StyleState::Focused, env.style_sheet.focused());

        let mut mouse_interaction = mouse::Interaction::default();

        let mut style_state = StyleState::Active;
        if bounds.contains(env.cursor_position) {
            style_state = style_state.max(StyleState::Hovered);
        }

        // TODO: Implement proper shadow support
        let shadow = if style[&style_state].shadow_offset == Vector::default() {
            Primitive::None
        } else {
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x + style[&style_state].shadow_offset.x,
                    y: bounds.y + style[&style_state].shadow_offset.y,
                    ..bounds
                },
                background: Background::Color([0.0, 0.0, 0.0, 0.5].into()),
                border_radius: style[&style_state].border_radius,
                border_width: style[&style_state].border_width,
                border_color: Color::TRANSPARENT,
            }
        };

        let background = Primitive::Quad {
            bounds,
            background: style[&style_state].background,
            border_radius: style[&style_state].border_radius,
            border_width: style[&style_state].border_width,
            border_color: style[&style_state].border_color,
        };

        let mut section_primitives: Vec<Primitive> = sections
            .iter()
            .zip(children)
            .map(|(section, layout)| {
                let style_state = if layout.bounds().contains(env.cursor_position) {
                    StyleState::Hovered
                } else {
                    StyleState::Active
                };

                let background =
                    style[&style_state]
                        .label_background
                        .map_or(Primitive::None, |background| Primitive::Quad {
                            bounds: layout.bounds(),
                            background,
                            border_radius: 0.0,
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                        });

                let (label, new_mouse_interaction) = section.label.draw(
                    self,
                    &Defaults {
                        text: defaults::Text {
                            color: style[&style_state].text_color,
                        },
                    },
                    layout,
                    env.cursor_position,
                    env.viewport.expect("A viewport should exist for Menu"),
                );

                mouse_interaction = mouse_interaction.max(new_mouse_interaction);

                Primitive::Group {
                    primitives: vec![background, label],
                }
            })
            .collect();

        let mut primitives = vec![shadow, background];
        primitives.append(&mut section_primitives);

        (Primitive::Group { primitives }, mouse_interaction)
    }
}

impl<B> crate::native::overlay::menu::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        state: &State,
        section: &Section<'_, Message, Self>,
    ) -> Self::Output {
        let children = env.layout.children();

        let mut style: HashMap<StyleState, Style> = HashMap::new();
        let _ = style.insert(StyleState::Active, env.style_sheet.active());
        let _ = style.insert(StyleState::Selected, env.style_sheet.selected());
        let _ = style.insert(StyleState::Hovered, env.style_sheet.hovered());
        let _ = style.insert(StyleState::Focused, env.style_sheet.focused());

        let mut mouse_interaction = mouse::Interaction::default();

        let primitives = children
            .into_iter()
            .zip(get_entry_list(section, &state.stack))
            .map(|(layout, entries)| {
                let (primitive, new_mouse_interaction) = draw_entries(
                    self,
                    &DrawEnvironment {
                        defaults: env.defaults,
                        layout,
                        cursor_position: env.cursor_position,
                        style_sheet: &style,
                        viewport: env.viewport,
                        focus: (),
                    },
                    state,
                    //&section.entries,
                    entries,
                );

                mouse_interaction = mouse_interaction.max(new_mouse_interaction);

                primitive
            })
            .collect();

        (Primitive::Group { primitives }, mouse_interaction)
    }
}

/// Draws the entries.
fn draw_entries<'a, Message, B>(
    renderer: &mut Renderer<B>,
    env: &DrawEnvironment<'_, Defaults, HashMap<StyleState, Style>, ()>,
    _state: &State,
    entries: &[Entry<'a, Message, Renderer<B>>],
) -> (Primitive, mouse::Interaction)
where
    B: Backend + backend::Text,
{
    let bounds = env.layout.bounds();

    let mut mouse_interaction = mouse::Interaction::default();

    // TODO: Implement proper shadow support
    let shadow = if env.style_sheet[&StyleState::Active].overlay_shadow_offset == Vector::default()
    {
        Primitive::None
    } else {
        Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x + env.style_sheet[&StyleState::Active].overlay_shadow_offset.x,
                y: bounds.y + env.style_sheet[&StyleState::Active].overlay_shadow_offset.y,
                ..bounds
            },
            background: Background::Color([0.0, 0.0, 0.0, 0.5].into()),
            border_radius: env.style_sheet[&StyleState::Active].overlay_border_radius,
            border_width: env.style_sheet[&StyleState::Active].overlay_border_width,
            border_color: Color::TRANSPARENT,
        }
    };

    let background = Primitive::Quad {
        bounds,
        background: env.style_sheet[&StyleState::Active].overlay_background,
        border_radius: env.style_sheet[&StyleState::Active].overlay_border_radius,
        border_width: env.style_sheet[&StyleState::Active].overlay_border_width,
        border_color: env.style_sheet[&StyleState::Active].overlay_border_color,
    };

    let mut entry_primitives: Vec<Primitive> = entries
        .iter()
        .zip(env.layout.children())
        .map(|(entry, layout)| {
            let bounds = layout.bounds();

            let style_state = if layout.bounds().contains(env.cursor_position) {
                StyleState::Hovered
            } else {
                StyleState::Active
            };

            let background = env.style_sheet[&style_state]
                .overlay_label_background
                .map_or(Primitive::None, |background| Primitive::Quad {
                    bounds,
                    background,
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                });

            let label_layout = layout
                .children()
                .next()
                .expect("Graphics: Entry should have a label layout");
            let (primitive, new_mouse_interaction) = match entry {
                Entry::Item(element, _) | Entry::Group(element, _) => element.draw(
                    renderer,
                    &Defaults {
                        text: defaults::Text {
                            color: env.style_sheet[&style_state].overlay_text_color,
                        },
                    },
                    label_layout,
                    env.cursor_position,
                    &label_layout.bounds(),
                ),
                Entry::Separator => {
                    // TODO
                    (Primitive::None, mouse::Interaction::default())
                }
            };

            let group_icon = match entry {
                Entry::Group(_, _) => Primitive::Text {
                    content: Icon::CaretRightFill.into(),
                    bounds: Rectangle {
                        x: bounds.x + bounds.width - 8.0, // TODO
                        y: bounds.center_y(),
                        width: 16.0,
                        height: 16.0,
                    },
                    color: env.style_sheet[&style_state].overlay_text_color,
                    size: 16.0,
                    font: ICON_FONT,
                    horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                    vertical_alignment: iced_graphics::VerticalAlignment::Center,
                },
                _ => Primitive::None,
            };

            mouse_interaction = mouse_interaction.max(new_mouse_interaction);

            Primitive::Group {
                primitives: vec![background, primitive, group_icon],
            }
        })
        .collect();

    let mut primitives = vec![shadow, background];
    primitives.append(&mut entry_primitives);

    (Primitive::Group { primitives }, mouse_interaction)
}

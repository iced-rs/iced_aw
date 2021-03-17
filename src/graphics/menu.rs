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
use crate::native::overlay::menu::{GROUP_ICON_SIZE, TOGGLE_ICON_SIZE};

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
        let _ = style.insert(StyleState::Disabled, env.style_sheet.disabled());

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
                let mut style_state = StyleState::Active;
                if layout.bounds().contains(env.cursor_position) {
                    style_state = style_state.max(StyleState::Hovered);
                }
                if section.entries.is_empty() {
                    style_state = style_state.max(StyleState::Disabled);
                }

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
        let _ = style.insert(StyleState::Disabled, env.style_sheet.disabled());

        let mut mouse_interaction = mouse::Interaction::default();

        let mut selected_childs: Vec<Option<usize>> =
            state.stack.iter().skip(1).map(|i| Some(*i)).fold(
                Vec::with_capacity(state.stack.len()),
                |mut v, i| {
                    v.push(i);
                    v
                },
            );
        selected_childs.push(None);

        let primitives = children
            .into_iter()
            .zip(get_entry_list(section, &state.stack))
            .zip(selected_childs)
            .map(|((layout, entries), selected_child)| {
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
                    selected_child,
                );

                mouse_interaction = mouse_interaction.max(new_mouse_interaction);

                primitive
            })
            .collect();

        (Primitive::Group { primitives }, mouse_interaction)
    }
}

/// Draws the entries.
#[allow(clippy::too_many_lines)]
fn draw_entries<'a, Message, B>(
    renderer: &mut Renderer<B>,
    env: &DrawEnvironment<'_, Defaults, HashMap<StyleState, Style>, ()>,
    _state: &State,
    entries: &[Entry<'a, Message, Renderer<B>>],
    selected_child: Option<usize>,
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
        .enumerate()
        .zip(env.layout.children())
        .map(|((i, entry), layout)| {
            let bounds = layout.bounds();

            let is_disabled = match entry {
                Entry::Item(_, message) => message.is_none(),
                Entry::Toggle(_, _, message) => message.is_none(),
                Entry::Group(_, entries) => entries.is_empty(),
                Entry::Separator => false,
            };

            let mut style_state = StyleState::Active;
            if (layout.bounds().contains(env.cursor_position) && selected_child.is_none())
                || Some(i) == selected_child
            {
                style_state = style_state.max(StyleState::Hovered);
            }
            if is_disabled {
                style_state = style_state.max(StyleState::Disabled)
            }

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
                Entry::Item(element, _)
                | Entry::Toggle(element, _, _)
                | Entry::Group(element, _) => element.draw(
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

            let checkmark_icon = if let Entry::Toggle(_, checked, _) = entry {
                // TODO: clean up, once chained if let becomes stable.
                if *checked {
                    Primitive::Text {
                        content: Icon::Check.into(),
                        bounds: Rectangle {
                            x: bounds.x + TOGGLE_ICON_SIZE / 2.0,
                            y: bounds.center_y(),
                            width: TOGGLE_ICON_SIZE,
                            height: TOGGLE_ICON_SIZE,
                        },
                        color: env.style_sheet[&style_state].overlay_text_color,
                        size: TOGGLE_ICON_SIZE,
                        font: ICON_FONT,
                        horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                        vertical_alignment: iced_graphics::VerticalAlignment::Center,
                    }
                } else {
                    Primitive::None
                }
            } else {
                Primitive::None
            };

            let group_icon = if let Entry::Group(_, _) = entry {
                Primitive::Text {
                    content: Icon::CaretRightFill.into(),
                    bounds: Rectangle {
                        x: bounds.x + bounds.width - GROUP_ICON_SIZE / 2.0,
                        y: bounds.center_y(),
                        width: GROUP_ICON_SIZE,
                        height: GROUP_ICON_SIZE,
                    },
                    color: env.style_sheet[&style_state].overlay_text_color,
                    size: GROUP_ICON_SIZE,
                    font: ICON_FONT,
                    horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                    vertical_alignment: iced_graphics::VerticalAlignment::Center,
                }
            } else {
                Primitive::None
            };

            mouse_interaction = mouse_interaction.max(new_mouse_interaction);

            Primitive::Group {
                primitives: vec![background, checkmark_icon, primitive, group_icon],
            }
        })
        .collect();

    let mut primitives = vec![shadow, background];
    primitives.append(&mut entry_primitives);

    (
        Primitive::Clip {
            bounds,
            offset: Vector::new(0, 0),
            content: Box::new(Primitive::Group { primitives }),
        },
        mouse_interaction,
    )
}

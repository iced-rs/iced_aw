//! Displays a [`TabBar`](TabBar) to select the content to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](super::tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*
use iced_graphics::{backend, Backend, Color, Primitive, Rectangle, Renderer};
use iced_native::{mouse, Font, HorizontalAlignment, Layout, VerticalAlignment};
pub use tab_bar::tab_label::TabLabel;

pub use crate::style::tab_bar::{Style, StyleSheet};
use crate::{core::renderer::DrawEnvironment, native::tab_bar};

/// A tab bar to show tabs.
///
/// This is an alias of an `iced_native` `TabBar` with an `iced_wgpu::Renderer`.
pub type TabBar<Message, Backend> = tab_bar::TabBar<Message, Renderer<Backend>>;

impl<B> tab_bar::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_ICON_SIZE: u16 = 32;

    const DEFAULT_TEXT_SIZE: u16 = 16;

    const DEFAULT_CLOSE_SIZE: u16 = 16;

    const DEFAULT_PADDING: u16 = 5;

    const DEFAULT_SPACING: u16 = 0;

    fn draw(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        active_tab: usize,
        tab_labels: &[TabLabel],
        icon_font: Option<Font>,
        text_font: Option<Font>,
    ) -> Self::Output {
        // TODO tab bar background
        let bounds = env.layout.bounds();
        let children = env.layout.children();
        let is_mouse_over = bounds.contains(env.cursor_position);
        let style = if is_mouse_over {
            env.style_sheet.hovered(false)
        } else {
            env.style_sheet.active(false)
        };

        let mut mouse_interaction = mouse::Interaction::default();

        let mut primitives = vec![Primitive::Quad {
            bounds,
            background: style
                .background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
            border_radius: 0.0,
            border_width: style.border_width,
            border_color: style.border_color.unwrap_or(Color::TRANSPARENT),
        }];

        primitives = tab_labels.iter().enumerate().zip(children).fold(
            primitives,
            |mut primitives, ((i, tab), layout)| {
                let (primitive, new_mouse_interaction) = draw_tab(
                    tab,
                    layout,
                    env.style_sheet,
                    i == active_tab,
                    env.cursor_position,
                    icon_font.unwrap_or(B::ICON_FONT),
                    text_font.unwrap_or_default(),
                );

                if new_mouse_interaction > mouse_interaction {
                    mouse_interaction = new_mouse_interaction;
                }

                primitives.push(primitive);
                primitives
            },
        );

        (Primitive::Group { primitives }, mouse_interaction)
    }
}

/// Draws a tab.
#[allow(clippy::borrowed_box, clippy::too_many_lines)]
fn draw_tab(
    tab: &TabLabel,
    layout: Layout<'_>,
    style_sheet: &Box<dyn StyleSheet>,
    is_selected: bool,
    cursor_position: iced_native::Point,
    icon_font: Font,
    text_font: Font,
) -> (Primitive, mouse::Interaction) {
    let is_mouse_over = layout.bounds().contains(cursor_position);
    let style = if is_mouse_over {
        style_sheet.hovered(is_selected)
    } else {
        style_sheet.active(is_selected)
    };

    let bounds = layout.bounds();
    let mut children = layout.children();
    let label_layout = children
        .next()
        .expect("Graphics: Layout should have a label layout");
    let mut label_layout_children = label_layout.children();

    let background = Primitive::Quad {
        bounds,
        background: style.tab_label_background,
        border_radius: 0.0,
        border_width: style.tab_label_border_width,
        border_color: style.tab_label_border_color,
    };

    let cross = children.next().map_or(Primitive::None, |cross_layout| {
        let cross_bounds = cross_layout.bounds();
        let is_mouse_over_cross = cross_bounds.contains(cursor_position);

        Primitive::Text {
            content: super::icons::Icon::X.into(),
            font: super::icons::ICON_FONT,
            size: cross_bounds.height + if is_mouse_over_cross { 5.0 } else { 0.0 },
            bounds: Rectangle {
                x: cross_bounds.center_x(),
                y: cross_bounds.center_y(),
                ..cross_bounds
            },
            color: style.icon_color,
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        }
    });

    let primitive = match tab {
        TabLabel::Icon(icon) => {
            let icon_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have an icon layout for an Icon")
                .bounds();

            Primitive::Group {
                primitives: vec![
                    background,
                    Primitive::Text {
                        content: icon.to_string(),
                        font: icon_font,
                        size: icon_bounds.height,
                        bounds: Rectangle {
                            x: icon_bounds.center_x(),
                            y: icon_bounds.center_y(),
                            ..icon_bounds
                        },
                        color: style.icon_color,
                        horizontal_alignment: HorizontalAlignment::Center,
                        vertical_alignment: VerticalAlignment::Center,
                    },
                    cross,
                ],
            }
        }
        TabLabel::Text(text) => {
            let text_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have a text layout for a Text")
                .bounds();

            Primitive::Group {
                primitives: vec![
                    background,
                    Primitive::Text {
                        content: text.to_string(),
                        font: text_font,
                        size: text_bounds.height,
                        bounds: Rectangle {
                            x: text_bounds.center_x(),
                            y: text_bounds.center_y(),
                            ..text_bounds
                        },
                        color: style.text_color,
                        horizontal_alignment: HorizontalAlignment::Center,
                        vertical_alignment: VerticalAlignment::Center,
                    },
                    cross,
                ],
            }
        }
        TabLabel::IconText(icon, text) => {
            let icon_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have an icons layout for an IconText")
                .bounds();
            let text_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have a text layout for an IconText")
                .bounds();

            Primitive::Group {
                primitives: vec![
                    background,
                    Primitive::Text {
                        content: icon.to_string(),
                        font: icon_font,
                        size: icon_bounds.height,
                        bounds: Rectangle {
                            x: icon_bounds.center_x(),
                            y: icon_bounds.center_y(),
                            ..icon_bounds
                        },
                        color: style.icon_color,
                        horizontal_alignment: HorizontalAlignment::Center,
                        vertical_alignment: VerticalAlignment::Center,
                    },
                    Primitive::Text {
                        content: text.to_string(),
                        font: text_font,
                        size: text_bounds.height,
                        bounds: Rectangle {
                            x: text_bounds.center_x(),
                            y: text_bounds.center_y(),
                            ..text_bounds
                        },
                        color: style.text_color,
                        horizontal_alignment: HorizontalAlignment::Center,
                        vertical_alignment: VerticalAlignment::Center,
                    },
                    cross,
                ],
            }
        }
    };

    (
        primitive,
        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        },
    )
}

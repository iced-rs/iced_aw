//! [`MenuBar`]

#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use iced::{
    advanced::{
        layout::{Limits, Node},
        mouse, overlay, renderer,
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    alignment, event, Element, Event, Length, Padding, Rectangle, Size,
    };

use super::{common::*, flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*};
use crate::style::menu_bar::*;

#[derive(Default)]
pub(super) struct MenuBarState {
    pub(super) active_root: Index,
    pub(super) open: bool,
    pub(super) is_pressed: bool,
}

/// menu bar
#[must_use]
pub struct MenuBar<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    roots: Vec<Item<'a, Message, Theme, Renderer>>,
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    check_bounds_width: f32,
    style: Theme::Style,
}
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    /// Creates a [`MenuBar`] with the given root items.
    pub fn new(mut roots: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        for i in &mut roots {
            if let Some(m) = i.menu.as_mut() {
                m.axis = Axis::Vertical;
            }
        }

        Self {
            roots,
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            check_bounds_width: 50.0,
            style: Theme::Style::default(),
        }
    }

    /// Sets the width of the [`MenuBar`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`MenuBar`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the spacing of the [`MenuBar`].
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the width of the check bounds of the [`Menu`]s in the [`MenuBar`].
    pub fn check_bounds_width(mut self, check_bounds_width: f32) -> Self {
        self.check_bounds_width = check_bounds_width;
        self
    }

    /// Sets the padding of the [`MenuBar`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the style variant of this [`MenuBar`].
    pub fn style(mut self, style: impl Into<Theme::Style>) -> Self {
        self.style = style.into();
        self
    }
}
impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MenuBar<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuBarState>()
    }

    fn state(&self) -> tree::State {
        tree::State::Some(Box::<MenuBarState>::default())
    }

    /// \[Tree{stateless, \[widget_state, menu_state]}...]
    fn children(&self) -> Vec<Tree> {
        self.roots
            .iter()
            .map(Item::tree)
            .collect::<Vec<_>>()
    }

    /// tree: Tree{bar_state, \[item_tree...]}
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(
            &self.roots,
            |tree, item| item.diff(tree),
            Item::tree,
        );
    }

    /// tree: Tree{bar_state, \[item_tree...]}
    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            alignment::Alignment::Center,
            &self.roots.iter().map(|item| &item.item).collect::<Vec<_>>(),
            &mut tree
                .children
                .iter_mut()
                .map(|tree| &mut tree.children[0])
                .collect::<Vec<_>>(),
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        use event::Status::*;

        let status = self
            .roots
            .iter_mut() // [Item...]
            .zip(tree.children.iter_mut()) // [item_tree...]
            .zip(layout.children()) // [widget_node...]
            .map(|((item, tree), layout)| {
                item.on_event(
                    tree,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(Ignored, event::Status::merge);

        let bar = tree.state.downcast_mut::<MenuBarState>();
        let bar_bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(bar_bounds) {
                    bar.is_pressed = true;
                    Captured
                } else {
                    Ignored
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if cursor.is_over(bar_bounds) && bar.is_pressed {
                    bar.open = true;
                    bar.is_pressed = false;
                    for (i, l) in layout.children().enumerate() {
                        if cursor.is_over(l.bounds()) {
                            bar.active_root = Some(i);
                            break;
                        }
                    }
                    Captured
                } else {
                    Ignored
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if bar.open {
                    if cursor.is_over(bar_bounds) {
                        for (i, l) in layout.children().enumerate() {
                            if cursor.is_over(l.bounds()) {
                                bar.active_root = Some(i);
                                break;
                            }
                        }
                    } else {
                        bar.open = false;
                    }
                    Captured
                } else {
                    Ignored
                }
            }
            _ => Ignored,
        }
        .merge(status)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<MenuBarState>();
        let cursor = if state.open {
            state
                .active_root
                .and_then(|active| {
                    layout
                        .children()
                        .nth(active)
                        .map(|l| mouse::Cursor::Available(l.bounds().center()))
                })
                .unwrap_or(cursor)
        } else {
            cursor
        };

        let styling = theme.appearance(&self.style);
        renderer.fill_quad(
            renderer::Quad {
                bounds: pad_rectangle(layout.bounds(), styling.bar_background_expand),
                border: styling.bar_border,
                shadow: styling.bar_shadow,
            },
            styling.bar_background,
        );

        self.roots
            .iter() // [Item...]
            .zip(tree.children.iter()) // [item_tree...]
            .zip(layout.children()) // [widget_node...]
            .for_each(|((item, tree), layout)| {
                item.draw(tree, renderer, theme, style, layout, cursor, viewport);
            });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<MenuBarState>();

        let init_bar_bounds = layout.bounds();
        let init_root_bounds = layout.children().map(|l| l.bounds()).collect();

        if state.open {
            Some(
                MenuBarOverlay {
                    translation,
                    tree,
                    roots: &mut self.roots,
                    init_bar_bounds,
                    init_root_bounds,
                    check_bounds_width: self.check_bounds_width,
                    style: &self.style,
                }
                .overlay_element(),
            )
        } else {
            None
        }
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: 'a + renderer::Renderer,
{
    fn from(value: MenuBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

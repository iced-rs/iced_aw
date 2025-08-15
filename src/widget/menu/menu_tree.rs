//! [`Item`] and [`Menu`]
//!
#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unused_self)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::pedantic)]
#![allow(clippy::similar_names)]

use super::common::*;
use super::flex;
use iced::advanced::overlay::Group;
use iced::advanced::widget::Operation;
use iced::Pixels;
use iced::{
    advanced::{
        layout::{Layout, Limits, Node},
        mouse, overlay, renderer,
        widget::tree::{self, Tree},
        Clipboard, Shell,
    },
    alignment, Element, Event, Length, Padding, Point, Rectangle, Size, Vector,
};
use std::iter::once;

use crate::style::menu_bar::*;

/*
menu tree:
Item{
    widget
    Menu [
        Item{...}
        Item{...}
        Item{...}
        ...
    ]
}

state tree:
Tree{
    item state
    [
        Tree{widget state}
        Tree{
            menu state
            [
                Tree{item state [...]}
                Tree{item state [...]}
                Tree{item state [...]}
                ...
            ]
        }
    ]
}

*/

#[derive(Debug)]
pub(super) struct MenuState {
    scroll_offset: f32,
    pub(super) active: Index,
    pub(super) slice: MenuSlice,
    pub(super) pressed: bool,
}
impl Default for MenuState {
    fn default() -> Self {
        Self {
            scroll_offset: 0.0,
            active: None,
            slice: MenuSlice {
                start_index: 0,
                end_index: usize::MAX - 1,
                lower_bound_rel: 0.0,
                upper_bound_rel: f32::MAX,
            },
            pressed: false,
        }
    }
}

/// Menu
#[must_use]
pub struct Menu<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) items: Vec<Item<'a, Message, Theme, Renderer>>,
    pub(super) spacing: Pixels,
    pub(super) max_width: f32,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) axis: Axis,
    pub(super) offset: f32,
}
impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    /// Creates a [`Menu`] with the given items.
    pub fn new(items: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            items,
            spacing: Pixels::ZERO,
            max_width: f32::MAX,
            width: Length::Fill,
            height: Length::Shrink,
            axis: Axis::Horizontal,
            offset: 0.0,
        }
    }

    /// Sets the maximum width of the [`Menu`].
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the width of the [`Menu`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the spacing of the [`Menu`].
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// The offset from the menu's parent item.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Rebuild state tree
    pub(super) fn tree(&self) -> Tree {
        Tree {
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<Message, Theme, Renderer> Menu<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    // pub(super) fn size(&self) -> Size<Length> {
    //     Size::new(self.width, self.height)
    // }

    pub(super) fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuState>()
    }

    pub(super) fn state(&self) -> tree::State {
        tree::State::Some(Box::<MenuState>::default())
    }

    /// out: \[item_tree...]
    pub(super) fn children(&self) -> Vec<Tree> {
        self.items.iter().map(Item::tree).collect()
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    pub(super) fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(&self.items, |tree, item| item.diff(tree), Item::tree);
    }

    /// tree: Tree{ menu_state, \[item_tree...] }
    ///
    /// out: Node{inf, \[ items_node, prescroll, offset_bounds, check_bounds ]}
    pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
        check_bounds_width: f32,
        parent_bounds: Rectangle,
        parent_direction: (Direction, Direction),
        viewport: &Rectangle,
    ) -> (Node, (Direction, Direction)) {
        let limits = limits.max_width(self.max_width);

        let items_node = flex::resolve(
            flex::Axis::Vertical,
            renderer,
            &limits,
            self.width,
            self.height,
            Padding::ZERO,
            self.spacing,
            alignment::Alignment::Center,
            &self.items.iter().map(|i| &i.item).collect::<Vec<_>>(),
            &mut tree
                .children
                .iter_mut()
                .map(|t| &mut t.children[0])
                .collect::<Vec<_>>(),
        );

        let aod = Aod::new(
            self.axis,
            viewport.size(),
            parent_bounds,
            parent_direction,
            self.offset,
        );

        let children_size = items_node.bounds().size();
        let (children_position, offset_position, child_direction) =
            aod.resolve(parent_bounds, children_size, viewport.size());

        // calc auxiliary bounds
        let delta = children_position - offset_position;
        let offset_size = if delta.x.abs() > delta.y.abs() {
            Size::new(self.offset, children_size.height)
        } else {
            Size::new(children_size.width, self.offset)
        };

        let offset_bounds = Rectangle::new(offset_position, offset_size);
        let children_bounds = Rectangle::new(children_position, children_size);
        let check_bounds = pad_rectangle(children_bounds, check_bounds_width.into());

        let menu_state = tree.state.downcast_mut::<MenuState>();

        // calc slice
        let slice = MenuSlice::new(
            &items_node,
            children_position - Point::ORIGIN,
            viewport.size(),
            menu_state.scroll_offset,
        );
        menu_state.slice = slice;

        let slice_node = if slice.start_index == slice.end_index {
            let node = &items_node.children()[slice.start_index];
            let bounds = node.bounds();
            let start_offset = slice.lower_bound_rel - bounds.y;
            let height = slice.upper_bound_rel - slice.lower_bound_rel;

            Node::with_children(
                Size::new(items_node.bounds().width, height),
                once(clip_node_y(node, height, start_offset)).collect(),
            )
        } else {
            let start_node = {
                let node = &items_node.children()[slice.start_index];
                let bounds = node.bounds();
                let start_offset = slice.lower_bound_rel - bounds.y;
                let height = bounds.height - start_offset;
                clip_node_y(node, height, start_offset)
            };

            let end_node = {
                let node = &items_node.children()[slice.end_index];
                let bounds = node.bounds();
                let height = slice.upper_bound_rel - bounds.y;
                clip_node_y(node, height, 0.0)
            };

            Node::with_children(
                Size::new(
                    items_node.bounds().width,
                    slice.upper_bound_rel - slice.lower_bound_rel,
                ),
                once(start_node)
                    .chain(
                        items_node.children()[slice.start_index + 1..slice.end_index]
                            .iter()
                            .map(Clone::clone),
                    )
                    .chain(once(end_node))
                    .collect(),
            )
        };

        (
            Node::with_children(
                Size::INFINITY,
                [
                    slice_node
                        .move_to(children_position)
                        .translate([0.0, menu_state.scroll_offset]), // slice layout
                    Node::new(children_size).move_to(children_position), // prescroll bounds
                    Node::new(offset_bounds.size()).move_to(offset_bounds.position()), // offset boundss
                    Node::new(check_bounds.size()).move_to(check_bounds.position()), // check bounds
                ]
                .into(),
            ),
            child_direction,
        )
    }

    /// tree: Tree{ menu_state, \[item_tree...] }
    ///
    /// layout: Node{inf, \[ slice_node, prescroll, offset_bounds, check_bounds ]}
    pub(super) fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
        scroll_speed: ScrollSpeed,
    ) {
        // STK: cleanup
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = &menu_state.slice;

        for ((item, tree), layout) in self.items[slice.start_index..=slice.end_index] // [item...]
            .iter_mut()
            .zip(tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
            .zip(slice_layout.children())
        {
            item.update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            )
        }
        // let status = self.items[slice.start_index..=slice.end_index] // [item...]
        //     .iter_mut()
        //     .zip(tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
        //     .zip(slice_layout.children()) // [item_layout...]
        //     .map(|((item, tree), layout)| {
        //         item.on_event(
        //             tree,
        //             event.clone(),
        //             layout,
        //             cursor,
        //             renderer,
        //             clipboard,
        //             shell,
        //             viewport,
        //         )
        //     })
        //     .fold(Ignored, event::Status::merge);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(prescroll) {
                    menu_state.pressed = true;
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                menu_state.pressed = false;
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(prescroll) {
                    process_scroll_event(
                        menu_state,
                        prescroll,
                        *delta,
                        scroll_speed,
                        viewport.size(),
                    );
                    shell.capture_event();
                } else if cursor.is_over(offset_bounds) || cursor.is_over(check_bounds) {
                    shell.capture_event();
                }
                shell.request_redraw();
            }
            _ => {}
        }
    }

    pub(super) fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let _prescroll = lc.next().unwrap().bounds();
        let _offset_bounds = lc.next().unwrap().bounds();
        let _check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = &menu_state.slice;

        operation.container(None, layout.bounds(), &mut |operation| {
            self.items[slice.start_index..=slice.end_index] // [item...]
                .iter()
                .zip(tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
                .zip(slice_layout.children())
                .for_each(|((child, state), layout)| {
                    child.operate(state, layout, renderer, operation);
                });
        });
    }

    #[allow(dead_code)]
    pub(super) fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let mut lc = layout.children();
        let slice_layout = lc.next()?;
        let _prescroll = lc.next()?.bounds();
        let _offset_bounds = lc.next()?.bounds();
        let _check_bounds = lc.next()?.bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = &menu_state.slice;

        let children = self.items[slice.start_index..=slice.end_index] // [item...]
            .iter_mut()
            .zip(tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
            .zip(slice_layout.children())
            .filter_map(|((child, state), layout)| {
                child.overlay(state, layout, renderer, translation)
            })
            .collect::<Vec<_>>();

        (!children.is_empty()).then(|| Group::with_children(children).overlay())
    }

    /// tree: Tree{ menu_state, \[item_tree...] }
    ///
    /// layout: Node{inf, \[ slice_node, prescroll, offset_bounds, check_bounds ]}
    pub(super) fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();

        let menu_state = tree.state.downcast_ref::<MenuState>();
        let slice = &menu_state.slice;
        let max_item_slice = (self.items.len() - 1).min(slice.end_index);
        let max_tree_slice = (tree.children.len() - 1).min(slice.end_index);

        self.items[slice.start_index..=max_item_slice]
            .iter()
            .zip(tree.children[slice.start_index..=max_tree_slice].iter()) // [item_tree...]
            .zip(slice_layout.children()) // [item_layout...]
            .map(|((item, tree), layout)| item.mouse_interaction(tree, layout, cursor, renderer))
            .max()
            .unwrap_or_default()
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    ///
    /// layout: Node{inf, \[ items_node, slice_node, prescroll, offset_bounds, check_bounds ]}
    pub(super) fn draw(
        &self,
        draw_path: &DrawPath,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        theme_style: &Style,
        layout: Layout<'_>,
        mut cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        // let offset_bounds = lc.next().unwrap().bounds();
        // let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_ref::<MenuState>();
        let slice = &menu_state.slice;

        // debug_draw(renderer, prescroll, check_bounds, offset_bounds);

        // draw background
        let pad_rectangle = pad_rectangle(prescroll, theme_style.menu_background_expand);
        if pad_rectangle.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: pad_rectangle,
                    border: theme_style.menu_border,
                    shadow: theme_style.menu_shadow,
                    ..Default::default()
                },
                theme_style.menu_background,
            );
        }

        // draw path
        if let Some(active) = menu_state.active {
            let Some(active_bounds) = slice_layout
                .children()
                .nth(active - menu_state.slice.start_index)
                .map(|l| l.bounds())
            else {
                return;
            };

            match draw_path {
                DrawPath::Backdrop => {
                    if active_bounds.intersects(viewport) {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: active_bounds,
                                border: theme_style.path_border,
                                ..Default::default()
                            },
                            theme_style.path,
                        );
                    }
                }
                DrawPath::FakeHovering => {
                    if !cursor.is_over(active_bounds) {
                        cursor = mouse::Cursor::Available(active_bounds.center());
                    }
                }
            }
        }

        // prep start
        let Some(start) = self.items.get(slice.start_index) else {
            return;
        };
        let Some(start_tree) = tree.children.get(slice.start_index) else {
            return;
        };
        let Some(start_layout) = slice_layout.children().next() else {
            return;
        };

        if slice.end_index == slice.start_index {
            // draw start
            renderer.with_layer(start_layout.bounds(), |r| {
                start.draw(start_tree, r, theme, style, start_layout, cursor, viewport);
            });
        } else {
            // prep end
            let Some(end) = self.items.get(slice.end_index) else {
                return;
            };
            let Some(end_tree) = tree.children.get(slice.end_index) else {
                return;
            };
            let Some(end_layout) = slice_layout.children().last() else {
                return;
            };

            // draw start
            renderer.with_layer(start_layout.bounds(), |r| {
                start.draw(start_tree, r, theme, style, start_layout, cursor, viewport);
            });

            // draw middle
            if (slice.end_index - slice.start_index) > 1 {
                let Some(items) = self.items.get(slice.start_index + 1..=slice.end_index - 1)
                else {
                    return;
                };

                let Some(trees) = tree
                    .children
                    .get(slice.start_index + 1..=slice.end_index - 1)
                else {
                    return;
                };

                for ((item, tree), layout) in items
                    .iter()
                    .zip(trees.iter())
                    .zip(slice_layout.children().skip(1))
                {
                    item.draw(tree, renderer, theme, style, layout, cursor, viewport);
                }
            }

            // draw end
            renderer.with_layer(end_layout.bounds(), |r| {
                end.draw(end_tree, r, theme, style, end_layout, cursor, viewport)
            })
        }
    }

    pub(super) fn open_event(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        // let prescroll = lc.next().unwrap().bounds();
        // let offset_bounds = lc.next().unwrap().bounds();
        // let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = &menu_state.slice;
        let pre_menu_state = menu_state.active.take();

        for (i, (item, layout)) in self.items[slice.start_index..=slice.end_index]
            .iter()
            .zip(slice_layout.children())
            .enumerate()
        {
            if item.menu.is_some() && cursor.is_over(layout.bounds()) {
                if pre_menu_state != Some(i + slice.start_index) {
                    // New submenu selected, make sure it gets displayed
                    shell.request_redraw();
                }
                menu_state.active = Some(i + slice.start_index);
                return;
            }
        }
    }

    pub(super) fn close_event(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
        parent_bounds: Rectangle,
        prev_bounds_list: &[Rectangle],
        prev: &mut Index,
    ) {
        let mut lc = layout.children();
        let _slice_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();

        if menu_state.pressed {
            return;
        }

        let open = {
            if cursor.is_over(prescroll)
                || cursor.is_over(parent_bounds)
                || cursor.is_over(offset_bounds)
            {
                true
            } else if prev_bounds_list.iter().any(|r| cursor.is_over(*r)) {
                false
            } else {
                cursor.is_over(check_bounds)
            }
        };

        if !open {
            shell.request_redraw();
            *prev = None;
            menu_state.scroll_offset = 0.0;
            menu_state.active = None;
            menu_state.pressed = false;
        }
    }
}

/// Item inside a [`Menu`]
#[must_use]
pub struct Item<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) item: Element<'a, Message, Theme, Renderer>,
    pub(super) menu: Option<Box<Menu<'a, Message, Theme, Renderer>>>,
}
impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    /// Creates an [`Item`] with the given element.
    pub fn new(item: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            item: item.into(),
            menu: None,
        }
    }

    /// Creates an [`Item`] with the given element and menu.
    pub fn with_menu(
        item: impl Into<Element<'a, Message, Theme, Renderer>>,
        menu: Menu<'a, Message, Theme, Renderer>,
    ) -> Self {
        Self {
            item: item.into(),
            menu: Some(Box::new(menu)),
        }
    }

    /// Rebuild state tree
    pub(super) fn tree(&self) -> Tree {
        Tree {
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<Message, Theme, Renderer> Item<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    // pub(super) fn size(&self) -> Size<Length> {
    //     self.item.as_widget().size()
    // }

    pub(super) fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    pub(super) fn state(&self) -> tree::State {
        tree::State::None
    }

    /// out: \[widget_tree, menu_tree]
    pub(super) fn children(&self) -> Vec<Tree> {
        self.menu.as_ref().map_or_else(
            || [Tree::new(&self.item)].into(),
            |m| [Tree::new(&self.item), m.tree()].into(),
        )
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    #[allow(clippy::option_if_let_else)]
    pub(super) fn diff(&self, tree: &mut Tree) {
        if let Some(t0) = tree.children.get_mut(0) {
            t0.diff(&self.item);
            if let Some(m) = self.menu.as_ref() {
                if let Some(t1) = tree.children.get_mut(1) {
                    m.diff(t1);
                } else {
                    *tree = self.tree();
                }
            }
        } else {
            *tree = self.tree();
        }
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.item.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.item.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            &layout.bounds(),
            renderer,
        )
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.item.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    pub(super) fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.item
            .as_widget()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    #[allow(dead_code)]
    pub(super) fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.item.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            &layout.bounds(),
            translation,
        )
    }
}

/// Adaptive open direction
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
struct Aod {
    // whether or not to use overlap
    horizontal_overlap: bool,
    vertical_overlap: bool,

    // default direction
    horizontal_direction: Direction,
    vertical_direction: Direction,

    // Offset of the child in the default direction
    horizontal_offset: f32,
    vertical_offset: f32,
}
impl Aod {
    /// Returns (child position, offset position, child direction)
    fn adaptive(
        parent_pos: f32,
        parent_size: f32,
        child_size: f32,
        max_size: f32,
        offset: f32,
        overlap: bool,
        direction: Direction,
    ) -> (f32, f32, Direction) {
        /*
        Imagine there're two sticks, parent and child
        parent: o-----o
        child:  o----------o

        Now we align the child to the parent in one dimension
        There are 4 possibilities:

        1. to the right
                    o-----oo----------o

        2. to the right with overlaping
                    o-----o
                    o----------o

        3. to the left
        o----------oo-----o

        4. to the left with overlaping
                    o-----o
               o----------o

        The child goes to the default direction by default,
        if the space on the default direction runs out it goes to the the other,
        whether to use overlap is the caller's decision

        This can be applied to any direction
        */

        match direction {
            Direction::Positive => {
                let space_negative = parent_pos;
                let space_positive = max_size - parent_pos - parent_size;

                if overlap {
                    let overshoot = child_size - parent_size;
                    if space_negative > space_positive && overshoot > space_positive {
                        (
                            parent_pos - overshoot,
                            parent_pos - overshoot,
                            direction.flip(),
                        )
                    } else {
                        (parent_pos, parent_pos, direction)
                    }
                } else {
                    let overshoot = child_size + offset;
                    if space_negative > space_positive && overshoot > space_positive {
                        (
                            parent_pos - overshoot,
                            parent_pos - offset,
                            direction.flip(),
                        )
                    } else {
                        (
                            parent_pos + parent_size + offset,
                            parent_pos + parent_size,
                            direction,
                        )
                    }
                }
            }
            Direction::Negative => {
                let space_positive = parent_pos;
                let space_negative = max_size - parent_pos - parent_size;

                if overlap {
                    let overshoot = child_size - parent_size;
                    if space_negative > space_positive && overshoot > space_positive {
                        (parent_pos, parent_pos, direction.flip())
                    } else {
                        (parent_pos - overshoot, parent_pos - overshoot, direction)
                    }
                } else {
                    let overshoot = child_size + offset;
                    if space_negative > space_positive && overshoot > space_positive {
                        (
                            parent_pos + parent_size + offset,
                            parent_pos + parent_size,
                            direction.flip(),
                        )
                    } else {
                        (parent_pos - overshoot, parent_pos - offset, direction)
                    }
                }
            }
        }
    }

    /// Returns (child position, offset position, child direction)
    fn resolve(
        &self,
        parent_bounds: Rectangle,
        children_size: Size,
        viewport_size: Size,
    ) -> (Point, Point, (Direction, Direction)) {
        let (x, ox, dx) = Self::adaptive(
            parent_bounds.x,
            parent_bounds.width,
            children_size.width,
            viewport_size.width,
            self.horizontal_offset,
            self.horizontal_overlap,
            self.horizontal_direction,
        );
        let (y, oy, dy) = Self::adaptive(
            parent_bounds.y,
            parent_bounds.height,
            children_size.height,
            viewport_size.height,
            self.vertical_offset,
            self.vertical_overlap,
            self.vertical_direction,
        );

        ([x, y].into(), [ox, oy].into(), (dx, dy))
    }

    fn new(
        axis: Axis,
        viewport: Size,
        parent_bounds: Rectangle,
        parent_direction: (Direction, Direction),
        offset: f32,
    ) -> Self {
        let hcenter = viewport.width / 2.0;
        let vcenter = viewport.height / 2.0;

        let phcenter = parent_bounds.x + parent_bounds.width / 2.0;
        let pvcenter = parent_bounds.y + parent_bounds.height / 2.0;

        let (pdx, pdy) = parent_direction;
        match axis {
            Axis::Horizontal => {
                let horizontal_direction = pdx;
                let vertical_direction = if pvcenter < vcenter {
                    Direction::Positive
                } else {
                    Direction::Negative
                };
                Self {
                    horizontal_overlap: false,
                    vertical_overlap: true,
                    horizontal_direction,
                    vertical_direction,
                    horizontal_offset: offset,
                    vertical_offset: 0.0,
                }
            }
            Axis::Vertical => {
                let horizontal_direction = if phcenter < hcenter {
                    Direction::Positive
                } else {
                    Direction::Negative
                };
                let vertical_direction = pdy;
                Self {
                    horizontal_overlap: true,
                    vertical_overlap: false,
                    horizontal_direction,
                    vertical_direction,
                    horizontal_offset: 0.0,
                    vertical_offset: offset,
                }
            }
        }
    }
}

fn process_scroll_event(
    menu_state: &mut MenuState,
    prescroll_children_bounds: Rectangle,
    delta: mouse::ScrollDelta,
    scroll_speed: ScrollSpeed,
    viewport_size: Size,
) {
    use mouse::ScrollDelta;

    let pcb = prescroll_children_bounds;

    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } => y * scroll_speed.line,
        ScrollDelta::Pixels { y, .. } => y * scroll_speed.pixel,
    };

    let max_offset = (0.0 - pcb.y).max(0.0);
    let min_offset = (viewport_size.height - (pcb.y + pcb.height)).min(0.0);
    menu_state.scroll_offset = (menu_state.scroll_offset + delta_y).clamp(min_offset, max_offset);
}

#[derive(Debug, Clone, Copy)]
pub(super) struct MenuSlice {
    pub(super) start_index: usize,
    pub(super) end_index: usize,
    pub(super) lower_bound_rel: f32,
    pub(super) upper_bound_rel: f32,
}
impl MenuSlice {
    fn new(items_node: &Node, translation: Vector, viewport: Size, scroll_offset: f32) -> Self {
        let items_bounds = items_node.bounds() + translation;
        let max_index = items_node.children().len().saturating_sub(1);

        // viewport space absolute bounds
        let lower_bound = items_bounds.y.max(0.0);
        let upper_bound = (items_bounds.y + items_bounds.height).min(viewport.height);

        // menu space relative bounds
        let lower_bound_rel = lower_bound - (items_bounds.y + scroll_offset);
        let upper_bound_rel = upper_bound - (items_bounds.y + scroll_offset);

        // let start_index = search_bound_lin(lower_bound_rel, items_node.children(), 0);
        // let end_index = search_bound_lin(upper_bound_rel, items_node.children(), start_index);

        let nodes = items_node.children();
        let start_index = search_bound(0, max_index, lower_bound_rel, nodes);
        let end_index = search_bound(start_index, max_index, upper_bound_rel, nodes);

        Self {
            start_index,
            end_index,
            lower_bound_rel,
            upper_bound_rel,
        }
    }
}

/* fn search_bound_lin(
    bound: f32,
    nodes: &[Node],
    mut start_index: usize, // should be less than nodes.len()-1
) -> usize{
    for (i, n) in nodes.iter().enumerate().skip(start_index){
        let b = n.bounds();
        if !(bound > b.y + b.height){
            start_index = i;
            break;
        }
    }
    start_index
} */

fn search_bound(default_left: usize, default_right: usize, bound: f32, list: &[Node]) -> usize {
    // binary search
    let mut left = default_left;
    let mut right = default_right;

    while left != right {
        let m = ((left + right) / 2) + 1;
        if list[m].bounds().y > bound {
            right = m - 1;
        } else {
            left = m;
        }
    }
    left
}

fn clip_node_y(node: &Node, height: f32, offset: f32) -> Node {
    let node_bounds = node.bounds();
    Node::with_children(
        Size::new(node_bounds.width, height),
        node.children()
            .iter()
            .map(|n| n.clone().translate([0.0, -offset]))
            .collect(),
    )
    .move_to(node_bounds.position())
    .translate([0.0, offset])
}

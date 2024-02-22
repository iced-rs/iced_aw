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
use iced::{
    advanced::{
        layout::{Layout, Limits, Node},
        mouse, renderer,
        widget::tree::{self, Tree},
        Clipboard, Shell, },
    alignment, event, Element, Event, Length, Padding, Point, Rectangle,
    Size, Vector,
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
}
impl Default for MenuState {
    fn default() -> Self {
        Self {
            scroll_offset: 0.0,
            active: None,
            slice: MenuSlice {
                start_index: 0,
                end_index: usize::MAX,
                lower_bound_rel: 0.0,
                upper_bound_rel: f32::MAX,
            },
        }
    }
}

/// Menu
#[must_use]
pub struct Menu<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    pub(super) items: Vec<Item<'a, Message, Theme, Renderer>>,
    pub(super) spacing: f32,
    pub(super) padding: Padding,
    pub(super) max_width: f32,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) axis: Axis,
    pub(super) offset: f32,
}
impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    /// Creates a [`Menu`] with the given items.
    pub fn new(items: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            items,
            spacing: 0.0,
            padding: Padding::ZERO,
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
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the padding of the [`Menu`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// The offset from the bounds of the menu's parent item.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// rebuild state tree
    pub(super) fn tree(&self) -> Tree {
        Tree {
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
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
        tree.diff_children_custom(
            &self.items,
            |tree, item| item.diff(tree),
            Item::tree,
        );
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
            self.padding,
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
        let check_bounds = pad_rectangle(children_bounds, [check_bounds_width; 4].into());

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
            let factor = ((bounds.height - start_offset) / bounds.height).max(0.0);

            Node::with_children(
                Size::new(
                    items_node.bounds().width,
                    slice.upper_bound_rel - slice.lower_bound_rel,
                ),
                once(scale_node_y(node, factor).translate([0.0, start_offset])).collect(),
            )
        } else {
            let start_node = {
                let node = &items_node.children()[slice.start_index];
                let bounds = node.bounds();
                let start_offset = slice.lower_bound_rel - bounds.y;
                let factor = ((bounds.height - start_offset) / bounds.height).max(0.0);
                scale_node_y(node, factor).translate([0.0, start_offset])
            };

            let end_node = {
                let node = &items_node.children()[slice.end_index];
                let bounds = node.bounds();
                Node::with_children(
                    Size::new(bounds.width, slice.upper_bound_rel - bounds.y),
                    node.children().iter().map(Clone::clone).collect(),
                )
                .move_to(bounds.position())
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
    pub(super) fn on_event(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        use event::Status::*;

        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = &menu_state.slice;

        let status = self.items[slice.start_index..=slice.end_index] // [item...]
            .iter_mut()
            .zip(tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
            .zip(slice_layout.children()) // [item_layout...]
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
        
        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(prescroll) {
                    process_scroll_event(menu_state, prescroll, *delta, viewport.size());
                    Captured
                } else if cursor.is_over(offset_bounds) || cursor.is_over(check_bounds) {
                    Captured
                } else {
                    Ignored
                }
            }
            _ => Ignored,
        }
        .merge(status)
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    ///
    /// layout: Node{inf, \[ items_node, slice_node, prescroll, offset_bounds, check_bounds ]}
    pub(super) fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        theme_style: &Theme::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        // let offset_bounds = lc.next().unwrap().bounds();
        // let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_ref::<MenuState>();
        let slice = &menu_state.slice;

        let styling = theme.appearance(theme_style);

        // debug_draw(renderer, prescroll, check_bounds, offset_bounds);

        // draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds: pad_rectangle(prescroll, styling.menu_background_expand),
                border: styling.menu_border,
                shadow: styling.menu_shadow,
            },
            styling.menu_background,
        );

        let mut slc = slice_layout.children();

        // draw start
        let Some(start) = self.items.get(slice.start_index) else {
            return;
        };
        let Some(start_tree) = tree.children.get(slice.start_index) else {
            return;
        };
        let Some(start_layout) = slc.next() else {
            return;
        };

        if slice.end_index == slice.start_index {
            start.draw(
                start_tree,
                renderer,
                theme,
                style,
                start_layout,
                cursor,
                viewport,
            );
        } else {
            let start_bounds = start_layout.bounds();
            renderer.with_layer(start_bounds, |r| {
                start.draw(start_tree, r, theme, style, start_layout, cursor, viewport);
            });

            // draw the rest
            let Some(items) = self
                .items
                .get(slice.start_index + 1..slice.end_index.saturating_add(1))
            else {
                return;
            };

            let Some(trees) = tree
                .children
                .get(slice.start_index + 1..slice.end_index.saturating_add(1))
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
    }

    pub(super) fn open_event(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) -> event::Status {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        // let prescroll = lc.next().unwrap().bounds();
        // let offset_bounds = lc.next().unwrap().bounds();
        // let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = &menu_state.slice;
        menu_state.active = None;

        for (i, (item, layout)) in self.items[slice.start_index..=slice.end_index]
            .iter()
            .zip(slice_layout.children())
            .enumerate()
        {
            if item.menu.is_some() && cursor.is_over(layout.bounds()) {
                menu_state.active = Some(i + slice.start_index);
                return event::Status::Captured;
            }
        }
        event::Status::Ignored
    }

    pub(super) fn close_event(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        parent_bounds: Rectangle,
        prev_bounds_list: &[Rectangle],
        prev: &mut Index,
    ) {
        let mut lc = layout.children();
        let _slice_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

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
            *prev = None;
            let menu_state = tree.state.downcast_mut::<MenuState>();
            menu_state.scroll_offset = 0.0;
            menu_state.active = None;
        }
    }
}

/* fn debug_draw<Renderer: renderer::Renderer>(
    renderer: &mut Renderer,
    prescroll: Rectangle,
    check_bounds: Rectangle,
    offset_bounds: Rectangle,
){
    [
        prescroll,
        check_bounds,
        offset_bounds,
    ].iter()
    .zip([
        Color::from([1.0, 1.0, 1.0, 0.8]),
        Color::from([1.0, 0.0, 0.0, 0.1]),
        Color::from([0.0, 0.0, 1.0, 0.3]),
    ])
    .for_each(|(b, c)|{
        renderer.fill_quad(
            renderer::Quad{
                bounds: *b,
                border: Border{
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            c
        );
    });
} */

/// Item inside a [`Menu`]
#[must_use]
pub struct Item<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    pub(super) item: Element<'a, Message, Theme, Renderer>,
    pub(super) menu: Option<Box<Menu<'a, Message, Theme, Renderer>>>,
}
impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
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

    /// rebuild state tree
    pub(super) fn tree(&self) -> Tree {
        Tree {
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
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
        self.menu
            .as_ref()
            .map_or([Tree::new(&self.item)].into(), |m| {
                [Tree::new(&self.item), m.tree()].into()
            })
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
    /* pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node {
        self.item
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    } */

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.item.as_widget_mut().on_event(
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
    viewport_size: Size,
) {
    use mouse::ScrollDelta;

    let pcb = prescroll_children_bounds;

    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => y,
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

fn scale_node_y(node: &Node, factor: f32) -> Node {
    let node_bounds = node.bounds();
    Node::with_children(
        Size::new(node_bounds.width, node_bounds.height * factor),
        node.children()
            .iter()
            .map(|n| {
                let n_bounds = n.bounds();
                scale_node_y(n, factor).move_to(Point::new(n_bounds.x, n_bounds.y * factor))
            })
            .collect(),
    )
    .move_to(node_bounds.position())
}

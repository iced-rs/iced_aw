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
use iced_core::{
    alignment,
    layout::{Layout, Limits, Node},
    mouse, renderer,
    time::Instant,
    widget::tree::{self, Tree},
    widget::Operation,
    window, Clipboard, Element, Event, Length, Padding, Pixels, Point, Rectangle, Shell, Size,
    Vector,
};
use std::iter::once;

use super::menu_bar::*;
use crate::style::menu_bar::*;

#[cfg(feature = "debug_log")]
use log::{debug, warn};

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
    pub(super) scroll_offset: f32,
    pub(super) active: Index,
    pub(super) slice: MenuSlice,
}
impl MenuState {
    /// item_tree: Tree{item state, [Tree{widget state}, Tree{menu state, [...]}]}
    pub(super) fn open_new_menu<'a, Message, Theme: Catalog, Renderer: renderer::Renderer>(
        &mut self,
        active_index: usize,
        item: &Item<'a, Message, Theme, Renderer>,
        item_tree: &mut Tree,
    ) {
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuState::open_new_menu", "");
        let Some(menu) = item.menu.as_ref() else {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuState::open_new_menu", "item.menu is None");
            return;
        };

        self.active = Some(active_index);

        // build the state tree for the new menu
        let menu_tree = menu.tree();

        #[cfg(feature = "debug_log")]
        {
            if item_tree.children.len() > 2 {
                warn!(
                    target:"menu::MenuState::open_new_menu",
                    "item_tree.children.len() > 2 | len: {:?}",
                    item_tree.children.len()
                );
            }
        }

        if item_tree.children.len() == 1 {
            item_tree.children.push(menu_tree);
        } else {
            item_tree.children[1] = menu_tree;
        }
    }
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
    pub(super) padding: Padding,
    pub(super) close_on_item_click: Option<bool>,
    pub(super) close_on_background_click: Option<bool>,
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
            padding: Padding::new(5.0),
            close_on_item_click: None,
            close_on_background_click: None,
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

    /// Sets the padding of the [`Menu`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the close on item click option of the [`Menu`].
    pub fn close_on_item_click(mut self, value: bool) -> Self {
        self.close_on_item_click = Some(value);
        self
    }

    /// Sets the close on background click option of the [`Menu`].
    pub fn close_on_background_click(mut self, value: bool) -> Self {
        self.close_on_background_click = Some(value);
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
    /// out: Node{inf, \[ slice_node, items_bounds, offset_bounds]}
    pub(super) fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
        parent_bounds: Rectangle,
        parent_direction: (Direction, Direction),
        viewport: &Rectangle,
    ) -> (Node, (Direction, Direction)) {
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::Menu::layout", "");
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
            &mut self
                .items
                .iter_mut()
                .map(|i| &mut i.item)
                .collect::<Vec<_>>(),
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

        let menu_state = tree.state.downcast_mut::<MenuState>();

        // calc slice
        let (lower_bound_rel, upper_bound_rel) = cal_bounds_rel_menu(
            &items_node,
            children_position - Point::ORIGIN,
            viewport.size(),
            menu_state.scroll_offset,
        );
        let slice =
            MenuSlice::from_bounds_rel(lower_bound_rel, upper_bound_rel, &items_node, |n| {
                n.bounds().y
            });
        menu_state.slice = slice;
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::Menu::layout", "slice: {:?}", slice);

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
                Size::INFINITE,
                [
                    slice_node
                        .move_to(children_position)
                        .translate([0.0, menu_state.scroll_offset]), // slice_layout
                    Node::new(children_size).move_to(children_position), // items_bounds
                    Node::new(offset_bounds.size()).move_to(offset_bounds.position()), // offset_bounds
                ]
                .into(),
            ),
            child_direction,
        )
    }

    /// tree: Tree{ menu_state, \[item_tree...] }
    ///
    /// layout: Node{inf, \[ slice_node, items_bounds, offset_bounds]}
    pub(super) fn update(
        &mut self,
        global_state: &mut GlobalState,
        global_parameters: &GlobalParameters<'_, Theme>,
        rec_event: RecEvent,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
        parent_bounds: Rectangle,
        prev_bounds_list: &[Rectangle],
        prev_active: &mut Index,
    ) -> RecEvent {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let items_bounds = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let background_bounds = pad_rectangle(items_bounds, self.padding);
        let safe_bounds = pad_rectangle(
            background_bounds,
            Padding::new(global_parameters.safe_bounds_margin),
        );

        enum Op {
            UpdateItems,
            OpenEvent,
            LeftPress,
            ScrollEvent,
            RedrawUpdate,
        }

        let mut run_op = |global_state: &mut GlobalState, tree: &mut Tree, op: &Op| {
            let Tree {
                state,
                children: item_trees,
                ..
            } = tree;
            let menu_state = state.downcast_mut::<MenuState>();

            match op {
                Op::UpdateItems => {
                    itl_iter_slice!(
                        menu_state.slice,
                        self.items;iter_mut,
                        item_trees;iter_mut,
                        slice_layout.children()
                    )
                    .for_each(|((item, tree), layout)| {
                        item.update(
                            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
                        );
                    });
                }
                Op::RedrawUpdate => {
                    let cursor = if let Some(active) = menu_state.active {
                        match &global_parameters.draw_path {
                            DrawPath::FakeHovering => {
                                let active_in_slice = active - menu_state.slice.start_index;
                                let center = slice_layout
                                    .children()
                                    .nth(active_in_slice)
                                    .expect(&format!(
                                        " Index {:?} (in slice space) is not within the slice layout \
                                        | slice_layout.children().count(): {:?} \
                                        | This should not happen, please report this issue
                                        ",
                                        active_in_slice,
                                        slice_layout.children().count()
                                    ))
                                    .bounds()
                                    .center();
                                mouse::Cursor::Available(center)
                            }
                            DrawPath::Backdrop => mouse::Cursor::Unavailable,
                        }
                    } else {
                        cursor
                    };

                    let mut temp_messages = vec![];
                    let mut temp_shell = Shell::new(&mut temp_messages);

                    let redraw_event =
                        Event::Window(window::Event::RedrawRequested(Instant::now()));

                    itl_iter_slice!(
                        menu_state.slice,
                        self.items;iter_mut,
                        item_trees;iter_mut,
                        slice_layout.children()
                    )
                    .for_each(|((item, tree), layout)| {
                        item.update(
                            tree,
                            &redraw_event,
                            layout,
                            cursor,
                            renderer,
                            clipboard,
                            &mut temp_shell,
                            viewport,
                        );
                    });
                    shell.merge(temp_shell, |message| message);
                }
                Op::LeftPress => match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        schedule_close_on_click(
                            global_state,
                            global_parameters,
                            menu_state.slice,
                            &mut self.items,
                            slice_layout.children(),
                            cursor,
                            self.close_on_item_click,
                            self.close_on_background_click,
                        );
                    }
                    _ => {}
                },
                Op::ScrollEvent => match event {
                    Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                        if cursor.is_over(background_bounds) {
                            let delta_y = match delta {
                                mouse::ScrollDelta::Lines { y, .. } => {
                                    y * global_parameters.scroll_speed.line
                                }
                                mouse::ScrollDelta::Pixels { y, .. } => {
                                    y * global_parameters.scroll_speed.pixel
                                }
                            };

                            let max_offset = (0.0 - items_bounds.y).max(0.0);
                            let min_offset = (viewport.size().height
                                - (items_bounds.y + items_bounds.height))
                                .min(0.0);
                            menu_state.scroll_offset =
                                (menu_state.scroll_offset + delta_y).clamp(min_offset, max_offset);
                        }
                        shell.request_redraw();
                    }
                    _ => {}
                },
                Op::OpenEvent => {
                    if !global_state.pressed {
                        assert!(
                            menu_state.active.is_none(),
                            "
                        Menu::open_event() is called only when RecEvent::Close is returned, \
                        which means no child menu should be open (menu_state.active must be None). \
                        If this assert fails, please report this issue.
                    "
                        );

                        try_open_menu(
                            &mut self.items,
                            menu_state,
                            item_trees,
                            slice_layout.children(),
                            cursor,
                            shell,
                        );
                    }
                }
            }
        };

        let mut update = |global_state: &mut GlobalState, tree: &mut Tree, ops: &[Op]| {
            for op in ops.iter() {
                run_op(global_state, tree, op);
            }
        };

        match rec_event {
            RecEvent::Event => {
                // menu not in focus
                update(global_state, tree, &[Op::RedrawUpdate]);
                shell.capture_event();
                RecEvent::Event
            }
            RecEvent::Close => {
                if cursor.is_over(background_bounds) || cursor.is_over(offset_bounds) {
                    // menu in focus
                    update(
                        global_state,
                        tree,
                        &[
                            Op::UpdateItems,
                            Op::LeftPress,
                            Op::ScrollEvent,
                            Op::OpenEvent,
                        ],
                    );
                    shell.capture_event();
                    RecEvent::Event
                } else if cursor.is_over(parent_bounds) {
                    // menu not in focus
                    update(global_state, tree, &[Op::RedrawUpdate]);
                    // the cursor is over the parent bounds
                    // let the parent process the event
                    assert!(
                        shell.is_event_captured() == false,
                        "Returning RecEvent::None"
                    );
                    RecEvent::None
                } else {
                    let open = {
                        if global_state.pressed {
                            true
                        } else {
                            if prev_bounds_list.iter().any(|r| cursor.is_over(*r)) {
                                false
                            } else {
                                cursor.is_over(safe_bounds)
                            }
                        }
                    };

                    if open {
                        // the current menu is not ready to close
                        update(global_state, tree, &[Op::UpdateItems, Op::LeftPress]);
                        shell.capture_event();
                        RecEvent::Event
                    } else {
                        // the current menu is ready to close
                        #[cfg(feature = "debug_log")]
                        debug!(target:"menu::Menu::update", "close menu");
                        assert!(
                            shell.is_event_captured() == false,
                            "Returning RecEvent::Close"
                        );
                        *prev_active = None;
                        if tree.children.len() == 2 {
                            // prune the menu tree when the menu is closed
                            let _ = tree.children.pop();
                        }
                        shell.invalidate_layout();
                        shell.request_redraw();
                        RecEvent::Close
                    }
                }
            }
            RecEvent::None => {
                update(
                    global_state,
                    tree,
                    &[Op::UpdateItems, Op::LeftPress, Op::ScrollEvent],
                );
                shell.capture_event();
                RecEvent::Event
            }
        }
    }

    pub(super) fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        let slice = menu_state.slice;

        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            itl_iter_slice!(slice, self.items;iter_mut, tree.children;iter_mut, slice_layout.children())
                .for_each(|((child, state), layout)| {
                    child.operate(state, layout, renderer, operation);
                });
        });
    }

    /// tree: Tree{ menu_state, \[item_tree...] }
    ///
    /// layout: Node{inf, \[ slice_node, items_bounds, offset_bounds]}
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
        let slice = menu_state.slice;

        itl_iter_slice!(slice, self.items;iter, tree.children;iter, slice_layout.children())
            .map(|((item, tree), layout)| item.mouse_interaction(tree, layout, cursor, renderer))
            .max()
            .unwrap_or_default()
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    ///
    /// layout: Node{inf, \[ items_node, slice_node, items_bounds, offset_bounds]}
    pub(super) fn draw(
        &self,
        draw_path: &DrawPath,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        theme_style: &Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut lc = layout.children();
        let slice_layout = lc.next().unwrap();
        let items_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_ref::<MenuState>();
        let slice = menu_state.slice;

        // draw background
        let pad_rectangle = pad_rectangle(items_bounds, self.padding);
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

        if let (DrawPath::Backdrop, Some(active)) = (draw_path, menu_state.active) {
            let active_in_slice = active - menu_state.slice.start_index;
            let active_bounds = slice_layout
                .children()
                .nth(active_in_slice)
                .expect(&format!(
                    "Index {:?} (in slice space) is not within the slice layout \
                    | slice_layout.children().count(): {:?} \
                    | This should not happen, please report this issue
                    ",
                    active_in_slice,
                    slice_layout.children().count()
                ))
                .bounds();

            renderer.fill_quad(
                renderer::Quad {
                    bounds: active_bounds,
                    border: theme_style.path_border,
                    ..Default::default()
                },
                theme_style.path,
            );
        }

        renderer.with_layer(items_bounds, |r| {
            itl_iter_slice!(slice, self.items;iter, tree.children;iter, slice_layout.children())
                .for_each(|((item, tree), layout)| {
                    item.draw(tree, r, theme, style, layout, cursor, viewport);
                });
        });
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
    pub(super) close_on_click: Option<bool>,
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
            close_on_click: None,
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
            close_on_click: None,
        }
    }

    /// Sets the close on click option of the [`Item`].
    pub fn close_on_click(mut self, close_on_click: bool) -> Self {
        self.close_on_click = Some(close_on_click);
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
        // self.menu.as_ref().map_or_else(
        //     || [Tree::new(&self.item)].into(),
        //     |m| [Tree::new(&self.item), m.tree()].into(),
        // )
        vec![Tree::new(&self.item)]
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
        #[cfg(feature = "debug_log")]
        debug!(target:"Item::update", "");
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
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.item
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        Imagine there are two sticks, parent and child
        parent: o-----o
        child:  o----------o

        Now we align the child to the parent in one dimension
        There are 4 possibilities:

        1. to the right
                    o-----oo----------o

        2. to the right with overlapping
                    o-----o
                    o----------o

        3. to the left
        o----------oo-----o

        4. to the left with overlapping
                    o-----o
               o----------o

        The child goes to the default direction by default,
        if the space on the default direction runs out it goes to the other,
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

fn cal_bounds_rel_menu(
    items_node: &Node,
    translation: Vector,
    viewport: Size,
    scroll_offset: f32,
) -> (f32, f32) {
    let items_bounds = items_node.bounds() + translation; // viewport space

    // viewport space absolute bounds
    let lower_bound = items_bounds.y.max(0.0);
    let upper_bound = (items_bounds.y + items_bounds.height).min(viewport.height);

    // menu space relative bounds
    let lower_bound_rel = lower_bound - (items_bounds.y + scroll_offset);
    let upper_bound_rel = upper_bound - (items_bounds.y + scroll_offset);

    (lower_bound_rel, upper_bound_rel)
}

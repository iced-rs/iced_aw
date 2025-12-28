//! [`MenuBar`]

#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use iced_core::{
    Clipboard, Element, Event, Layout, Length, Padding, Pixels, Rectangle, Shell, Size, Widget,
    alignment, event,
    layout::{Limits, Node},
    mouse, overlay, renderer,
    widget::{Operation, Tree, tree},
    window,
};

use super::{common::*, flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*};
use crate::style::menu_bar::*;
pub use crate::style::status::{Status, StyleFn};

#[cfg(feature = "debug_log")]
use log::debug;

#[derive(Debug, Clone, Copy)]
pub(super) enum MenuBarTask {
    OpenOnClick,
    CloseOnClick,
}

#[derive(Default, Debug)]
pub(super) struct GlobalState {
    pub(super) open: bool,
    pub(super) pressed: bool,
    task: Option<MenuBarTask>,
}
impl GlobalState {
    pub(super) fn schedule(&mut self, task: MenuBarTask) {
        self.task = Some(task);
    }

    pub(super) fn task(&self) -> Option<MenuBarTask> {
        self.task
    }

    pub(super) fn clear_task(&mut self) {
        self.task = None;
    }
}

#[derive(Default, Debug)]
pub(super) struct MenuBarState {
    pub(super) global_state: GlobalState,
    pub(super) menu_state: MenuState,
}
impl MenuBarState {
    pub(super) fn open<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
        &mut self,
        roots: &mut [Item<'a, Message, Theme, Renderer>],
        item_trees: &mut [Tree],
        item_layouts: impl Iterator<Item = Layout<'b>>,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) {
        if !self.global_state.open {
            self.global_state.open = true;
            self.menu_state.active = None;
        }

        try_open_menu(
            roots,
            &mut self.menu_state,
            item_trees,
            item_layouts,
            cursor,
            shell,
        );

        self.global_state.task = None;
    }

    pub(super) fn close<Message>(
        &mut self,
        item_trees: &mut [Tree],
        shell: &mut Shell<'_, Message>,
    ) {
        if self.global_state.pressed {
            return;
        }

        for item_tree in item_trees.iter_mut() {
            if item_tree.children.len() == 2 {
                let _ = item_tree.children.pop();
                shell.invalidate_layout();
            }
        }
        self.global_state.pressed = false;
        self.global_state.task = None;
        self.global_state.open = false;
        self.menu_state.active = None;
        shell.request_redraw();
    }
}

/// menu bar
#[must_use]
pub struct MenuBar<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) roots: Vec<Item<'a, Message, Theme, Renderer>>,
    spacing: Pixels,
    padding: Padding,
    width: Length,
    height: Length,
    close_on_item_click: Option<bool>,
    close_on_background_click: Option<bool>,
    pub(super) global_parameters: GlobalParameters<'a, Theme>,
}
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
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
            spacing: Pixels::ZERO,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            close_on_item_click: None,
            close_on_background_click: None,
            global_parameters: GlobalParameters {
                safe_bounds_margin: 50.0,
                draw_path: DrawPath::FakeHovering,
                scroll_speed: ScrollSpeed {
                    line: 60.0,
                    pixel: 1.0,
                },
                close_on_item_click: false,
                close_on_background_click: false,
                class: Theme::default(),
            },
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
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets the margin of the safe bounds of the [`Menu`]s in the [`MenuBar`].
    ///
    /// Defines a rectangular safe area that extends each menu's background bounds by a margin.
    /// If the cursor moves outside this area, the menu will be closed.
    pub fn safe_bounds_margin(mut self, margin: f32) -> Self {
        self.global_parameters.safe_bounds_margin = margin;
        self
    }

    /// Sets the draw path option of the [`MenuBar`]
    pub fn draw_path(mut self, draw_path: DrawPath) -> Self {
        self.global_parameters.draw_path = draw_path;
        self
    }

    /// Sets the scroll speed of the [`Menu`]s in the [`MenuBar`]
    pub fn scroll_speed(mut self, scroll_speed: ScrollSpeed) -> Self {
        self.global_parameters.scroll_speed = scroll_speed;
        self
    }

    /// Sets the close on item click option of the [`MenuBar`]
    pub fn close_on_item_click(mut self, value: bool) -> Self {
        self.close_on_item_click = Some(value);
        self
    }

    /// Sets the close on background click option of the [`MenuBar`]
    pub fn close_on_background_click(mut self, value: bool) -> Self {
        self.close_on_background_click = Some(value);
        self
    }

    /// Sets the global default close on item click option
    pub fn close_on_item_click_global(mut self, value: bool) -> Self {
        self.global_parameters.close_on_item_click = value;
        self
    }

    /// Sets the global default close on background click option
    pub fn close_on_background_click_global(mut self, value: bool) -> Self {
        self.global_parameters.close_on_background_click = value;
        self
    }

    /// Sets the padding of the [`MenuBar`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the style of the [`MenuBar`].
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.global_parameters.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`MenuBar`].
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.global_parameters.class = class.into();
        self
    }
}
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MenuBar<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
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
        self.roots.iter().map(Item::tree).collect::<Vec<_>>()
    }

    /// tree: Tree{bar, \[item_tree...]}
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(&self.roots, |tree, item| item.diff(tree), Item::tree);
    }

    /// tree: Tree{bar, \[item_tree...]}
    ///
    /// out: Node{bar bounds , \[widget_layout, widget_layout, ...]}
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        // TODO: unify layout code with Menu

        let MenuBarState {
            menu_state: bar_menu_state,
            ..
        } = tree.state.downcast_mut::<MenuBarState>();

        let items_node = flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            &Limits::new(
                Size {
                    width: 0.0,
                    height: limits.min().height,
                },
                Size {
                    width: f32::INFINITY,
                    height: limits.max().height,
                },
            ),
            Length::Shrink,
            // self.width,
            self.height,
            self.padding,
            self.spacing,
            alignment::Alignment::Center,
            &mut self
                .roots
                .iter_mut()
                .map(|item| &mut item.item)
                .collect::<Vec<_>>(),
            &mut tree
                .children
                .iter_mut()
                .map(|tree| &mut tree.children[0])
                .collect::<Vec<_>>(),
        );

        let items_node_bounds = items_node.bounds();
        #[cfg(feature = "debug_log")]
        debug!("menu::MenuBar::layout | items_node_bounds: {items_node_bounds:?}");

        let resolved_width = match self.width {
            Length::Fill | Length::FillPortion(_) => items_node_bounds
                .width
                .min(limits.max().width)
                .max(limits.min().width),
            Length::Fixed(amount) => amount.min(limits.max().width).max(limits.min().width),
            Length::Shrink => items_node_bounds.width,
        };

        let lower_bound_rel = self.padding.left - bar_menu_state.scroll_offset;
        let upper_bound_rel = lower_bound_rel + resolved_width - self.padding.x();

        let slice =
            MenuSlice::from_bounds_rel(lower_bound_rel, upper_bound_rel, &items_node, |n| {
                n.bounds().x
            });
        #[cfg(feature = "debug_log")]
        debug!("menu::MenuBar::layout | slice: {slice:?}");

        bar_menu_state.slice = slice;

        let slice_node = if slice.start_index == slice.end_index {
            let node = &items_node.children()[slice.start_index];
            let bounds = node.bounds();
            let start_offset = slice.lower_bound_rel - bounds.x;
            let width = slice.upper_bound_rel - slice.lower_bound_rel;

            Node::with_children(
                Size::new(width, items_node.bounds().height),
                std::iter::once(clip_node_x(node, width, start_offset)).collect(),
            )
        } else {
            let start_node = {
                let node = &items_node.children()[slice.start_index];
                let bounds = node.bounds();
                let start_offset = slice.lower_bound_rel - bounds.x;
                let width = bounds.width - start_offset;
                clip_node_x(node, width, start_offset)
            };

            let end_node = {
                let node = &items_node.children()[slice.end_index];
                let bounds = node.bounds();
                let width = slice.upper_bound_rel - bounds.x;
                clip_node_x(node, width, 0.0)
            };

            Node::with_children(
                items_node_bounds.size(),
                std::iter::once(start_node)
                    .chain(
                        items_node.children()[slice.start_index + 1..slice.end_index]
                            .iter()
                            .map(Clone::clone),
                    )
                    .chain(std::iter::once(end_node))
                    .collect(),
            )
        };

        Node::with_children(
            Size {
                width: resolved_width,
                height: items_node_bounds.height,
            },
            [
                // items_node
                slice_node.translate([bar_menu_state.scroll_offset, 0.0]),
            ]
            .into(),
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBar::update", "");

        let slice_layout = layout.children().next().unwrap();

        let Tree {
            state,
            children: item_trees,
            ..
        } = tree;
        let bar = state.downcast_mut::<MenuBarState>();
        let MenuBarState {
            global_state,
            menu_state: bar_menu_state,
        } = bar;

        let slice = bar_menu_state.slice;
        itl_iter_slice!(
            slice,
            self.roots;iter_mut,
            item_trees;iter_mut,
            slice_layout.children()
        )
        .for_each(|((item, tree), layout)| {
            item.update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        });

        let bar_bounds = layout.bounds();
        // println!("bar_bounds: {:?}", bar_bounds);
        // println!("cursor: {:?}", cursor);
        // println!("cursor in bar_bounds: {:?}", cursor.is_over(bar_bounds));

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(bar_bounds) {
                    global_state.pressed = true;
                    if global_state.open {
                        schedule_close_on_click(
                            global_state,
                            &self.global_parameters,
                            slice,
                            &mut self.roots,
                            slice_layout.children(),
                            cursor,
                            self.close_on_item_click,
                            self.close_on_background_click,
                        );
                    } else {
                        global_state.schedule(MenuBarTask::OpenOnClick);
                    }
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                global_state.pressed = false;

                if let Some(task) = global_state.task {
                    match task {
                        MenuBarTask::OpenOnClick => {
                            bar.open(
                                &mut self.roots,
                                item_trees,
                                slice_layout.children(),
                                cursor,
                                shell,
                            );
                        }
                        MenuBarTask::CloseOnClick => {
                            bar.close(item_trees, shell);
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if global_state.open {
                    if cursor.is_over(bar_bounds) {
                        try_open_menu(
                            &mut self.roots,
                            bar_menu_state,
                            item_trees,
                            slice_layout.children(),
                            cursor,
                            shell,
                        );
                        shell.capture_event();
                    } else {
                        bar.close(item_trees, shell);
                    }
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(bar_bounds) && slice_layout.bounds().width > layout.bounds().width
                // check if scrolling is on
                {
                    let scroll_speed = self.global_parameters.scroll_speed;
                    let delta_x = match delta {
                        mouse::ScrollDelta::Lines { x, .. } => x * scroll_speed.line,
                        mouse::ScrollDelta::Pixels { x, .. } => x * scroll_speed.pixel,
                    };

                    let min_offset = -(slice_layout.bounds().width - layout.bounds().width);

                    bar_menu_state.scroll_offset =
                        (bar_menu_state.scroll_offset + delta_x).clamp(min_offset, 0.0);
                    shell.invalidate_layout();
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Window(window::Event::Resized { .. }) => {
                if slice_layout.bounds().width > layout.bounds().width {
                    let min_offset = -(slice_layout.bounds().width - layout.bounds().width);

                    bar_menu_state.scroll_offset =
                        bar_menu_state.scroll_offset.clamp(min_offset, 0.0);
                }
                shell.invalidate_layout();
                shell.request_redraw();
            }
            _ => {}
        }

        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBar::update", "return | bar: {bar:?}");
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let slice_layout = layout.children().next().unwrap();

        let MenuBarState {
            menu_state: bar_menu_state,
            ..
        } = tree.state.downcast_ref::<MenuBarState>();

        let slice = bar_menu_state.slice;

        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            itl_iter_slice!(slice, self.roots;iter_mut, tree.children;iter_mut, slice_layout.children())
                .for_each(|((child, state), layout)| {
                    child.operate(state, layout, renderer, operation);
                });
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let slice_layout = layout.children().next().unwrap();

        let MenuBarState {
            menu_state: bar_menu_state,
            ..
        } = tree.state.downcast_ref::<MenuBarState>();

        itl_iter_slice!(bar_menu_state.slice, self.roots;iter, tree.children;iter, slice_layout.children())
            .map(|((item, tree), layout)| item.mouse_interaction(tree, layout, cursor, renderer))
            .max()
            .unwrap_or_default()
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
        let slice_layout = layout.children().next().unwrap();

        let MenuBarState {
            global_state,
            menu_state: bar_menu_state,
        } = tree.state.downcast_ref::<MenuBarState>();

        let slice = bar_menu_state.slice;

        let styling = theme.style(&self.global_parameters.class, Status::Active);
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: styling.bar_border,
                shadow: styling.bar_shadow,
                ..Default::default()
            },
            styling.bar_background,
        );

        if let (DrawPath::Backdrop, true, Some(active)) = (
            &self.global_parameters.draw_path,
            global_state.open,
            bar_menu_state.active,
        ) {
            let active_in_slice = active - slice.start_index;
            let active_bounds = slice_layout
                .children()
                .nth(active_in_slice)
                .expect(
                    "Index (in slice space) is not within the menu bar layout. \
                    This should not happen, please report this issue",
                )
                .bounds();

            renderer.fill_quad(
                renderer::Quad {
                    bounds: active_bounds,
                    border: styling.path_border,
                    ..Default::default()
                },
                styling.path,
            );
        }

        renderer.with_layer(
            Rectangle {
                x: layout.bounds().x + self.padding.left,
                y: layout.bounds().y + self.padding.top,
                width: layout.bounds().width - self.padding.x(),
                height: layout.bounds().height - self.padding.y(),
            },
            |r| {
                itl_iter_slice!(slice, self.roots;iter, tree.children;iter, slice_layout.children())
                .for_each(|((item, tree), layout)| {
                    item.draw(tree, r, theme, style, layout, cursor, viewport);
                });
            },
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced_core::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBar::overlay", "");
        let bar = tree.state.downcast_mut::<MenuBarState>();

        if bar.global_state.open {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBar::overlay", "return | Menu Overlay");
            Some(
                MenuBarOverlay {
                    menu_bar: self,
                    layout,
                    translation,
                    tree,
                }
                .overlay_element(),
            )
        } else {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBar::overlay", "state not open | try return root overlays");
            let slice_layout = layout.children().next()?;

            let Tree {
                state,
                children: item_trees,
                ..
            } = tree;
            let bar = state.downcast_mut::<MenuBarState>();
            let MenuBarState {
                menu_state: bar_menu_state,
                ..
            } = bar;

            let slice = bar_menu_state.slice;

            let overlays = itl_iter_slice!(slice, self.roots;iter_mut, item_trees;iter_mut, slice_layout.children())
                .filter_map(|((item, item_tree), item_layout)| {
                    item.item.as_widget_mut().overlay(
                        &mut item_tree.children[0],
                        item_layout,
                        renderer,
                        viewport,
                        translation,
                    )
                })
                .collect::<Vec<_>>();

            if overlays.is_empty() {
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBar::overlay", "return | None");
                None
            } else {
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBar::overlay", "return | Root Item Overlay");
                Some(overlay::Group::with_children(overlays).overlay())
            }
        }
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(value: MenuBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

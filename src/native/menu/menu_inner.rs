//! Menu tree overlay

use super::menu_bar::MenuBarState;
use super::menu_tree::MenuTree;
use crate::style::menu_bar::StyleSheet;
use iced_native::widget::Tree;
use iced_native::{
    event, layout, mouse, overlay, renderer, touch, Clipboard, Color, Padding, Point, Rectangle,
    Shell, Size, Vector,
};

/// The condition of when to close a menu
#[derive(Debug, Clone, Copy)]
pub struct CloseCondition {
    /// Close menus when the cursor moves outside the check bounds
    pub leave: bool,

    /// Close menus when the cursor clicks outside the check bounds
    pub click_outside: bool,

    /// Close menus when the cursor clicks inside the check bounds
    pub click_inside: bool,
}

/// The width of an item
#[derive(Debug, Clone, Copy)]
pub enum ItemWidth {
    /// Use uniform width
    Uniform(u16),
    /// Static tries to use the width value of each menu(menu tree with children),
    /// the widths of items(menu tree with empty children) will be the same as the menu they're in,
    /// if that value is None,
    /// the default value will be used instead,
    /// which is the value of the Static variant
    Static(u16),
}

/// The height of an item
#[derive(Debug, Clone, Copy)]
pub enum ItemHeight {
    /// Use uniform height
    Uniform(u16),
    /// Static tries to use the height value of each menu tree,
    /// if that value is None,
    /// the default value will be used instead,
    /// which is the value of the Static variant
    Static(u16),
    // TODO: Flex,
}

/// Methods for drawing path highlight
#[derive(Debug, Clone, Copy)]
pub enum PathHighlight {
    /// Draw the full path,
    Full,
    /// Omit the active item(the last item in the path)
    OmitActive,
    /// Omit the active item if it's not a menu
    MenuActive,
}

/// X+ goes right and Y+ goes down
#[derive(Debug, Clone, Copy)]
pub(super) enum Direction {
    Positive,
    Negative,
}

/// Adaptive open direction
#[derive(Debug)]
struct Aod {
    // whether or not to use aod
    horizontal: bool,
    vertical: bool,

    // whether or not to use overlap
    horizontal_overlap: bool,
    vertical_overlap: bool,

    // default direction
    horizontal_direction: Direction,
    vertical_direction: Direction,
}
impl Aod {
    fn adaptive(
        parent_pos: f32,
        parent_size: f32,
        child_size: f32,
        max_size: f32,
        on: bool,
        overlap: bool,
        direction: Direction,
    ) -> f32 {
        /*
        Imagine there're two sticks, parent and child
        parent: o-----o
        child:  o----------o

        Now we align the child to the parent in one dimension
        There are 4 possibilities:

        1. to the right
                    o-----oo----------o

        2. to the right but allow overlaping
                    o-----o
                    o----------o

        3. to the left
        o----------oo-----o

        4. to the left but allow overlaping
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
                    if on && space_negative > space_positive && overshoot > space_positive {
                        parent_pos - overshoot
                    } else {
                        parent_pos
                    }
                } else {
                    let overshoot = child_size;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        parent_pos - overshoot
                    } else {
                        parent_pos + parent_size
                    }
                }
            }
            Direction::Negative => {
                let space_positive = parent_pos;
                let space_negative = max_size - parent_pos - parent_size;

                if overlap {
                    let overshoot = child_size - parent_size;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        parent_pos
                    } else {
                        parent_pos - overshoot
                    }
                } else {
                    let overshoot = child_size;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        parent_pos + parent_size
                    } else {
                        parent_pos - overshoot
                    }
                }
            }
        }
    }

    fn point(&self, parent_bounds: Rectangle, children_size: Size, viewport_size: Size) -> Point {
        let x = Self::adaptive(
            parent_bounds.x,
            parent_bounds.width,
            children_size.width,
            viewport_size.width,
            self.horizontal,
            self.horizontal_overlap,
            self.horizontal_direction,
        );
        let y = Self::adaptive(
            parent_bounds.y,
            parent_bounds.height,
            children_size.height,
            viewport_size.height,
            self.vertical,
            self.vertical_overlap,
            self.vertical_direction,
        );

        [x, y].into()
    }
}

/// A part of a menu where items are displayed.
///
/// When the bounds of a menu exceed the viewport,
/// only items inside the viewport will be displayed,
/// when scrolling happens, this should be updated
#[derive(Debug, Clone, Copy)]
struct MenuSlice {
    start_index: usize,
    end_index: usize,
    lower_bound_rel: f32,
    upper_bound_rel: f32,
}

/// Menu bounds in overlay space
struct MenuBounds {
    child_positions: Vec<f32>,
    children_bounds: Rectangle,
    parent_bounds: Rectangle,
    check_bounds: Rectangle,
}
impl MenuBounds {
    fn new<Message, Renderer>(
        menu_tree: &MenuTree<'_, Message, Renderer>,
        item_width: ItemWidth,
        item_height: ItemHeight,
        viewport_size: Size,
        overlay_offset: Vector,
        aod: &Aod,
        bounds_expand: u16,
        parent_bounds: Rectangle,
    ) -> Self
    where
        Renderer: renderer::Renderer,
    {
        let children_size = get_children_size(menu_tree, item_width, item_height);

        // viewport space parent bounds
        let view_parent_bounds = parent_bounds + overlay_offset;

        // overlay space children position
        let children_position =
            aod.point(view_parent_bounds, children_size, viewport_size) - overlay_offset;

        let children_bounds = Rectangle::new(children_position, children_size);
        let child_positions = get_child_positions(menu_tree, item_height);
        let check_bounds = pad_rectangle(children_bounds, [bounds_expand; 4].into());

        Self {
            child_positions,
            children_bounds,
            parent_bounds,
            check_bounds,
        }
    }
}

pub(super) struct MenuState {
    pub(super) index: Option<usize>,
    scroll_offset: f32,
    menu_bounds: MenuBounds,
}
impl MenuState {
    fn layout<Message, Renderer>(
        &self,
        overlay_offset: Vector,
        slice: MenuSlice,
        item_height: ItemHeight,
        renderer: &Renderer,
        menu_tree: &MenuTree<'_, Message, Renderer>,
    ) -> layout::Node
    where
        Renderer: renderer::Renderer,
    {
        let MenuSlice {
            start_index,
            end_index,
            lower_bound_rel,
            upper_bound_rel,
        } = slice;

        assert_eq!(
            menu_tree.children.len(),
            self.menu_bounds.child_positions.len()
        );

        // viewport space children bounds
        let children_bounds = self.menu_bounds.children_bounds + overlay_offset;

        let child_nodes = self.menu_bounds.child_positions[start_index..=end_index]
            .iter()
            .zip(menu_tree.children[start_index..=end_index].iter())
            .map(|(cp, mt)| {
                let mut position = *cp;
                let mut size = get_item_size(mt, children_bounds.width, item_height);

                if position < lower_bound_rel && (position + size.height) > lower_bound_rel {
                    size.height = position + size.height - lower_bound_rel;
                    position = lower_bound_rel;
                } else if position <= upper_bound_rel && (position + size.height) > upper_bound_rel
                {
                    size.height = upper_bound_rel - position;
                }

                let limits = layout::Limits::new(Size::ZERO, size);

                let mut node = mt.item.as_widget().layout(renderer, &limits);
                node.move_to(Point::new(0.0, position + self.scroll_offset));
                node
            })
            .collect::<Vec<_>>();

        let mut node = layout::Node::with_children(children_bounds.size(), child_nodes);
        node.move_to(children_bounds.position());
        node
    }

    fn layout_single<Message, Renderer>(
        &self,
        overlay_offset: Vector,
        index: usize,
        item_height: ItemHeight,
        renderer: &Renderer,
        menu_tree: &MenuTree<'_, Message, Renderer>,
    ) -> layout::Node
    where
        Renderer: renderer::Renderer,
    {
        // viewport space children bounds
        let children_bounds = self.menu_bounds.children_bounds + overlay_offset;

        let position = self.menu_bounds.child_positions[index];
        let limits = layout::Limits::new(
            Size::ZERO,
            get_item_size(menu_tree, children_bounds.width, item_height),
        );
        let parent_offset = children_bounds.position() - Point::ORIGIN;
        let mut node = menu_tree.item.as_widget().layout(renderer, &limits);
        node.move_to(Point::new(
            parent_offset.x,
            parent_offset.y + position + self.scroll_offset,
        ));
        node
    }

    fn slice<Message, Renderer>(
        &self,
        viewport_size: Size,
        overlay_offset: Vector,
        item_height: ItemHeight,
        menu_tree: &MenuTree<'_, Message, Renderer>,
    ) -> MenuSlice {
        // viewport space children bounds
        let children_bounds = self.menu_bounds.children_bounds + overlay_offset;

        let max_index = self.menu_bounds.child_positions.len().saturating_sub(1);

        // viewport space absolute bounds
        let lower_bound = children_bounds.y.max(0.0);
        let upper_bound = (children_bounds.y + children_bounds.height).min(viewport_size.height);

        // menu space relative bounds
        let lower_bound_rel = lower_bound - (children_bounds.y + self.scroll_offset);
        let upper_bound_rel = upper_bound - (children_bounds.y + self.scroll_offset);

        // index range
        let (start_index, end_index) = match item_height {
            ItemHeight::Uniform(u) => {
                let start_index = (lower_bound_rel / f32::from(u)).floor() as usize;
                let end_index = ((upper_bound_rel / f32::from(u)).floor() as usize).min(max_index);
                (start_index, end_index)
            }
            ItemHeight::Static(s) => {
                let positions = &self.menu_bounds.child_positions;

                let start_index =
                    search_bound(0, 0, max_index, s, lower_bound_rel, positions, menu_tree);
                let end_index = search_bound(
                    max_index,
                    start_index,
                    max_index,
                    s,
                    upper_bound_rel,
                    positions,
                    menu_tree,
                )
                .min(max_index);

                (start_index, end_index)
            }
        };

        MenuSlice {
            start_index,
            end_index,
            lower_bound_rel,
            upper_bound_rel,
        }
    }
}

pub(super) struct Menu<'a, 'b, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub(super) tree: &'b mut Tree,
    pub(super) menu_roots: &'b mut Vec<MenuTree<'a, Message, Renderer>>,
    pub(super) bounds_expand: u16,
    pub(super) close_condition: CloseCondition,
    pub(super) item_width: ItemWidth,
    pub(super) item_height: ItemHeight,
    pub(super) bar_bounds: Rectangle,
    pub(super) root_bounds_list: Vec<Rectangle>,
    pub(super) path_highlight: Option<PathHighlight>,
    pub(super) style: &'b <Renderer::Theme as StyleSheet>::Style,
}
impl<'a, 'b, Message, Renderer> Menu<'a, 'b, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub(super) fn overlay(self) -> overlay::Element<'b, Message, Renderer> {
        overlay::Element::new(Point::ORIGIN, Box::new(self))
    }
}
impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for Menu<'a, 'b, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(&self, _renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        // overlay space viewport rectangle
        layout::Node::new(bounds).translate(Point::ORIGIN - position)
    }

    fn on_event(
        &mut self,
        event: event::Event,
        layout: layout::Layout<'_>,
        view_cursor: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        use event::{
            Event::{Mouse, Touch},
            Status::{Captured, Ignored},
        };
        use mouse::{
            Button::Left,
            Event::{ButtonPressed, ButtonReleased, CursorMoved, WheelScrolled},
        };
        use touch::Event::{FingerLifted, FingerMoved, FingerPressed};

        if !self.tree.state.downcast_ref::<MenuBarState>().open {
            return Ignored;
        };

        let viewport = layout.bounds();
        let viewport_size = viewport.size();
        let overlay_offset = Point::ORIGIN - viewport.position();
        let overlay_cursor = view_cursor - overlay_offset;

        let menu_status = process_menu_events(
            self.tree,
            self.menu_roots,
            self.item_height,
            event.clone(),
            view_cursor,
            renderer,
            clipboard,
            shell,
            overlay_offset,
        );

        init_root_menu(
            self,
            overlay_cursor,
            viewport_size,
            overlay_offset,
            self.bar_bounds,
        );

        match event {
            Mouse(WheelScrolled { delta }) => {
                process_scroll_events(self, delta, overlay_cursor, viewport_size, overlay_offset)
                    .merge(menu_status)
            }

            Mouse(ButtonPressed(Left)) | Touch(FingerPressed { .. }) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();
                state.pressed = true;
                state.view_cursor = view_cursor;
                Captured
            }

            Mouse(CursorMoved { position }) | Touch(FingerMoved { position, .. }) => {
                let view_cursor = position;
                let overlay_cursor = view_cursor - overlay_offset;
                process_overlay_events(
                    self,
                    viewport_size,
                    overlay_offset,
                    view_cursor,
                    overlay_cursor,
                )
                .merge(menu_status)
            }

            Mouse(ButtonReleased(Left)) | Touch(FingerLifted { .. }) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();
                state.pressed = false;

                // process close condition
                if state.view_cursor.distance(view_cursor) < 2.0 {
                    let is_inside = state
                        .menu_states
                        .iter()
                        .any(|ms| ms.menu_bounds.check_bounds.contains(overlay_cursor));

                    if self.close_condition.click_inside && is_inside {
                        state.reset();
                        return Captured;
                    }

                    if self.close_condition.click_outside && !is_inside {
                        state.reset();
                        return Captured;
                    }
                }

                // close all menus when clicking inside the menu bar
                if self.bar_bounds.contains(overlay_cursor) {
                    state.reset();
                    Captured
                } else {
                    menu_status
                }
            }

            _ => menu_status,
        }
    }

    #[allow(unused_results)]
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        view_cursor: Point,
    ) {
        let state = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active_root) = state.active_root else{ return; };

        let viewport = layout.bounds();
        let viewport_size = viewport.size();
        let overlay_offset = Point::ORIGIN - viewport.position();
        let render_bounds = Rectangle::new(Point::ORIGIN, viewport.size());

        let styling = theme.appearance(self.style);

        let tree = &self.tree.children[active_root].children;
        let root = &self.menu_roots[active_root];

        let indices = state.get_trimmed_indices().collect::<Vec<_>>();

        state
            .menu_states
            .iter()
            .enumerate()
            .fold(root, |menu_root, (i, ms)| {
                let draw_path = self.path_highlight.as_ref().map_or(false, |ph| match ph {
                    PathHighlight::Full => true,
                    PathHighlight::OmitActive => !indices.is_empty() && i < indices.len() - 1,
                    PathHighlight::MenuActive => i < state.menu_states.len() - 1,
                });

                // react only to the last menu
                let view_cursor = if i == state.menu_states.len() - 1 {
                    view_cursor
                } else {
                    [-1.0; 2].into()
                };

                let draw_menu = |r: &mut Renderer| {
                    // calc slice
                    let slice =
                        ms.slice(viewport_size, overlay_offset, self.item_height, menu_root);
                    let start_index = slice.start_index;
                    let end_index = slice.end_index;

                    // calc layout
                    let children_node =
                        ms.layout(overlay_offset, slice, self.item_height, r, menu_root);
                    let children_layout = layout::Layout::new(&children_node);
                    let children_bounds = children_layout.bounds();

                    // draw menu background
                    let menu_quad = renderer::Quad {
                        bounds: pad_rectangle(children_bounds, styling.background_expand.into()),
                        border_radius: styling.border_radius.into(),
                        border_width: styling.border_width,
                        border_color: styling.border_color,
                    };
                    let menu_color = styling.background;
                    r.fill_quad(menu_quad, menu_color);

                    // draw path hightlight
                    if let (true, Some(active)) = (draw_path, ms.index) {
                        let active_bounds = children_layout
                            .children()
                            .nth(active.saturating_sub(start_index))
                            .expect("No active children were found in menu?")
                            .bounds();
                        let path_quad = renderer::Quad {
                            bounds: active_bounds,
                            border_radius: styling.border_radius.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                        };
                        let path_color = styling.path;
                        r.fill_quad(path_quad, path_color);
                    }

                    // draw item
                    menu_root.children[start_index..=end_index]
                        .iter()
                        .zip(children_layout.children())
                        .for_each(|(mt, clo)| {
                            mt.item.as_widget().draw(
                                &tree[mt.index],
                                r,
                                theme,
                                style,
                                clo,
                                view_cursor,
                                &children_layout.bounds(),
                            );
                        });
                };

                renderer.with_layer(render_bounds, draw_menu);

                // only the last menu can have a None active index
                ms.index
                    .map_or(menu_root, |active| &menu_root.children[active])
            });
    }
}

fn pad_rectangle(rect: Rectangle, padding: Padding) -> Rectangle {
    Rectangle {
        x: rect.x - padding.left,
        y: rect.y - padding.top,
        width: rect.width + padding.horizontal(),
        height: rect.height + padding.vertical(),
    }
}

fn init_root_menu<Message, Renderer>(
    menu: &mut Menu<'_, '_, Message, Renderer>,
    overlay_cursor: Point,
    viewport_size: Size,
    overlay_offset: Vector,
    bar_bounds: Rectangle,
) where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    let state = menu.tree.state.downcast_mut::<MenuBarState>();
    if !(state.menu_states.is_empty() && bar_bounds.contains(overlay_cursor)) {
        return;
    }

    for (i, (&root_bounds, mt)) in menu
        .root_bounds_list
        .iter()
        .zip(menu.menu_roots.iter())
        .enumerate()
    {
        if mt.children.is_empty() {
            continue;
        }

        if root_bounds.contains(overlay_cursor) {
            let view_center = viewport_size.width * 0.5;
            let rb_center = root_bounds.center_x();

            state.horizontal_direction = if rb_center > view_center {
                Direction::Negative
            } else {
                Direction::Positive
            };

            let aod = Aod {
                horizontal: true,
                vertical: true,
                horizontal_overlap: true,
                vertical_overlap: false,
                horizontal_direction: state.horizontal_direction,
                vertical_direction: state.vertical_direction,
            };

            let menu_bounds = MenuBounds::new(
                mt,
                menu.item_width,
                menu.item_height,
                viewport_size,
                overlay_offset,
                &aod,
                menu.bounds_expand,
                root_bounds,
            );

            state.active_root = Some(i);
            state.menu_states.push(MenuState {
                index: None,
                scroll_offset: 0.0,
                menu_bounds,
            });

            break;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn process_menu_events<'b, Message, Renderer>(
    tree: &'b mut Tree,
    menu_roots: &'b mut [MenuTree<'_, Message, Renderer>],
    item_height: ItemHeight,
    event: event::Event,
    view_cursor: Point,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
    overlay_offset: Vector,
) -> event::Status
where
    Renderer: renderer::Renderer,
{
    use event::Status;

    let state = tree.state.downcast_mut::<MenuBarState>();
    let Some(active_root) = state.active_root else { return Status::Ignored; };

    let indices = state.get_trimmed_indices().collect::<Vec<_>>();

    if indices.is_empty() {
        return Status::Ignored;
    }

    // get active item
    let mt = indices
        .iter()
        .fold(&mut menu_roots[active_root], |mt, &i| &mut mt.children[i]);

    // get layout
    let last_ms = &state.menu_states[indices.len() - 1];
    let child_node = last_ms.layout_single(
        overlay_offset,
        last_ms.index.expect("missing index within menu state."),
        item_height,
        renderer,
        mt,
    );
    let child_layout = layout::Layout::new(&child_node);

    // widget tree
    let tree = &mut tree.children[active_root].children[mt.index];

    // process only the last widget
    mt.item.as_widget_mut().on_event(
        tree,
        event,
        child_layout,
        view_cursor,
        renderer,
        clipboard,
        shell,
    )
}

#[allow(unused_results)]
fn process_overlay_events<Message, Renderer>(
    menu: &mut Menu<'_, '_, Message, Renderer>,
    viewport_size: Size,
    overlay_offset: Vector,
    view_cursor: Point,
    overlay_cursor: Point,
) -> event::Status
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    use event::Status::{Captured, Ignored};
    /*
    if no active root || pressed:
        return
    else:
        remove invalid menus // overlay space
        update active item
        if active item is a menu:
            add menu // viewport space
    */

    let state = menu.tree.state.downcast_mut::<MenuBarState>();

    let Some(active_root) = state.active_root else{
        if !menu.bar_bounds.contains(overlay_cursor){
            state.reset();
        }
        return Ignored;
    };

    if state.pressed {
        return Ignored;
    }

    /* When overlay is running, cursor_position in any widget method will go negative
    but I still want Widget::draw() to react to cursor movement */
    state.view_cursor = view_cursor;

    // * remove invalid menus
    let mut prev_bounds = std::iter::once(menu.bar_bounds)
        .chain(
            state.menu_states[..state.menu_states.len().saturating_sub(1)]
                .iter()
                .map(|ms| ms.menu_bounds.children_bounds),
        )
        .collect::<Vec<_>>();

    if menu.close_condition.leave {
        for i in (0..state.menu_states.len()).rev() {
            let mb = &state.menu_states[i].menu_bounds;

            if mb.parent_bounds.contains(overlay_cursor)
                || mb.children_bounds.contains(overlay_cursor)
                || (mb.check_bounds.contains(overlay_cursor)
                    && prev_bounds.iter().all(|pvb| !pvb.contains(overlay_cursor)))
            {
                break;
            }
            prev_bounds.pop();
            state.menu_states.pop();
        }
    } else {
        for i in (0..state.menu_states.len()).rev() {
            let mb = &state.menu_states[i].menu_bounds;

            if mb.parent_bounds.contains(overlay_cursor)
                || mb.children_bounds.contains(overlay_cursor)
                || prev_bounds.iter().all(|pvb| !pvb.contains(overlay_cursor))
            {
                break;
            }
            prev_bounds.pop();
            state.menu_states.pop();
        }
    }

    // get indices
    let indices = state
        .menu_states
        .iter()
        .map(|ms| ms.index)
        .collect::<Vec<_>>();

    // * update active item
    let Some(last_menu_state) = state.menu_states.last_mut() else{
        // no menus left
        state.active_root = None;

        // keep state.open when the cursor is still inside the menu bar
        // this allows the overlay to keep drawing when the cursor is
        // moving aroung the menu bar
        if !menu.bar_bounds.contains(overlay_cursor){
            state.open = false;
        }
        return Captured;
    };

    let last_menu_bounds = &last_menu_state.menu_bounds;
    let last_parent_bounds = last_menu_bounds.parent_bounds;
    let last_children_bounds = last_menu_bounds.children_bounds;

    if last_parent_bounds.contains(overlay_cursor)
    // cursor is in the parent part
    || !last_children_bounds.contains(overlay_cursor)
    // cursor is outside
    {
        last_menu_state.index = None;
        return Captured;
    }
    // cursor is in the children part

    // calc new index
    let height_diff = (overlay_cursor.y - (last_children_bounds.y + last_menu_state.scroll_offset))
        .clamp(0.0, last_children_bounds.height - 0.001);

    let active_menu_root = &menu.menu_roots[active_root];

    let active_menu = indices[0..indices.len().saturating_sub(1)]
        .iter()
        .fold(active_menu_root, |mt, i| {
            &mt.children[i.expect("missing active child index in menu")]
        });

    let new_index = match menu.item_height {
        ItemHeight::Uniform(u) => (height_diff / f32::from(u)).floor() as usize,
        ItemHeight::Static(s) => {
            let max_index = active_menu.children.len() - 1;
            search_bound(
                0,
                0,
                max_index,
                s,
                height_diff,
                &last_menu_bounds.child_positions,
                active_menu,
            )
        }
    };

    // set new index
    last_menu_state.index = Some(new_index);

    // get new active item
    let item = &active_menu.children[new_index];

    // * add new menu if the new item is a menu
    if !item.children.is_empty() {
        let item_position = Point::new(
            0.0,
            last_menu_bounds.child_positions[new_index] + last_menu_state.scroll_offset,
        );
        let item_size = get_item_size(
            item,
            last_menu_bounds.children_bounds.width,
            menu.item_height,
        );

        // overlay space item bounds
        let item_bounds = Rectangle::new(item_position, item_size)
            + (last_menu_bounds.children_bounds.position() - Point::ORIGIN);

        let aod = Aod {
            horizontal: true,
            vertical: true,
            horizontal_overlap: false,
            vertical_overlap: true,
            horizontal_direction: state.horizontal_direction,
            vertical_direction: state.vertical_direction,
        };

        state.menu_states.push(MenuState {
            index: None,
            scroll_offset: 0.0,
            menu_bounds: MenuBounds::new(
                item,
                menu.item_width,
                menu.item_height,
                viewport_size,
                overlay_offset,
                &aod,
                menu.bounds_expand,
                item_bounds,
            ),
        });
    }

    Captured
}

fn process_scroll_events<Message, Renderer>(
    menu: &mut Menu<'_, '_, Message, Renderer>,
    delta: mouse::ScrollDelta,
    overlay_cursor: Point,
    viewport_size: Size,
    overlay_offset: Vector,
) -> event::Status
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    use event::Status::{Captured, Ignored};
    use mouse::ScrollDelta;

    let state = menu.tree.state.downcast_mut::<MenuBarState>();

    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } => y * 60.0,
        ScrollDelta::Pixels { y, .. } => y,
    };

    let calc_offset_bounds = |menu_state: &MenuState, viewport_size: Size| -> (f32, f32) {
        // viewport space children bounds
        let children_bounds = menu_state.menu_bounds.children_bounds + overlay_offset;

        let max_offset = (0.0 - children_bounds.y).max(0.0);
        let min_offset =
            (viewport_size.height - (children_bounds.y + children_bounds.height)).min(0.0);
        (max_offset, min_offset)
    };

    // update
    if state.menu_states.is_empty() {
        return Ignored;
    } else if state.menu_states.len() == 1 {
        let last_ms = &mut state.menu_states[0];

        if last_ms.index.is_none() {
            return Captured;
        }

        let (max_offset, min_offset) = calc_offset_bounds(last_ms, viewport_size);
        last_ms.scroll_offset = (last_ms.scroll_offset + delta_y).clamp(min_offset, max_offset);
    } else {
        // >= 2
        let max_index = state.menu_states.len() - 1;
        let last_two = &mut state.menu_states[max_index - 1..=max_index];

        if last_two[1].index.is_some() {
            // scroll the last one
            let (max_offset, min_offset) = calc_offset_bounds(&last_two[1], viewport_size);
            last_two[1].scroll_offset =
                (last_two[1].scroll_offset + delta_y).clamp(min_offset, max_offset);
        } else {
            if !last_two[0]
                .menu_bounds
                .children_bounds
                .contains(overlay_cursor)
            {
                return Captured;
            }

            // scroll the second last one
            let (max_offset, min_offset) = calc_offset_bounds(&last_two[0], viewport_size);
            let scroll_offset = (last_two[0].scroll_offset + delta_y).clamp(min_offset, max_offset);
            let clamped_delta_y = scroll_offset - last_two[0].scroll_offset;
            last_two[0].scroll_offset = scroll_offset;

            // update the bounds of the last one
            last_two[1].menu_bounds.parent_bounds.y += clamped_delta_y;
            last_two[1].menu_bounds.children_bounds.y += clamped_delta_y;
            last_two[1].menu_bounds.check_bounds.y += clamped_delta_y;
        }
    }
    Captured
}

fn get_item_size<Message, Renderer>(
    menu_tree: &MenuTree<'_, Message, Renderer>,
    width: f32,
    item_height: ItemHeight,
) -> Size
where
    Renderer: renderer::Renderer,
{
    let height = match item_height {
        ItemHeight::Uniform(u) => f32::from(u),
        ItemHeight::Static(s) => f32::from(menu_tree.height.unwrap_or(s)),
    };

    Size::new(width, height)
}

fn get_children_size<Message, Renderer>(
    menu_tree: &MenuTree<'_, Message, Renderer>,
    item_width: ItemWidth,
    item_height: ItemHeight,
) -> Size
where
    Renderer: renderer::Renderer,
{
    let width = match item_width {
        ItemWidth::Uniform(u) => f32::from(u),
        ItemWidth::Static(s) => f32::from(menu_tree.width.unwrap_or(s)),
    };

    let height = match item_height {
        ItemHeight::Uniform(u) => f32::from(u) * (menu_tree.children.len() as f32),
        ItemHeight::Static(s) => menu_tree
            .children
            .iter()
            .fold(0.0, |h, mt| h + f32::from(mt.height.unwrap_or(s))),
    };

    Size::new(width, height)
}

fn get_child_positions<Message, Renderer>(
    menu_tree: &MenuTree<'_, Message, Renderer>,
    item_height: ItemHeight,
) -> Vec<f32>
where
    Renderer: renderer::Renderer,
{
    match item_height {
        ItemHeight::Uniform(u) => {
            let children_count = menu_tree.children.len();
            (0..children_count)
                .map(|i| (i as f32) * f32::from(u))
                .collect()
        }
        ItemHeight::Static(s) => {
            let max_index = menu_tree.children.len() - 1;
            std::iter::once(0.0)
                .chain(menu_tree.children[0..max_index].iter().scan(0.0, |p, mt| {
                    *p += f32::from(mt.height.unwrap_or(s));
                    Some(*p)
                }))
                .collect::<Vec<_>>()
        }
    }
}

fn search_bound<Message, Renderer>(
    default: usize,
    default_left: usize,
    default_right: usize,
    default_height: u16,
    bound: f32,
    positions: &[f32],
    menu_tree: &MenuTree<'_, Message, Renderer>,
) -> usize {
    // binary search
    let mut left = default_left;
    let mut right = default_right;

    let mut index = default;
    while left != right {
        let m = ((left + right) / 2) + 1;
        if positions[m] > bound {
            right = m - 1;
        } else {
            left = m;
        }
    }
    let height = f32::from(menu_tree.children[left].height.unwrap_or(default_height));
    if positions[left] + height > bound {
        index = left;
    }
    index
}

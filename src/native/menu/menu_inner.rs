//! Menu tree overlay

use super::menu_bar::MenuBarState;
use super::menu_tree::MenuTree;
use crate::style::menu_bar::StyleSheet;
use iced_native::widget::Tree;
use iced_native::{
    event, layout, mouse, overlay, renderer, touch, Clipboard, Color, Padding, Point, Rectangle,
    Shell, Size,
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

#[derive(Debug, Clone, Copy)]
struct MenuSlice {
    start_index: usize,
    end_index: usize,
    lower_bound_rel: f32,
    upper_bound_rel: f32,
}

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
        viewport: Size,
        aod_settings: [bool; 4],
        bounds_expand: u16,
        parent_bounds: Rectangle,
    ) -> Self
    where
        Renderer: renderer::Renderer,
    {
        let children_size = get_children_size(menu_tree, item_width, item_height);
        let children_position =
            adaptive_open_direction(parent_bounds, children_size, viewport, aod_settings);
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

        let children_bounds = self.menu_bounds.children_bounds;
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

        let mut node =
            layout::Node::with_children(self.menu_bounds.children_bounds.size(), child_nodes);
        node.move_to(self.menu_bounds.children_bounds.position());
        node
    }

    fn layout_single<Message, Renderer>(
        &self,
        index: usize,
        item_height: ItemHeight,
        renderer: &Renderer,
        menu_tree: &MenuTree<'_, Message, Renderer>,
    ) -> layout::Node
    where
        Renderer: renderer::Renderer,
    {
        let children_bounds = self.menu_bounds.children_bounds;
        let position = self.menu_bounds.child_positions[index];
        let limits = layout::Limits::new(
            Size::ZERO,
            get_item_size(menu_tree, children_bounds.width, item_height),
        );
        let parent_offset = self.menu_bounds.children_bounds.position() - Point::ORIGIN;
        let mut node = menu_tree.item.as_widget().layout(renderer, &limits);
        node.move_to(Point::new(
            parent_offset.x,
            parent_offset.y + position + self.scroll_offset,
        ));
        node
    }

    fn slice<Message, Renderer>(
        &self,
        viewport: Size,
        item_height: ItemHeight,
        menu_tree: &MenuTree<'_, Message, Renderer>,
    ) -> MenuSlice {
        let children_bounds = self.menu_bounds.children_bounds;
        let max_index = self.menu_bounds.child_positions.len().saturating_sub(1);

        // absolute bounds
        let lower_bound = children_bounds.y.max(0.0);
        let upper_bound = (children_bounds.y + children_bounds.height).min(viewport.height);

        // relative bounds
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
    fn layout(&self, _renderer: &Renderer, bounds: Size, _position: Point) -> layout::Node {
        layout::Node::new(bounds)
    }

    fn on_event(
        &mut self,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
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

        let viewport = layout.bounds().size();

        let menu_status = process_menu_events(
            self.tree,
            self.menu_roots,
            self.item_height,
            event.clone(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        );

        init_root_menu(self, cursor_position, viewport, self.bar_bounds);

        match event {
            Mouse(WheelScrolled { delta }) => {
                process_scroll_events(self, delta, viewport).merge(menu_status)
            }

            Mouse(ButtonPressed(Left)) | Touch(FingerPressed { .. }) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();
                state.pressed = true;
                state.cursor = cursor_position;
                Captured
            }

            Mouse(CursorMoved { position }) | Touch(FingerMoved { position, .. }) => {
                process_overlay_events(self, viewport, position).merge(menu_status)
            }

            Mouse(ButtonReleased(Left)) | Touch(FingerLifted { .. }) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();
                state.pressed = false;

                if state.cursor.distance(cursor_position) < 2.0 {
                    let is_inside = state
                        .menu_states
                        .iter()
                        .any(|ms| ms.menu_bounds.check_bounds.contains(cursor_position));

                    if self.close_condition.click_inside && is_inside {
                        state.reset();
                        return Captured;
                    }

                    if self.close_condition.click_outside && !is_inside {
                        state.reset();
                        return Captured;
                    }
                }

                if self.bar_bounds.contains(cursor_position) {
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
        cursor_position: Point,
    ) {
        let styling = theme.appearance(self.style);

        let state = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active_root) = state.active_root else{ return; };

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

                let draw_menu = |r: &mut Renderer| {
                    // calc slice
                    let slice = ms.slice(layout.bounds().size(), self.item_height, menu_root);
                    let start_index = slice.start_index;
                    let end_index = slice.end_index;

                    // calc layout
                    let children_node = ms.layout(slice, self.item_height, r, menu_root);
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
                                cursor_position,
                                &children_layout.bounds(),
                            );
                        });
                };

                renderer.with_layer(layout.bounds(), draw_menu);

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
    position: Point,
    viewport: Size,
    bar_bounds: Rectangle,
) where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    let state = menu.tree.state.downcast_mut::<MenuBarState>();
    if !(state.menu_states.is_empty() && bar_bounds.contains(position)) {
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

        if root_bounds.contains(position) {
            let menu_bounds = MenuBounds::new(
                mt,
                menu.item_width,
                menu.item_height,
                viewport,
                [false, true, true, false],
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
    cursor_position: Point,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
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
        cursor_position,
        renderer,
        clipboard,
        shell,
    )
}

#[allow(unused_results)]
fn process_overlay_events<Message, Renderer>(
    menu: &mut Menu<'_, '_, Message, Renderer>,
    viewport: Size,
    position: Point,
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
        remove invalid menus
        update active item
        if active item is a menu:
            add menu
    */

    let state = menu.tree.state.downcast_mut::<MenuBarState>();

    let Some(active_root) = state.active_root else{
        if !menu.bar_bounds.contains(position){
            state.reset();
        }
        return Ignored;
    };
    
    if state.pressed {
        return Ignored;
    }

    /* When overlay is running, cursor_position in any widget method will go negative
    but I still want Widget::draw() to react to cursor movement */
    state.cursor = position;

    // remove invalid menus
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

            if (mb.parent_bounds.contains(position) || mb.check_bounds.contains(position))
                && prev_bounds.iter().all(|pvb| !pvb.contains(position))
            {
                break;
            }
            prev_bounds.pop();
            state.menu_states.pop();
        }
    } else {
        for i in (0..state.menu_states.len()).rev() {
            let mb = &state.menu_states[i].menu_bounds;

            if mb.parent_bounds.contains(position)
                || prev_bounds.iter().all(|pvb| !pvb.contains(position))
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

    // update active item
    let Some(last_menu_state) = state.menu_states.last_mut() else{
        // no menus left
        state.active_root = None;

        // keep state.open when the cursor is still inside the menu bar
        // this allows the overlay to keep drawing when the cursor is
        // moving aroung the menu bar
        if !menu.bar_bounds.contains(position){
            state.open = false;
        }
        return Captured;
    };

    let last_menu_bounds = &last_menu_state.menu_bounds;
    let last_parent_bounds = last_menu_bounds.parent_bounds;
    let last_child_bounds = last_menu_bounds.children_bounds;
    let last_check_bounds = last_menu_bounds.check_bounds;

    if last_parent_bounds.contains(position)
    // cursor is in the parent part
    || !last_check_bounds.contains(position)
    // cursor is outside
    {
        last_menu_state.index = None;
        return Captured;
    }
    // cursor is in the children part

    // calc new index
    let height_diff = (position.y - (last_child_bounds.y + last_menu_state.scroll_offset))
        .clamp(0.0, last_child_bounds.height - 0.001);

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

    // add new menu if the new item is a menu
    if !item.children.is_empty() {
        // get new item bounds
        let item_position = Point::new(
            0.0,
            last_menu_bounds.child_positions[new_index] + last_menu_state.scroll_offset,
        );
        let item_size = get_item_size(
            item,
            last_menu_bounds.children_bounds.width,
            menu.item_height,
        );

        let item_bounds = Rectangle::new(item_position, item_size)
            + (last_menu_bounds.children_bounds.position() - Point::ORIGIN);

        state.menu_states.push(MenuState {
            index: None,
            scroll_offset: 0.0,
            menu_bounds: MenuBounds::new(
                item,
                menu.item_width,
                menu.item_height,
                viewport,
                [true, true, false, true],
                menu.bounds_expand,
                item_bounds,
            ),
        });
    }

    Captured
}

fn adaptive_open_direction(
    parent_bounds: Rectangle,
    children_size: Size,
    bounds: Size,
    setttings: [bool; 4],
) -> Point {
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

    The child goes to the right by default, if the right space runs out
    it goes to the left, whether to use overlap is the caller's decision

    This can be applied to any direction
    */

    let [horizontal, vertical, horizontal_overlap, vertical_overlap] = setttings;

    let calc_adaptive = |parent_pos, parent_size, child_size, max_size, on, overlap| {
        if on {
            let space_left = parent_pos;
            let space_right = max_size - parent_pos - parent_size;

            if space_left > space_right && child_size > space_right {
                return if overlap {
                    parent_pos - child_size + parent_size
                } else {
                    parent_pos - child_size
                };
            }
        }

        if overlap {
            parent_pos
        } else {
            parent_pos + parent_size
        }
    };

    let x = calc_adaptive(
        parent_bounds.x,
        parent_bounds.width,
        children_size.width,
        bounds.width,
        horizontal,
        horizontal_overlap,
    );
    let y = calc_adaptive(
        parent_bounds.y,
        parent_bounds.height,
        children_size.height,
        bounds.height,
        vertical,
        vertical_overlap,
    );

    [x, y].into()
}

fn process_scroll_events<Message, Renderer>(
    menu: &mut Menu<'_, '_, Message, Renderer>,
    delta: mouse::ScrollDelta,
    viewport: Size,
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

    let calc_offset_bounds = |menu_state: &MenuState, viewport: Size| -> (f32, f32) {
        let children_bounds = menu_state.menu_bounds.children_bounds;
        let max_offset = (0.0 - children_bounds.y).max(0.0);
        let min_offset = (viewport.height - (children_bounds.y + children_bounds.height)).min(0.0);
        (max_offset, min_offset)
    };

    // update
    if state.menu_states.is_empty() {
        return Ignored;
    } else if state.menu_states.len() == 1 {
        let last_ms = &mut state.menu_states[0];
        let (max_offset, min_offset) = calc_offset_bounds(last_ms, viewport);
        last_ms.scroll_offset = (last_ms.scroll_offset + delta_y).clamp(min_offset, max_offset);
    } else {
        // >= 2
        let max_index = state.menu_states.len() - 1;
        let last_two = &mut state.menu_states[max_index - 1..=max_index];

        if last_two[1].index.is_some() {
            // scroll the last one
            let (max_offset, min_offset) = calc_offset_bounds(&last_two[1], viewport);
            last_two[1].scroll_offset =
                (last_two[1].scroll_offset + delta_y).clamp(min_offset, max_offset);
        } else {
            // scroll the second last one
            let (max_offset, min_offset) = calc_offset_bounds(&last_two[0], viewport);
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

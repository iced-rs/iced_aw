use iced_core::{
    Padding, Rectangle, Shell, Size,
    layout::{Layout, Node},
    mouse, renderer,
    widget::Tree,
};

use super::menu_bar::{GlobalState, MenuBarTask};
use super::menu_tree::{Item, MenuState};
use crate::style::menu_bar::Catalog;

///
/// ## FakeHovering:
///
/// Places cursors at the path items,
/// useful when you want to customize the styling of each item in the path,
/// or you simple want the look of the items when they are hovered over.
///
/// The downside is when some widget in the path don't response to hovering,
/// the path won't be fully drawn, and when you want uniform path styling
/// but some widget response to hovering differently.
///
/// ## Backdrop:
///
/// Draws a rectangle behind each path item,
/// requires path items to have transparent backgrounds,
/// useful when you want uniform path styling.
///
/// The downside is,
/// depending on the style you're going for,
/// oftentimes manually syncing the path's styling to the path items' is necessary,
/// the default styling simply can't cover most use cases.
#[derive(Debug, Clone, Copy)]
pub enum DrawPath {
    /// FakeHovering
    FakeHovering,
    /// Backdrop
    Backdrop,
}

/// X+ goes right and Y+ goes down
#[derive(Debug, Clone, Copy)]
pub(super) enum Direction {
    Positive,
    Negative,
}
impl Direction {
    pub(super) fn flip(self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

/// Axis
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub(super) enum Axis {
    Horizontal,
    Vertical,
}

pub(super) type Index = Option<usize>;

/// Should be returned from the recursive event processing function,
/// tells the caller which type of event has been processed
///
/// `Event`: The child event has been processed.
/// The parent menu should not process the event.
///
/// `Close`: Either the child menu has decided to close itself,
/// or that there is no child menu open,
/// from the parent menu's perspective,
/// there is no difference between the two.
/// The parent menu should check if it should close itself,
/// if not then it should process the event.
///
/// `None`: A child menu is open, but it did not process the event,
/// this happens when the cursor hovers over the item that opens the child menu
/// but has not entered the child menu yet,
/// in this case the parent menu should process the event,
/// but close check is not needed.
///
#[derive(Debug, Clone, Copy)]
pub(super) enum RecEvent {
    Event,
    Close,
    None,
}

#[derive(Debug, Clone, Copy)]
/// Scroll speed
pub struct ScrollSpeed {
    /// Speed of line-based scroll movement
    pub line: f32,
    /// Speed of Pixel scroll movement
    pub pixel: f32,
}

pub fn pad_rectangle(rect: Rectangle, padding: Padding) -> Rectangle {
    Rectangle {
        x: rect.x - padding.left,
        y: rect.y - padding.top,
        width: rect.width + padding.x(),
        height: rect.height + padding.y(),
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct MenuSlice {
    pub(super) start_index: usize,
    pub(super) end_index: usize,
    pub(super) lower_bound_rel: f32,
    pub(super) upper_bound_rel: f32,
}
impl MenuSlice {
    pub(super) fn from_bounds_rel(
        lower_bound_rel: f32,
        upper_bound_rel: f32,
        items_node: &Node,
        get_position: fn(&Node) -> f32,
    ) -> Self {
        let max_index = items_node.children().len().saturating_sub(1);
        let nodes = items_node.children();
        let start_index = search_bound(0, max_index, lower_bound_rel, nodes, get_position);
        let end_index = search_bound(start_index, max_index, upper_bound_rel, nodes, get_position);

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

pub(super) fn search_bound(
    default_left: usize,
    default_right: usize,
    bound: f32,
    list: &[Node],
    get_position: fn(&Node) -> f32,
) -> usize {
    // binary search
    let mut left = default_left;
    let mut right = default_right;

    while left != right {
        let m = usize::midpoint(left, right) + 1;
        if get_position(&list[m]) > bound {
            right = m - 1;
        } else {
            left = m;
        }
    }
    left
}

pub(super) fn clip_node_y(node: &Node, height: f32, offset: f32) -> Node {
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

pub(super) fn clip_node_x(node: &Node, width: f32, offset: f32) -> Node {
    let node_bounds = node.bounds();
    Node::with_children(
        Size::new(width, node_bounds.height),
        node.children()
            .iter()
            .map(|n| n.clone().translate([-offset, 0.0]))
            .collect(),
    )
    .move_to(node_bounds.position())
    .translate([offset, 0.0])
}

/// Parameters that are shared by all menus in the menu bar
pub(super) struct GlobalParameters<'a, Theme: crate::style::menu_bar::Catalog> {
    pub(super) safe_bounds_margin: f32,
    pub(super) draw_path: DrawPath,
    pub(super) scroll_speed: ScrollSpeed,
    pub(super) close_on_item_click: bool,
    pub(super) close_on_background_click: bool,
    pub(super) class: Theme::Class<'a>,
}

/// Tries to open a menu at the given cursor position
pub(super) fn try_open_menu<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
    items: &mut [Item<'a, Message, Theme, Renderer>],
    menu_state: &mut MenuState,
    item_trees: &mut [Tree],
    item_layouts: impl Iterator<Item = Layout<'b>>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) {
    let old_active = menu_state.active;
    let slice = menu_state.slice;

    for (i, ((item, tree), layout)) in
        itl_iter_slice_enum!(slice, items;iter_mut, item_trees;iter_mut, item_layouts)
    {
        if cursor.is_over(layout.bounds()) {
            if item.menu.is_some() {
                menu_state.open_new_menu(i, item, tree);
            }
            break;
        }
    }

    if menu_state.active != old_active {
        shell.invalidate_layout();
        shell.request_redraw();
    }
}

/// Schedules a close on click task if applicable
///
/// This function assumes that a mouse::Event::ButtonPressed(mouse::Button::Left) event has occurred,
/// make sure to check the event before calling this function.
#[allow(clippy::too_many_arguments)]
pub(super) fn schedule_close_on_click<
    'a,
    'b,
    Message,
    Theme: Catalog,
    Renderer: renderer::Renderer,
>(
    global_state: &mut GlobalState,
    global_parameters: &GlobalParameters<'_, Theme>,
    slice: MenuSlice,
    items: &mut [Item<'a, Message, Theme, Renderer>],
    slice_layout: impl Iterator<Item = Layout<'b>>,
    cursor: mouse::Cursor,
    menu_coic: Option<bool>,
    menu_cobc: Option<bool>,
) {
    global_state.clear_task();

    let mut coc_handled = false;

    for (item, layout) in items[slice.start_index..=slice.end_index] // [item...]
        .iter_mut()
        .zip(slice_layout)
    {
        if cursor.is_over(layout.bounds()) {
            if let Some(coc) = item.close_on_click {
                coc_handled = true;
                if coc {
                    global_state.schedule(MenuBarTask::CloseOnClick);
                }
            }
            for cocfb in [menu_coic, Some(global_parameters.close_on_item_click)] {
                if let (false, Some(coc)) = (coc_handled, cocfb) {
                    coc_handled = true;
                    if coc {
                        global_state.schedule(MenuBarTask::CloseOnClick);
                    }
                }
            }
            break;
        }
    }

    for cocfb in [menu_cobc, Some(global_parameters.close_on_background_click)] {
        if let (false, Some(coc)) = (coc_handled, cocfb) {
            coc_handled = true;
            if coc {
                global_state.schedule(MenuBarTask::CloseOnClick);
            }
        }
    }
}

macro_rules! itl_iter_slice {
    ($slice:expr, $items:expr;$iter_0:ident, $item_trees:expr;$iter_1:ident, $slice_layout:expr) => {
        $items[$slice.start_index..=$slice.end_index]
            .$iter_0()
            .zip($item_trees[$slice.start_index..=$slice.end_index].$iter_1())
            .zip($slice_layout)
    };
}
pub(super) use itl_iter_slice;

macro_rules! itl_iter_slice_enum {
    ($slice:expr, $items:expr;$iter_0:ident, $item_trees:expr;$iter_1:ident, $slice_layout:expr) => {
        itl_iter_slice!($slice, $items;$iter_0, $item_trees;$iter_1, $slice_layout)
            .enumerate()
            .map(move |(i, ((item, tree), layout))| (i + $slice.start_index, ((item, tree), layout)))
    };
}
pub(super) use itl_iter_slice_enum;

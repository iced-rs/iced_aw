use iced::{
    advanced::{mouse, renderer, widget::Tree, Clipboard, Layout, Shell},
    window::RedrawRequest,
    Event, Padding, Rectangle,
};

use super::menu_bar::*;
use super::menu_tree::*;
use crate::style::menu_bar::*;

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
        width: rect.width + padding.horizontal(),
        height: rect.height + padding.vertical(),
    }
}

/// Parameters that are shared by all menus in the menu bar
pub(super) struct GlobalParameters<'a, Theme: crate::style::menu_bar::Catalog> {
    pub(super) check_bounds_width: f32,
    pub(super) draw_path: DrawPath,
    pub(super) scroll_speed: ScrollSpeed,
    pub(super) close_on_item_click: bool,
    pub(super) close_on_background_click: bool,
    pub(super) class: Theme::Class<'a>,
}

/// Merges the fake shell into the real shell,
/// this makes sure that the fake shell status does not propagate to the real shell.
///
/// Current limitation: messages are not merged.
pub fn merge_fake_shell<Message>(shell: &mut Shell<'_, Message>, fake_shell: Shell<'_, Message>) {
    if fake_shell.is_layout_invalid() {
        shell.invalidate_layout();
    }

    if fake_shell.are_widgets_invalid() {
        shell.invalidate_widgets();
    }

    let rr = fake_shell.redraw_request();

    if rr < shell.redraw_request() {
        match rr {
            RedrawRequest::NextFrame => {
                shell.request_redraw();
            }
            RedrawRequest::At(time) => {
                shell.request_redraw_at(time);
            }
            RedrawRequest::Wait => {}
        }
    }
}

/// Tries to open a menu at the given cursor position
pub(super) fn try_open_menu<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
    items: &mut [Item<'a, Message, Theme, Renderer>],
    menu_state: &mut MenuState,
    item_trees: &mut [Tree],
    item_layouts: impl Iterator<Item = Layout<'b>>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
    index_offset: usize,
) {
    let old_active = menu_state.active.clone();

    for (i, ((item, tree), layout)) in items
        .iter_mut() // [Item...]
        .zip(item_trees.iter_mut()) // [item_tree...]
        .zip(item_layouts)
        .enumerate()
    {
        if cursor.is_over(layout.bounds()) {
            if item.menu.is_some() {
                menu_state.open_new_menu(i + index_offset, item, tree);
            }
            break;
        }
    }

    if menu_state.active != old_active {
        shell.invalidate_layout();
        shell.request_redraw();
    }
}

pub(super) fn update_items<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
    items: &mut [Item<'a, Message, Theme, Renderer>],
    item_trees: &mut [Tree],
    item_layouts: impl Iterator<Item = Layout<'b>>,
    event: &Event,
    cursor: mouse::Cursor,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
    viewport: &Rectangle,
) {
    for ((item, tree), layout) in items // [item...]
        .iter_mut()
        .zip(item_trees.iter_mut()) // [item_tree...]
        .zip(item_layouts)
    {
        item.update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }
}

/// Schedules a close on click task if applicable
///
/// This function assumes that a mouse::Event::ButtonPressed(mouse::Button::Left) event has occurred,
/// make sure to check the event before calling this function.
pub(super) fn schedule_close_on_click<
    'a,
    'b,
    Message,
    Theme: Catalog,
    Renderer: renderer::Renderer,
>(
    global_state: &mut GlobalState,
    global_parameters: &GlobalParameters<'_, Theme>,
    items: &mut [Item<'a, Message, Theme, Renderer>],
    item_layouts: impl Iterator<Item = Layout<'b>>,
    cursor: mouse::Cursor,
    menu_coic: Option<bool>,
    menu_cobc: Option<bool>,
) {
    global_state.clear_task();

    let mut coc_handled = false;

    for (item, layout) in items // [item...]
        .iter_mut()
        .zip(item_layouts)
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

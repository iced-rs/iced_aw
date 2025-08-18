use iced::{Padding, Rectangle};

/* /// The condition of when to close a menu
#[derive(Debug, Clone, Copy)]
pub struct CloseCondition {
    /// Close menus when the cursor moves outside the check bounds
    pub leave: bool,

    /// Close menus when the cursor clicks outside the check bounds
    pub click_outside: bool,

    /// Close menus when the cursor clicks inside the check bounds
    pub click_inside: bool,
}
 */

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
/// The parent menu should process a redraw request event.
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
#[derive(Debug)]
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

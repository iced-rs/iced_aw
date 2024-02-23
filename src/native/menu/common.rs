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
/// The downside is when some widgets in the path don't response to hovering,
/// the path won't be fully drawn, and when you want uniform path styling
/// but some widgets response to hovering differently.
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

pub fn pad_rectangle(rect: Rectangle, padding: Padding) -> Rectangle {
    Rectangle {
        x: rect.x - padding.left,
        y: rect.y - padding.top,
        width: rect.width + padding.horizontal(),
        height: rect.height + padding.vertical(),
    }
}

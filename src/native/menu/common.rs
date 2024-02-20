use iced_widget::core::{
    Padding, Rectangle
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

// /// Methods for drawing path highlight
// #[derive(Debug, Clone, Copy)]
// pub enum PathHighlight {
//     /// Draw the full path,
//     Full,
//     /// Omit the active item(the last item in the path)
//     OmitActive,
//     /// Omit the active item if it's not a menu
//     MenuActive,
// }

/// X+ goes right and Y+ goes down
#[derive(Debug, Clone, Copy)]
pub(super) enum Direction {
    Positive,
    Negative,
}
impl Direction{
    pub(super) fn flip(&self) -> Direction{
        match self {
            Direction::Positive => Direction::Negative,
            Direction::Negative => Direction::Positive,
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

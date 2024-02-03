use iced_widget::core::{overlay, renderer};

use super::menu_tree_overlay::MenuTreeOverlay;


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

// /// The width of an item
// #[derive(Debug, Clone, Copy)]
// pub enum ItemWidth {
//     /// Use uniform width
//     Uniform(u16),
//     /// Static tries to use the width value of each menu(menu tree with children),
//     /// the widths of items(menu tree with empty children) will be the same as the menu they're in,
//     /// if that value is None,
//     /// the default value will be used instead,
//     /// which is the value of the Static variant
//     Static(u16),
// }

// /// The height of an item
// #[derive(Debug, Clone, Copy)]
// pub enum ItemHeight {
//     /// Use uniform height.
//     Uniform(u16),
//     /// Static tries to use `MenuTree.height` as item height,
//     /// when it's `None` it'll fallback to the value of the `Static` variant.
//     Static(u16),
//     /// Dynamic tries to automatically choose the proper item height for you,
//     /// but it only works in certain cases:
//     ///
//     /// - Fixed height
//     /// - Shrink height
//     /// - Menu tree height
//     ///
//     /// If none of these is the case, it'll fallback to the value of the `Dynamic` variant.
//     Dynamic(u16),
// }

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

/// Axis
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum Axis{
    Horizontal,
    Vertical,
}

#[allow(missing_docs)]
pub enum OpenCondition{
    Hover,
    Click,
}

pub(super) enum MenuOverlayElement<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    Overlay(overlay::Element<'b, Message, Theme, Renderer>),
    MenuTree(MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>)
}
impl<'a, 'b, Message, Theme, Renderer> 
    From<overlay::Element<'b, Message, Theme, Renderer>> for 
    MenuOverlayElement<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: overlay::Element<'b, Message, Theme, Renderer>) -> Self {
        Self::Overlay(value)
    }
}
impl<'a, 'b, Message, Theme, Renderer> 
    From<MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>> for 
    MenuOverlayElement<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>) -> Self {
        Self::MenuTree(value)
    }
}

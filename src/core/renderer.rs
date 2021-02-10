//! Helper struct for drawing

use iced_native::{Layout, Point, Rectangle};

/// Collection of all necessary data to draw a widget.
#[derive(Debug)]
pub struct DrawEnvironment<'a, Defaults, Style, Focus> {
    /// The defaults of the renderer.
    pub defaults: &'a Defaults,
    /// The layout of the widget.
    pub layout: Layout<'a>,
    /// The position of the cursor.
    pub cursor_position: Point,
    /// The style of the widget.
    pub style_sheet: &'a Style,
    /// The viewport of the renderer.
    pub viewport: Option<&'a Rectangle>,
    /// The focus to an input element on the widget.
    pub focus: Focus,
}

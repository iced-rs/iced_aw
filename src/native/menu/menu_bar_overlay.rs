use iced_widget::core::{
    event, layout::{self, Node, Limits}, mouse, overlay, renderer, touch, widget::tree::{self, Tree}, Alignment, Border, Clipboard, Color, Element, Event, Layout, Length, Padding, Point, Rectangle, Shell, Size, Vector, Widget
};

use super::{
    menu_tree::*,
    menu_bar::MenuBarState,
};

pub(super) struct MenuBarOverlay<'a, 'b,  Message, Theme, Renderer>
where   
    Renderer: renderer::Renderer,
{
    /// Tree{ bar_state, [item_tree...] }
    pub(super) tree: &'b mut Tree,
    /// Tree{ item_state, [widget_tree, menu_tree] }
    // pub(super) bar: &'b mut MenuBarState,
    // pub(super) active_tree: &'b mut Tree,
    // pub(super) active_root: &'b mut Item<'a, Message, Theme, Renderer>,
    pub(super) roots: &'b mut [Item<'a, Message, Theme, Renderer>],
    pub(super) parent_bounds: Rectangle,
    pub(super) bar_bounds: Rectangle,
}
impl<'a, 'b,  Message, Theme, Renderer> MenuBarOverlay<'a, 'b,  Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn overlay_element(self) -> overlay::Element<'b, Message, Theme, Renderer>{
        overlay::Element::new(Point::ORIGIN, Box::new(self))
    }
}
impl<'a, 'b,  Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer> for MenuBarOverlay<'a, 'b,  Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// out: Node{size:0, \[widget_node, menu_node]}
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
        translation: Vector,
    ) -> Node {
        println!("mbo layout");
        todo!()
    }

    /// layout: Node{size:0, \[widget_node, menu_node]}
    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        println!("mbo event");
        todo!()
    }
    
    /// layout: Node{size:0, \[widget_node, menu_node]}
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        todo!()
    }
}
use iced_widget::core::{
    event, layout::{self, Node, Limits}, mouse, overlay, renderer, touch, widget::tree::{self, Tree}, Alignment, Border, Clipboard, Color, Element, Event, Layout, Length, Padding, Point, Rectangle, Shell, Size, Vector, Widget
};
use crate::menu::menu_tree_overlay::MenuTreeOverlay;

use super::{
    menu_tree::{MenuTree, MenuTreeState},
    menu_bar::MenuBarState,
};

pub(super) struct MenuBarOverlay<'a, 'b,  Message, Theme, Renderer>
where   
    Renderer: renderer::Renderer,
{
    pub(super) tree: &'b mut Tree,
    pub(super) active_tree: &'b mut Tree,
    pub(super) active_root: &'b mut MenuTree<'a, Message, Theme, Renderer>,
    pub(super) parent_bounds: Rectangle,

}
impl<'a, 'b,  Message, Theme, Renderer> MenuBarOverlay<'a, 'b,  Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn overlay_element(self) -> overlay::Element<'a, Message, Theme, Renderer>{
        overlay::Element::new(Point::ORIGIN, Box::new(self))
    }
}
impl<'a, 'b,  Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer> for MenuBarOverlay<'a, 'b,  Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
        translation: Vector,
    ) -> Node {
        /*
        Node{
            rect:
            children: [
                Node{} // mto layout
                Node{} // overlays
                Node{} // next mto
            ]
        }
        */

        fn rec<'a, Message, Theme, Renderer:renderer::Renderer>(
            tree: &mut Tree,
            menu_tree: &MenuTree<'a, Message, Theme, Renderer>,
            parent_bounds: Rectangle,
            renderer: &Renderer,
            bounds: Size,
            position: Point,
            translation: Vector,
        ) -> Node {
            let size = Size::INFINITY;
            let state = tree.state.downcast_mut::<MenuTreeState>();

            let mut mto = MenuTreeOverlay{
                state,
                tree,
                menu_tree,
                parent_bounds,
            };

            let mto_layout = mto.layout(renderer, bounds, position, translation);
            
            let overlays = menu_tree.children
                .iter()
                .zip(tree.children[1].children.iter_mut())
                .zip(mto_layout.children()[0].children())
                .filter_map(|((mt, t), n)|{
                    mt.parent.as_widget_mut()
                        .overlay(t, Layout::new(n), renderer)
                        .and_then(|mut o| Some(o.layout(renderer, bounds, translation)))
                })
                .collect::<Vec<_>>();
            
            let next = menu_tree.children
                .iter()
                .zip(tree.children[1].children.iter_mut())
                .zip(mto_layout.children()[0].children())
                .find_map(|((mt, t), l)|{
                    let state = t.state.downcast_mut::<MenuTreeState>();
                    if state.open{
                        let parent_bounds = l.bounds();
                        // let position = parent_bounds.position();
                        let position = Point::ORIGIN;
                        Some(rec(t, mt, parent_bounds, renderer, bounds, position, position - Point::ORIGIN))
                    }else{
                        None
                    }
                });

            Node::with_children(
                size, 
                next.map_or(
                    [
                        mto_layout,
                        Node::with_children(size, overlays),
                    ].into(), 
                    |n| [
                        mto_layout,
                        Node::with_children(size, overlays),
                        n
                    ].into()
                )
            )
        }

        rec(
            self.active_tree, 
            self.active_root, 
            self.parent_bounds, 
            renderer, 
            bounds, 
            position, 
            translation
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        todo!()
    }
    
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        fn rec<'a, Message, Theme, Renderer:renderer::Renderer>(
            tree: &mut Tree,
            menu_tree: &MenuTree<'a, Message, Theme, Renderer>,
            parent_bounds: Rectangle,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ){
            let mut lc = layout.children();
            let mto_layout = lc.next().unwrap();
            let overlay_layout = lc.next().unwrap();

            let state = tree.state.downcast_mut::<MenuTreeState>();
            let mut mto = MenuTreeOverlay{
                state,
                tree,
                menu_tree,
                parent_bounds,
            };

            mto.draw(renderer, theme, style, mto_layout, cursor);

            menu_tree.children
                .iter()
                .zip(tree.children[1].children.iter_mut())
                .filter_map(|(mt, t)|{
                    mt.parent.as_widget_mut().overlay(t, Layout::new(&mt.layout(t, renderer, &Limits::NONE)), renderer)
                })
                .zip(overlay_layout.children())
                .for_each(|(o, l)|{
                    o.draw(renderer, theme, style, l, cursor)
                });

            if let Some(next_layout) = lc.next(){
                rec(tree, menu_tree, parent_bounds, renderer, theme, style, next_layout, cursor, viewport);
            }

        }
    }
}
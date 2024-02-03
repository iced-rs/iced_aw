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
        let viewport = Rectangle::new(Point::ORIGIN, bounds);
        let limits = Limits::NONE;
        
        let bar = self.tree.state.downcast_mut::<MenuBarState>();
        let active_root = &self.roots[bar.active_root];
        let active_tree = &mut self.tree.children[bar.active_root];

        active_root.layout(active_tree, renderer, &limits, &viewport, 
            // translation
        )
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
        fn rec<'a, Message, Theme, Renderer:renderer::Renderer>(
            bar: &mut MenuBarState,
            current_item: &mut Item<'a, Message, Theme, Renderer>,
            current_tree: &mut Tree,
            current_layout: Layout<'_>,
            event: Event,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            cursor: mouse::Cursor,
            parent_bounds: Rectangle,
            items_bounds_list: &mut Vec<Rectangle>,
        ) -> event::Status{
            let menu = current_item.menu.as_mut().expect("No menu defined in this item");
            let mut lc = current_layout.children(); 
            let _ = lc.next().unwrap(); // widget_node
            let menu_layout = lc.next().unwrap(); // menu_node: Node{size:inf, [items_layout, prescroll, offset_bounds, check_bounds]}
            let items_layout = menu_layout.children().next().unwrap(); // items_layout: Node{size:items_bounds, [item_layout...]}

            let menu_tree = &mut current_tree.children[1]; // menu_tree: Tree{ menu_state, [item_tree...] }

            let item_state = current_tree.state.downcast_mut::<ItemState>();
            let viewport = bar.viewport;

            // on_event
            let status = menu.on_event(
                item_state, menu_tree, event.clone(), menu_layout, cursor, renderer, clipboard, shell, &viewport, &parent_bounds, &items_bounds_list
            );

            // push items_bounds
            items_bounds_list.push(items_layout.bounds());

            if let Some((next_item, next_tree, next_layout)) = menu.items // [item...]
                .iter_mut()
                .zip(menu_tree.children.iter_mut()) // [item_tree...]
                .zip(items_layout.children()) // [item_layout...]
                .find_map(|((item, tree), layout)|{
                    let item_state = tree.state.downcast_ref::<ItemState>();
                    item_state.open.then(|| (item, tree, layout))
                })
            {
                rec(bar, next_item, next_tree, next_layout, event, renderer, clipboard, shell, cursor, parent_bounds, items_bounds_list)
                    .merge(status)
            }else{
                status
            }
        }

        let bar = self.tree.state.downcast_mut::<MenuBarState>();
        let active_root = &mut self.roots[bar.active_root];
        let active_tree = &mut self.tree.children[bar.active_root];
        let mut items_bounds_list = vec![self.bar_bounds];

        rec(
            bar, 
            active_root, 
            active_tree, 
            layout,
            event, 
            renderer, 
            clipboard, 
            shell, 
            cursor, 
            self.parent_bounds,
            &mut items_bounds_list,
        )
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
        /* let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let mut current_item = &self.active_root;
        let mut current_tree = &self.active_tree; // item_tree: Tree{item_state, [widget_tree, menu_tree]}
        let mut current_layout = layout; // item_layout: Node{size:0, [widget_node, menu_node]}

        loop {
            let cm = current_item.menu.as_mut();
            let menu = cm.expect("No menu defined in this item");
            let mut lc = current_layout.children(); 
            let widget_layout = lc.next().unwrap(); 
            let menu_layout = lc.next().unwrap(); // menu_node: Node{size:inf, [items_layout, prescroll, offset_bounds, check_bounds]}
            let items_layout = menu_layout.children().next().unwrap(); // items_layout: Node{size:items_bounds, [item_layout...]}

            // let widget_tree = &mut current_tree.children[0];
            let menu_tree = &mut current_tree.children[1]; // menu_tree: Tree{ menu_state, [item_tree...] }
            
            menu.draw(&menu_tree, renderer, theme, style, menu_layout, cursor, &bar.viewport);

            if let Some((next_item, next_tree, next_layout)) = menu.items // [item...]
                .iter_mut()
                .zip(menu_tree.children.iter_mut()) // [item_tree...]
                .zip(items_layout.children()) // [item_layout...]
                .find_map(|((item, tree), layout)|{
                    let item_state = tree.state.downcast_ref::<ItemState>();
                    item_state.open.then(|| (item, tree, layout))
                })
            {
                current_item = next_item;
                current_tree = next_tree;
                current_layout = next_layout;
            }else{
                break;
            }
        } */
    
        fn rec<'a, Message, Theme, Renderer:renderer::Renderer>(
            bar: &MenuBarState,
            current_item: &Item<'a, Message, Theme, Renderer>,
            current_tree: &Tree,
            current_layout: &Layout<'_>,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            cursor: mouse::Cursor,
        ){
            let menu = current_item.menu.as_ref().expect("No menu defined in this item");
            let mut lc = current_layout.children(); 
            let _ = lc.next().unwrap(); // widget_node
            let menu_layout = lc.next().unwrap(); // menu_node: Node{size:inf, [items_layout, prescroll, offset_bounds, check_bounds]}
            let items_layout = menu_layout.children().next().unwrap(); // items_layout: Node{size:items_bounds, [item_layout...]}

            let menu_tree = &current_tree.children[1]; // menu_tree: Tree{ menu_state, [item_tree...] }
            let viewport = bar.viewport;

            menu.draw(&menu_tree, renderer, theme, style, menu_layout, cursor, &viewport);

            if let Some((next_item, next_tree, next_layout)) = menu.items // [item...]
                .iter()
                .zip(menu_tree.children.iter()) // [item_tree...]
                .zip(items_layout.children()) // [item_layout...]
                .find_map(|((item, tree), layout)|{
                    let item_state = tree.state.downcast_ref::<ItemState>();
                    item_state.open.then(|| (item, tree, layout))
                })
            {
                rec(bar, next_item, &next_tree, &next_layout, renderer, theme, style, cursor)
            }
        }

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let active_root = &self.roots[bar.active_root];
        let active_tree = &self.tree.children[bar.active_root];
        rec(
            bar,
            active_root,
            active_tree,
            &layout,
            renderer, 
            theme, 
            style, 
            cursor
        )
    }
}
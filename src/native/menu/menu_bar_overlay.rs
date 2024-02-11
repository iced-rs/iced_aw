use iced_widget::{core::{
    event,
    layout::{self, Limits, Node},
    mouse, overlay, renderer, touch,
    widget::tree::{self, Tree},
    Alignment, Border, Clipboard, Color, Element, Event, Layout, Length, Padding, Point, Rectangle,
    Shell, Size, Vector, Widget,
}, shader::wgpu::naga::Barrier};

use super::{menu_bar::MenuBarState, menu_tree::*, types::*};

pub(super) struct MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    // pub(super) active: usize,
    /// Tree{ bar_state, [item_tree...] }
    pub(super) tree: &'b mut Tree,
    // pub(super) bar: &'b mut MenuBarState,
    // pub(super) active_tree: &'b mut Tree,
    // pub(super) active_root: &'b mut Item<'a, Message, Theme, Renderer>,
    pub(super) roots: &'b mut [Item<'a, Message, Theme, Renderer>],
    
    // pub(super) bar_layout: Layout<'b>,
    pub(super) init_bar_bounds: Rectangle,
    pub(super) init_root_bounds: Vec<Rectangle>,
    // pub(super) init_parent_bounds: Option<Rectangle>,
}
impl<'a, 'b, Message, Theme, Renderer> MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn overlay_element(self) -> overlay::Element<'b, Message, Theme, Renderer> {
        overlay::Element::new(Point::ORIGIN, Box::new(self))
    }
}
impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
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
        println!();
        println!("mbo layout");
        // println!("translation: {:?}", translation);

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let bar_bounds = self.init_bar_bounds;
        // println!("bar bounds: {:?}", bar_bounds);

        let bar_node = Node::with_children(
            bar_bounds.size(), 
            [].into()
        ).move_to(bar_bounds.position() + translation);

        let roots_node = Node::with_children(
            Size::ZERO, 
            self.init_root_bounds.iter().map(|r| 
                Node::new(r.size()).move_to(r.position()) 
            ).collect()
        ).translate(translation);

        let Some(active) = bar.active_root else {
            println!("no active");
            return Node::with_children(
                bounds, 
                [
                    bar_node,
                    roots_node,
                ].into()
            )
        };

        let active_root = &mut self.roots[active];
        let active_tree = &mut self.tree.children[active]; // item_tree: Tree{ stateless, [ widget_tree, menu_tree ] }
        let parent_bounds = self.init_root_bounds[active] + translation;
        // println!("parent bounds: {:?}", parent_bounds);
        
        fn rec<'a, Message, Theme, Renderer: renderer::Renderer>(
            renderer: &Renderer,
            item: &Item<'a, Message, Theme, Renderer>,
            tree: &mut Tree,
            menu_nodes: &mut Vec<Node>,
            parent_bounds: Rectangle,
            parent_direction: (Direction, Direction),
            translation: Vector,
            // parent_offset: Vector,
            viewport: &Rectangle,
        ) {
            let menu = item.menu.as_ref().unwrap();
            let menu_tree = &mut tree.children[1];

            let (menu_node, direction) = menu.layout(menu_tree, renderer, &Limits::NONE, parent_bounds, parent_direction, translation, viewport); 
                // Node{inf, [ items_node, prescroll, offset_bounds, check_bounds ]}
            menu_nodes.push(menu_node);
            
            let menu_state = menu_tree.state.downcast_ref::<MenuState>();

            if let Some(active) = menu_state.active{
                let next_item = &menu.items[active];
                let next_tree = &mut menu_tree.children[active];
                let items_node = &menu_nodes.last().unwrap().children()[0];
                let next_parent_bounds = items_node.children()[active].bounds() + 
                    (items_node.bounds().position() - Point::ORIGIN);
                // let next_parent_direction = 
                // println!("items bounds: {:?}", items_node.bounds());
                // println!("next parent bounds: {:?}", next_parent_bounds);

                rec(renderer, next_item, next_tree, menu_nodes, next_parent_bounds, direction, translation, viewport);
            }
        }

        let mut menu_nodes = vec![];

        let parent_direction = {
            let hcenter = bounds.width / 2.0;
            let vcenter = bounds.height / 2.0;
    
            let phcenter = parent_bounds.x + parent_bounds.width / 2.0;
            let pvcenter = parent_bounds.y + parent_bounds.height / 2.0;

            (
                if phcenter < hcenter {
                    Direction::Positive
                } else {
                    Direction::Negative
                },
                if pvcenter < vcenter {
                    Direction::Positive
                } else {
                    Direction::Negative
                }
            )
        };

        rec(
            renderer,
            active_root,
            active_tree,
            &mut menu_nodes,
            parent_bounds,
            parent_direction,
            translation,
            &Rectangle::new(position, bounds),
        );

        Node::with_children(
            bounds, 
            [
                bar_node,
                roots_node,
                Node::with_children(Size::ZERO, menu_nodes),
            ].into()
        )
    }
    
    #[allow(unused_results)]
    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        use event::Status::*;

        println!();
        println!("mbo event");

        let viewport = layout.bounds();
        let mut lc = layout.children();
        let bar_bounds = lc.next().unwrap().bounds();
        let roots_layout = lc.next().unwrap();
        
        let bar = self.tree.state.downcast_mut::<MenuBarState>();

        // if cursor.is_over(bar_bounds){
        //     return Ignored;
        // }

        let Some(active) = bar.active_root else {
            return Ignored;
        };

        let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let Some(menu_layouts_layout) = lc.next() else { return Ignored; }; // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &mut self.roots[active];
        let active_tree = &mut self.tree.children[active];
        let mut prev_bounds_list = vec![bar_bounds];
        
        fn rec<'a, 'b, Message, Theme, Renderer: renderer::Renderer>(
            tree: &mut Tree,
            item: &mut Item<'a, Message, Theme, Renderer>,
            event: Event,
            layout_iter: &mut impl Iterator< Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            parent_bounds: Rectangle,
            viewport: &Rectangle,
            prev_bounds_list: &mut Vec<Rectangle>,
            prev: &mut Index,
        ) -> RecEvent {
            let menu = item.menu.as_mut().expect("No menu defined in this item");
            let menu_tree = &mut tree.children[1];

            let Some(menu_layout) = layout_iter.next() else { return RecEvent::None; }; // menu_node: Node{inf, [ items_node, prescroll, offset_bounds, check_bounds ]}

            let mut mc = menu_layout.children();
            let items_layout = mc.next().unwrap(); // items_node
            let prescroll = mc.next().unwrap().bounds();
            prev_bounds_list.push(prescroll);
            
            let menu_state = menu_tree.state.downcast_mut::<MenuState>();

            if let Some(active) = menu_state.active{
                let next_tree = &mut menu_tree.children[active];
                let next_item = &mut menu.items[active];
                let next_parent_bounds = items_layout.children().nth(active).unwrap().bounds();

                let re = rec(
                    next_tree, 
                    next_item, 
                    event.clone(), 
                    layout_iter, 
                    cursor, 
                    renderer, 
                    clipboard, 
                    shell, 
                    next_parent_bounds, 
                    viewport, 
                    prev_bounds_list, 
                    &mut menu_state.active
                );
                
                prev_bounds_list.pop();

                match re {
                    RecEvent::Event => RecEvent::Event,
                    RecEvent::Close => {
                        menu.close_event(menu_tree, menu_layout, cursor, parent_bounds, prev_bounds_list, prev);
                        if prev.is_some(){
                            RecEvent::None
                        }else{
                            RecEvent::Close
                        }
                    }
                    RecEvent::None => {
                        if cursor.is_over(prescroll){
                            menu.on_event(menu_tree, event, menu_layout, cursor, renderer, clipboard, shell, viewport);
                            menu.open_event(menu_tree, menu_layout, cursor);
                            RecEvent::Event
                        }else{
                            RecEvent::None
                        }
                    }
                }
            }else{
                prev_bounds_list.pop();

                if cursor.is_over(prescroll){
                    menu.on_event(menu_tree, event, menu_layout, cursor, renderer, clipboard, shell, viewport);
                    menu.open_event(menu_tree, menu_layout, cursor);
                    RecEvent::Event
                }else{
                    menu.close_event(menu_tree, menu_layout, cursor, parent_bounds, prev_bounds_list, prev);
                    if prev.is_some(){
                        RecEvent::None
                    }else{
                        RecEvent::Close
                    }
                }
            }
        }
        
        rec(
            active_tree, 
            active_root, 
            event, 
            &mut menu_layouts,
            cursor, 
            renderer, 
            clipboard, 
            shell, 
            parent_bounds, 
            &viewport, 
            &mut prev_bounds_list,
            &mut bar.active_root,
        );

        if cursor.is_over(bar_bounds){
            Ignored
        }else{
            Captured
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let viewport = layout.bounds();
        let mut lc = layout.children();
        let _bar_bounds = lc.next().unwrap().bounds();
        let roots_layout = lc.next().unwrap();

        let bar = self.tree.state.downcast_ref::<MenuBarState>();

        let Some(active) = bar.active_root else {
            return;
        };

        let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &self.roots[active];
        let active_tree = &self.tree.children[active];

        /* println!("bar bounds: {:?}", bar_bounds);
        renderer.fill_quad(
            renderer::Quad{
                bounds: bar_bounds,
                border: Border{
                    // width: 6.0,
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            Color::from([0.0, 1.0, 1.0, 0.3])
        );

        roots_layout.children().for_each(|l|{
            println!("root bounds: {:?}", l.bounds());
            renderer.fill_quad(
                renderer::Quad{
                    bounds: l.bounds(),
                    ..Default::default()
                }, 
                Color::from([1.0, 0.0, 0.0, 0.6])
            );
        }); */

        fn rec<'a, 'b, Message, Theme, Renderer: renderer::Renderer>(
            tree: &Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator< Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            parent_bounds: Rectangle,
            viewport: &Rectangle,
        ) {
            let menu = item.menu.as_ref().expect("No menu defined in this item");
            let menu_tree = &tree.children[1];

            let Some(menu_layout) = layout_iter.next() else { return }; // menu_node: Node{inf, [ items_node, prescroll, offset_bounds, check_bounds ]}
            
            menu.draw(
                menu_tree, 
                renderer, 
                theme, 
                style, 
                menu_layout, 
                cursor, 
                viewport, 
                parent_bounds
            );

            let items_layout = menu_layout.children().next().unwrap(); // items_node
            
            let menu_state = menu_tree.state.downcast_ref::<MenuState>();

            if let Some(active) = menu_state.active{
                let next_tree = &menu_tree.children[active];
                let next_item = &menu.items[active];
                let next_parent_bounds = items_layout.children().nth(active).unwrap().bounds();
                renderer.with_layer(
                    *viewport, 
                    |r| rec(
                        next_tree, 
                        next_item, 
                        layout_iter, 
                        cursor, 
                        r, 
                        theme, 
                        style, 
                        next_parent_bounds, 
                        viewport
                    )
                );
            }
        }
    
        rec(
            active_tree, 
            active_root, 
            &mut menu_layouts, 
            cursor, 
            renderer, 
            theme, 
            style, 
            parent_bounds, 
            &viewport
        )
    
    }

    fn is_over(
        &self, 
        layout: Layout<'_>, 
        _renderer: &Renderer, 
        cursor_position: Point
    ) -> bool {
        // let viewport = layout.bounds();
        // let mut lc = layout.children();
        // let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        // let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]
        // // let parent_bounds = lc.next().unwrap().bounds();

        // let isover = menu_layouts.any(|l|{
        //     let items_layout = l.children().next().unwrap();
        //     items_layout.bounds().contains(cursor_position)
        // });
        // // if isover{
        // //     println!("is_over");
        // // }
        // isover
        false
    }
}

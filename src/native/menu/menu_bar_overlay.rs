use iced_widget::core::{
    event,
    layout::{Limits, Node},
    mouse, overlay, renderer,
    widget::tree::Tree,
    Alignment, Border, Clipboard, Color, Element, Event, Layout, Length, Padding, Point, Rectangle,
    Shell, Size, Vector, Widget,
};

use super::{menu_bar::MenuBarState, menu_tree::*, common::*};
use crate::style::menu_bar::*;

/// Should be returned from the recursive event processing function, 
/// tells the caller which type of event has been processed
enum RecEvent{
    Event, 
    Close,
    None 
}

pub(super) struct MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    /// Tree{ bar_state, [item_tree...] }
    pub(super) tree: &'b mut Tree,

    pub(super) roots: &'b mut [Item<'a, Message, Theme, Renderer>],
    pub(super) init_bar_bounds: Rectangle,
    pub(super) init_root_bounds: Vec<Rectangle>,
    pub(super) check_bounds_width: f32,
    pub(super) style: &'b Theme::Style,
}
impl<'a, 'b, Message, Theme, Renderer> MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    pub(super) fn overlay_element(self) -> overlay::Element<'b, Message, Theme, Renderer> {
        overlay::Element::new(Point::ORIGIN, Box::new(self))
    }
}
impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
        translation: Vector,
    ) -> Node {
        // println!();
        // println!("mbo layout");

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let bar_bounds = self.init_bar_bounds;

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
            // println!("no active");
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
        
        fn rec<'a, Message, Theme: StyleSheet, Renderer: renderer::Renderer>(
            renderer: &Renderer,
            item: &Item<'a, Message, Theme, Renderer>,
            tree: &mut Tree,
            menu_nodes: &mut Vec<Node>,
            check_bounds_width: f32,
            parent_bounds: Rectangle,
            parent_direction: (Direction, Direction),
            // translation: Vector,
            // parent_offset: Vector,
            viewport: &Rectangle,
        ) {
            let menu = item.menu.as_ref().unwrap();
            let menu_tree = &mut tree.children[1];

            let (menu_node, direction) = menu.layout(
                menu_tree, 
                renderer, 
                &Limits::NONE, 
                check_bounds_width, 
                parent_bounds, 
                parent_direction, 
                viewport
            ); 
                // Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}
            menu_nodes.push(menu_node);
            
            let menu_state = menu_tree.state.downcast_ref::<MenuState>();

            if let Some(active) = menu_state.active{
                let next_item = &menu.items[active];
                let next_tree = &mut menu_tree.children[active];
                let next_parent_bounds = {
                    let slice_node = &menu_nodes.last().unwrap().children()[0];
                    let Some(node) = slice_node.children().get(active - menu_state.slice.start_index) 
                    else { return; };

                    node.bounds() + (slice_node.bounds().position() - Point::ORIGIN)
                };
                rec(
                    renderer, 
                    next_item, 
                    next_tree, 
                    menu_nodes, 
                    check_bounds_width, 
                    next_parent_bounds, 
                    direction, 
                    viewport
                );
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
            self.check_bounds_width,
            parent_bounds,
            parent_direction,
            // translation,
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

        // println!();
        // println!("mbo event");

        let viewport = layout.bounds();
        let mut lc = layout.children();
        let bar_bounds = lc.next().unwrap().bounds();
        let roots_layout = lc.next().unwrap();
        
        let bar = self.tree.state.downcast_mut::<MenuBarState>();

        let Some(active) = bar.active_root else {
            return Ignored;
        };

        let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let Some(menu_layouts_layout) = lc.next() else { return Ignored; }; // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &mut self.roots[active];
        let active_tree = &mut self.tree.children[active];
        let mut prev_bounds_list = vec![bar_bounds];
        
        fn rec<'a, 'b, Message, Theme: StyleSheet, Renderer: renderer::Renderer>(
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

            let Some(menu_layout) = layout_iter.next() else { return RecEvent::None; }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}

            let mut mc = menu_layout.children();
            let slice_layout = mc.next().unwrap(); // slice_node
            let prescroll = mc.next().unwrap().bounds();
            let offset_bounds = mc.next().unwrap().bounds();
            prev_bounds_list.push(prescroll);
            
            let menu_state = menu_tree.state.downcast_mut::<MenuState>();

            if let Some(active) = menu_state.active{
                let next_tree = &mut menu_tree.children[active];
                let next_item = &mut menu.items[active];
                let next_parent_bounds = {
                    let Some(layout) = slice_layout.children().nth(active - menu_state.slice.start_index) 
                    else { return RecEvent::Event };
                    
                    layout.bounds()
                };

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
                        if cursor.is_over(prescroll){
                            menu.on_event(menu_tree, event, menu_layout, cursor, renderer, clipboard, shell, viewport);
                            menu.open_event(menu_tree, menu_layout, cursor);
                            RecEvent::Event
                        }else if cursor.is_over(offset_bounds){
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
                    RecEvent::None => {
                        if cursor.is_over(prescroll){
                            menu.on_event(menu_tree, event, menu_layout, cursor, renderer, clipboard, shell, viewport);
                            menu.open_event(menu_tree, menu_layout, cursor);
                            RecEvent::Event
                        }else if cursor.is_over(offset_bounds){
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
                }else if cursor.is_over(offset_bounds){
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
        
        let re = rec(
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

        match re {
            RecEvent::Event => {
                Captured
            },
            RecEvent::Close | RecEvent::None => {
                if cursor.is_over(bar_bounds){
                    Ignored
                }else{
                    Captured
                }
            },
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
        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active) = bar.active_root else { return; };

        let viewport = layout.bounds();
        let mut lc = layout.children();
        let _bar_bounds = lc.next().unwrap().bounds();
        let _roots_layout = lc.next().unwrap();

        // let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &self.roots[active];
        let active_tree = &self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: StyleSheet, Renderer: renderer::Renderer>(
            tree: &Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator< Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            theme_style: &Theme::Style,
            viewport: &Rectangle,
        ) {
            let menu = item.menu.as_ref().expect("No menu defined in this item");
            let menu_tree = &tree.children[1];

            let Some(menu_layout) = layout_iter.next() else { return }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}
            
            let mut draw_menu = |cursor| 
                menu.draw(
                    menu_tree, 
                    renderer, 
                    theme, 
                    style, 
                    theme_style,
                    menu_layout, 
                    cursor, 
                    viewport, 
                );
            
            let menu_state = menu_tree.state.downcast_ref::<MenuState>();
            
            if let Some(active) = menu_state.active{
                let next_tree = &menu_tree.children[active];
                let next_item = &menu.items[active];

                let mut mc = menu_layout.children();
                let slice_layout = mc.next().unwrap(); // slice_node
                let active_bounds = {
                    let Some(layout) = slice_layout.children().nth(active - menu_state.slice.start_index)
                    else { return; };
                    layout.bounds()
                };

                draw_menu(
                    if cursor.is_over(active_bounds){
                        cursor
                    }else{
                        mouse::Cursor::Available(active_bounds.center())
                    }
                );

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
                        theme_style,
                        viewport
                    )
                );
            }else{
                draw_menu(cursor)
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
            self.style,
            &viewport
        )
    
    }

    fn is_over(
        &self, 
        _layout: Layout<'_>, 
        _renderer: &Renderer, 
        _cursor_position: Point
    ) -> bool {
        false
    }
}
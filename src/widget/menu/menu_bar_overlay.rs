#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::similar_names)]

use iced::{
    advanced::{
        layout::{Limits, Node},
        mouse, overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    }, time::Instant, window::{self, RedrawRequest}, Event, Padding, Point, Rectangle, Size, Vector
};

use super::{common::*, menu_bar::{MenuBar, MenuBarState}, menu_tree::*};
use crate::style::{menu_bar::*, Status};

pub(super) struct MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) menu_bar: &'b mut MenuBar<'a, Message, Theme, Renderer>,
    pub(super) layout: Layout<'b>,
    pub(super) translation: Vector,
    /// Tree{ bar_state, [item_tree...] }
    pub(super) tree: &'b mut Tree,
}
impl<'a, 'b, Message, Theme, Renderer> MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) fn overlay_element(self) -> overlay::Element<'b, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
    }
}
impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for MenuBarOverlay<'_, '_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        println!("MenuBarOverlay::layout()");
        let translation = self.translation;

        let bar_bounds = self.layout.bounds();
        let root_bounds = self.layout.children().map(|l| l.bounds()).collect::<Vec<_>>();

        let bar = self.tree.state.downcast_ref::<MenuBarState>();

        let bar_node = Node::with_children(bar_bounds.size(), [].into())
            .move_to(bar_bounds.position() + translation);

        let roots_node = Node::with_children(
            Size::ZERO,
            root_bounds
                .iter()
                .map(|r| Node::new(r.size()).move_to(r.position()))
                .collect(),
        )
        .translate(translation);

        let Some(active) = bar.active_root else {
            return Node::with_children(bounds, [bar_node, roots_node].into());
        };

        let active_root = &mut self.menu_bar.roots[active];
        let active_tree = &mut self.tree.children[active]; // item_tree: Tree{ stateless, [ widget_tree, menu_tree ] }
        let parent_bounds = root_bounds[active] + translation;

        fn rec<Message, Theme: Catalog, Renderer: renderer::Renderer>(
            renderer: &Renderer,
            item: &Item<'_, Message, Theme, Renderer>,
            tree: &mut Tree,
            menu_nodes: &mut Vec<Node>,
            parent_bounds: Rectangle,
            parent_direction: (Direction, Direction),
            viewport: &Rectangle,
        ) {
            if let Some(menu) = item.menu.as_ref() {
                let menu_tree = &mut tree.children[1];

                let (menu_node, direction) = menu.layout(
                    menu_tree,
                    renderer,
                    &Limits::NONE,
                    parent_bounds,
                    parent_direction,
                    viewport,
                );
                // Node{inf, [ slice_node, prescroll, offset_bounds]}
                menu_nodes.push(menu_node);

                let menu_state = menu_tree.state.downcast_ref::<MenuState>();

                if let Some(active) = menu_state.active {
                    let next_item = &menu.items[active];
                    let next_tree = &mut menu_tree.children[active];
                    let next_parent_bounds = {
                        let slice_node = &menu_nodes.last().unwrap().children()[0];
                        let active_in_slice = active - menu_state.slice.start_index;
                        let node = &slice_node.children()[active_in_slice];
                        node.bounds() + (slice_node.bounds().position() - Point::ORIGIN)
                    };
                    rec(
                        renderer,
                        next_item,
                        next_tree,
                        menu_nodes,
                        next_parent_bounds,
                        direction,
                        viewport,
                    );
                }
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
                },
            )
        };

        rec(
            renderer,
            active_root,
            active_tree,
            &mut menu_nodes,
            parent_bounds,
            parent_direction,
            &Rectangle::new(Point::ORIGIN, bounds),
        );

        Node::with_children(
            bounds,
            [
                bar_node,
                roots_node,
                Node::with_children(Size::ZERO, menu_nodes),
            ]
            .into(),
        )
    }

    #[allow(unused_results)]
    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        println!("MenuBarOverlay::update()");
        let viewport = layout.bounds();
        let mut lc = layout.children();
        let bar_bounds = lc.next().unwrap().bounds();
        let roots_layout = lc.next().unwrap();

        let bar = self.tree.state.downcast_mut::<MenuBarState>();

        let Some(active) = bar.active_root else {
            return;
        };

        let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let Some(menu_layouts_layout) = lc.next() else {
            return;
        }; // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &mut self.menu_bar.roots[active];
        let active_tree = &mut self.tree.children[active];
        let mut prev_bounds_list = vec![bar_bounds];

        #[rustfmt::skip]
        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            global_parameters: &GlobalParameters<'a, Theme>,
            tree: &mut Tree,
            item: &mut Item<'a, Message, Theme, Renderer>,
            event: &Event,
            layout_iter: &mut impl Iterator<Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            parent_bounds: Rectangle,
            viewport: &Rectangle,
            prev_bounds_list: &mut Vec<Rectangle>,
            prev: &mut Index,
            depth: usize,
        ) -> RecEvent {
            let Some(menu) = item.menu.as_mut() else {
                return RecEvent::Close;
            };
            let menu_tree = &mut tree.children[1];

            let Some(menu_layout) = layout_iter.next() else {
                return RecEvent::Close;
            }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds]}

            let mut mc = menu_layout.children();
            let slice_layout = mc.next().unwrap(); // slice_node
            let prescroll = mc.next().unwrap().bounds();
            let offset_bounds = mc.next().unwrap().bounds();
            let background_bounds = pad_rectangle(prescroll, menu.padding);
            let check_bounds = pad_rectangle(background_bounds, Padding::new(global_parameters.check_bounds_width));

            prev_bounds_list.push(background_bounds);

            let menu_state = menu_tree.state.downcast_mut::<MenuState>();
            let is_pressed = menu_state.pressed;

            let rec_event = if let Some(active) = menu_state.active {
                let next_tree = &mut menu_tree.children[active];
                let next_item = &mut menu.items[active];
                let active_in_slice = active - menu_state.slice.start_index;
                let next_parent_bounds = slice_layout
                    .children()
                    .nth(active_in_slice)
                    .expect(&format!("Index {:?} is not within the slice layout \
                        | slice_layout.children().count(): {:?} \
                        | This should not happen, please report this issue
                        ", 
                        active_in_slice,
                        slice_layout.children().count()
                    ))
                    .bounds();

                rec(
                    global_parameters,
                    next_tree,
                    next_item,
                    event,
                    layout_iter,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    next_parent_bounds,
                    viewport,
                    prev_bounds_list,
                    &mut menu_state.active,
                    depth + 1,
                )
            } else {
                RecEvent::Close
            };

            prev_bounds_list.pop();

            println!();
            println!("MenuBarOverlay | depth: {:?}", depth);
            println!("MenuBarOverlay | menu_state.active: {:?}", menu_state.active);
            println!("MenuBarOverlay | menu_state.pressed: {:?}", menu_state.pressed);
            println!("MenuBarOverlay | menu_state.slice: {:?}", menu_state.slice);
            println!("MenuBarOverlay | rec_event: {:?}", rec_event);
            println!("MenuBarOverlay | event: {:?}", event);
            println!("MenuBarOverlay | cursor: {:?}", cursor);
            println!("MenuBarOverlay | cursor is over background bounds: {:?}", cursor.is_over(background_bounds));
            println!("MenuBarOverlay | shell.is_event_captured(): {:?}", shell.is_event_captured());
            

            match rec_event {
                RecEvent::Event => {
                    let active = menu_state.active.expect("
                        `RecEvent::Event` is returned, but `menu_state.active` is `None`. \
                        This should not happen, please report this issue
                    ");

                    let cursor = match &global_parameters.draw_path {
                        DrawPath::FakeHovering => {
                            let active_in_slice = active - menu_state.slice.start_index;
                            let center = slice_layout
                                .children()
                                .nth(active_in_slice)
                                .expect(&format!(" Index {:?} is not within the slice layout \
                                    | slice_layout.children().count(): {:?} \
                                    | This should not happen, please report this issue
                                    ", 
                                    active_in_slice,
                                    slice_layout.children().count()
                                ))
                                .bounds()
                                .center();
                            mouse::Cursor::Available(center)
                        },
                        DrawPath::Backdrop => mouse::Cursor::Unavailable
                    };

                    menu.fake_update(menu_tree, menu_layout, cursor, renderer, clipboard, shell, viewport);

                    shell.capture_event();
                    RecEvent::Event
                }
                RecEvent::Close => {
                    if is_pressed || cursor.is_over(background_bounds) || cursor.is_over(offset_bounds){
                        menu.update(
                            menu_tree,
                            event,
                            menu_layout,
                            cursor,
                            renderer,
                            clipboard,
                            shell,
                            viewport,
                            global_parameters.scroll_speed,
                        );
                        menu.open_event(menu_tree, menu_layout, cursor, shell);
                        shell.capture_event();
                        RecEvent::Event
                    } else if cursor.is_over(parent_bounds){
                        menu.fake_update(menu_tree, menu_layout, cursor, renderer, clipboard, shell, viewport);
                        // the cursor is over the parent bounds
                        // let the parent process the event
                        assert_eq!(shell.is_event_captured(), false, "Returning RecEvent::None");
                        RecEvent::None
                    } else {
                        menu.close_event(
                            menu_tree, 
                            menu_layout, 
                            cursor, 
                            shell,
                            check_bounds, 
                            background_bounds, 
                            parent_bounds, 
                            prev_bounds_list, 
                            prev
                        );
                        if prev.is_some() {
                            // the current menu is not ready to close
                            shell.capture_event();
                            RecEvent::Event
                        } else {
                            assert_eq!(shell.is_event_captured(), false, "Returning RecEvent::Close");
                            // the current menu has closed itself
                            RecEvent::Close
                        }
                    }
                }
                RecEvent::None => {
                    menu.update(
                        menu_tree,
                        event,
                        menu_layout,
                        cursor,
                        renderer,
                        clipboard,
                        shell,
                        viewport,
                        global_parameters.scroll_speed,
                    );
                    shell.capture_event();
                    RecEvent::Event
                }
            }
        }

        let re = rec(
            &self.menu_bar.global_parameters,
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
            0,
        );

        if let (true, Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) 
        = (self.menu_bar.global_parameters.close_on_click, event) {
            bar.active_root = None;
            shell.invalidate_layout();
        }

        // if the function reaches here, 
        // that means bar.active_root was Some, 
        // if bar.active_root is None, 
        // we can be sure that the layout has changed
        if bar.active_root.is_none() && !cursor.is_over(bar_bounds){
            bar.open = false;
            bar.is_pressed = false;
            shell.invalidate_layout();
        }
        
        match re {
            RecEvent::Event => {
                let redraw_event = Event::Window(window::Event::RedrawRequested(Instant::now()));
                let mut fake_messages = vec![];
                let mut fake_shell = Shell::new(&mut fake_messages);

                let Self { menu_bar, layout, tree, .. } = self;
                let draw_path = menu_bar.global_parameters.draw_path;
                let cursor = match draw_path {
                    DrawPath::FakeHovering => {
                        let center = parent_bounds.center();
                        mouse::Cursor::Available(center)
                    }
                    DrawPath::Backdrop => mouse::Cursor::Unavailable
                };

                println!("MenuBarOverlay::update() | calling MenuBar::update_items()");
                menu_bar.update_items(
                    tree, 
                    &redraw_event, 
                    *layout, 
                    cursor, 
                    renderer, 
                    clipboard, 
                    &mut fake_shell, 
                    &viewport
                );
            },
            _ => {
                // if RecEvent::Close or RecEvent::None is returned,
                // let the menu bar process the event
            }
        }

        println!("MenuBarOverlay::update() | shell.is_layout_invalid(): {:?}", shell.is_layout_invalid());
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        println!("MenuBarOverlay::mouse_interaction()");
        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active) = bar.active_root else {
            return mouse::Interaction::default();
        };

        // let viewport = layout.bounds();
        let mut lc = layout.children();
        let _bar_bounds = lc.next().unwrap().bounds();
        let _roots_layout = lc.next().unwrap();

        // let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &self.menu_bar.roots[active];
        let active_tree = &self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            tree: &Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator<Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            let Some(menu) = item.menu.as_ref() else {
                return mouse::Interaction::default();
            };
            let menu_tree = &tree.children[1];

            let Some(menu_layout) = layout_iter.next() else {
                println!("MenuBarOverlay::mouse_interaction() | menu exists, but menu_layout is None");
                return mouse::Interaction::default();
            }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds]}

            let menu_state = menu_tree.state.downcast_ref::<MenuState>();

            let i = menu.mouse_interaction(menu_tree, menu_layout, cursor, renderer);

            menu_state.active.map_or(i, |active| {
                let next_tree = &menu_tree.children[active];
                let next_item = &menu.items[active];
                rec(next_tree, next_item, layout_iter, cursor, renderer).max(i)
            })
        }

        rec(
            active_tree,
            active_root,
            &mut menu_layouts,
            cursor,
            renderer,
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active) = bar.active_root else {
            return;
        };

        // let viewport = layout.bounds();
        let mut lc = layout.children();
        let _bar_bounds = lc.next().unwrap().bounds();
        let _roots_layout = lc.next().unwrap();

        // let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &self.menu_bar.roots[active];
        let active_tree = &mut self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            tree: &mut Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator<Item = Layout<'b>>,
            renderer: &Renderer,
            operation: &mut dyn Operation<()>,
        ) {
            let Some(menu) = item.menu.as_ref() else {
                return;
            };

            let menu_tree = &mut tree.children[1];

            let Some(menu_layout) = layout_iter.next() else {
                return;
            };

            menu.operate(menu_tree, menu_layout, renderer, operation);

            operation.container(None, menu_layout.bounds(), &mut |operation| {
                menu.items
                    .iter() // [Item...]
                    .zip(menu_tree.children.iter_mut()) // [item_tree...] // [widget_node...]
                    .for_each(|(child, state)| {
                        rec(state, child, layout_iter, renderer, operation);
                    });
            });
        }

        rec(
            active_tree,
            active_root,
            &mut menu_layouts,
            renderer,
            operation,
        );
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        println!("MenuBarOverlay::overlay()");
        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let active = bar.active_root?;
        let mut lc = layout.children();
        let _bar_bounds = lc.next()?.bounds();
        let _roots_layout = lc.next()?;
        let menu_layouts_layout = lc.next()?; // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]
        let active_root = &mut self.menu_bar.roots[active];
        let active_tree = &mut self.tree.children[active];
        let menu = active_root.menu.as_mut()?;
        let menu_tree = &mut active_tree.children[1];
        let menu_layout = menu_layouts.next()?;

        menu.overlay(menu_tree, menu_layout, renderer, Vector::ZERO)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        println!("MenuBarOverlay::draw()");
        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active) = bar.active_root else {
            return;
        };

        let viewport = layout.bounds();
        let mut lc = layout.children();
        let _bar_bounds = lc.next().unwrap().bounds();
        let _roots_layout = lc.next().unwrap();

        // let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &self.menu_bar.roots[active];
        let active_tree = &self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            global_parameters: &GlobalParameters<'a, Theme>,
            tree: &Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator<Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            theme_style: &Style,
            viewport: &Rectangle,
        ) {
            let Some(menu) = item.menu.as_ref() else {
                return;
            };

            let menu_tree = &tree.children[1];

            let Some(menu_layout) = layout_iter.next() else {
                println!("MenuBarOverlay::draw() | menu exists, but menu_layout is None");
                return;
            }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds]}

            let menu_state = menu_tree.state.downcast_ref::<MenuState>();

            menu.draw(
                &global_parameters.draw_path,
                menu_tree,
                renderer,
                theme,
                style,
                theme_style,
                menu_layout,
                cursor,
                viewport,
            );

            if let Some(active) = menu_state.active {
                let next_tree = &menu_tree.children[active];
                let next_item = &menu.items[active];

                renderer.with_layer(*viewport, |r| {
                    rec(
                        global_parameters,
                        next_tree,
                        next_item,
                        layout_iter,
                        cursor,
                        r,
                        theme,
                        style,
                        theme_style,
                        viewport,
                    );
                });
            }
        }

        let theme_style = theme.style(&self.menu_bar.global_parameters.class, Status::Active);

        rec(
            &self.menu_bar.global_parameters,
            active_tree,
            active_root,
            &mut menu_layouts,
            cursor,
            renderer,
            theme,
            style,
            &theme_style,
            &viewport,
        );
    }
}


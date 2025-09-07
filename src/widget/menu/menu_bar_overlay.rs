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
        Clipboard, Layout, Shell,
    },
    time::Instant,
    window, Event, Point, Rectangle, Size, Vector,
};

use super::{common::*, menu_bar::*, menu_tree::*};
use crate::style::{menu_bar::*, Status};

#[cfg(feature = "debug_log")]
use log::{debug, trace, warn};

pub(super) struct MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub(super) menu_bar: &'b mut MenuBar<'a, Message, Theme, Renderer>,
    pub(super) layout: Layout<'b>,
    pub(super) translation: Vector,
    /// Tree{ bar, [item_tree...] }
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
    /// out: Node{inf, [ bar_node, roots_node, menu_nodes_node{0, [ menu_node,...]} ]}
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::layout", "");
        let translation = self.translation;

        let bar_bounds = self.layout.bounds();
        let root_bounds = self
            .layout
            .children()
            .map(|l| l.bounds())
            .collect::<Vec<_>>();

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

        let Some(bar_menu_state) = bar.menu_state.as_ref() else {
            return Node::with_children(bounds, [bar_node, roots_node].into());
        };

        let Some(active) = bar_menu_state.active else {
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
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::update", "");

        let bar = self.tree.state.downcast_mut::<MenuBarState>();
        let MenuBarState {
            global_state,
            menu_state: bar_state,
        } = bar;

        let Some(bar_menu_state) = bar_state.as_mut() else {
            return;
        };

        let Some(active) = bar_menu_state.active else {
            return;
        };

        let viewport = layout.bounds();
        let mut lc = layout.children();
        let bar_bounds = lc.next().unwrap().bounds();
        let roots_layout = lc.next().unwrap();

        let parent_bounds = roots_layout.children().nth(active).unwrap().bounds();
        let menu_layouts_layout = lc.next().unwrap(); // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        let active_root = &mut self.menu_bar.roots[active];
        let active_tree = &mut self.tree.children[active];
        let mut prev_bounds_list = vec![bar_bounds];

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                global_state.pressed = true;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                global_state.pressed = false;
                shell.request_redraw();
            }
            _ => {}
        }

        #[rustfmt::skip]
        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            global_state: &mut GlobalState,
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
            prev_active: &mut Index,
            depth: usize,
        ) -> RecEvent {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBarOverlay::update", "rec");

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
            let background_bounds = pad_rectangle(prescroll, menu.padding);

            prev_bounds_list.push(background_bounds);

            let menu_state = menu_tree.state.downcast_mut::<MenuState>();

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
                    global_state,
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
            } else if let mouse::Cursor::Unavailable = cursor{
                // there no child menu open
                // and the cursor is unavailable
                // this means there is an overlay from one of the items open
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::update", "rec | cursor is unavailable");
                RecEvent::Event
            } else {
                RecEvent::Close
            };

            prev_bounds_list.pop();

            #[cfg(feature = "debug_log")]
            {
                debug!(target:"menu::MenuBarOverlay::update", "");
                debug!(target:"menu::MenuBarOverlay::update", "depth: {:?}", depth);
                debug!(target:"menu::MenuBarOverlay::update", "menu_state.active: {:?}", menu_state.active);
                debug!(target:"menu::MenuBarOverlay::update", "menu_state.slice: {:?}", menu_state.slice);
                debug!(target:"menu::MenuBarOverlay::update", "rec_event: {:?}", rec_event);
                debug!(target:"menu::MenuBarOverlay::update", "event: {:?}", event);
                debug!(target:"menu::MenuBarOverlay::update", "cursor: {:?}", cursor);
                debug!(target:"menu::MenuBarOverlay::update", "cursor is over background bounds: {:?}", cursor.is_over(background_bounds));
                debug!(target:"menu::MenuBarOverlay::update", "shell.is_event_captured(): {:?}", shell.is_event_captured());
            }

            menu.update(
                global_state,
                global_parameters,
                rec_event,
                menu_tree,
                event,
                menu_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
                parent_bounds,
                prev_bounds_list,
                prev_active,
            )
        }

        let re = rec(
            global_state,
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
            &mut bar_menu_state.active,
            0,
        );

        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::update", "re: {:?}", re);

        if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            if let Some(MenuBarTask::CloseOnClick) = global_state.task() {
                bar.close(self.tree.children.as_mut_slice(), shell);
                return;
            }
        }

        match re {
            RecEvent::Event => {
                let redraw_event = Event::Window(window::Event::RedrawRequested(Instant::now()));
                let mut fake_messages = vec![];
                let mut fake_shell = Shell::new(&mut fake_messages);

                let Self {
                    menu_bar,
                    layout,
                    tree,
                    ..
                } = self;
                let draw_path = menu_bar.global_parameters.draw_path;
                let cursor = match draw_path {
                    DrawPath::FakeHovering => {
                        let center = parent_bounds.center();
                        mouse::Cursor::Available(center - self.translation)
                    }
                    DrawPath::Backdrop => mouse::Cursor::Unavailable,
                };

                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::update", "calling update_items() on MenuBar");
                update_items(
                    menu_bar.roots.as_mut_slice(),
                    tree.children.as_mut_slice(),
                    layout.children(),
                    &redraw_event,
                    cursor,
                    renderer,
                    clipboard,
                    &mut fake_shell,
                    &viewport,
                );
            }
            RecEvent::Close => {
                if !cursor.is_over(bar_bounds) {
                    bar.close(self.tree.children.as_mut_slice(), shell);
                }

                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::update", "RecEvent::Close | MenuBar should process the event");

                assert!(
                    shell.is_event_captured() == false,
                    "MenuBarOverlay::update() | RecEvent::Close | Returning"
                );
                // let the menu bar process the event
            }
            RecEvent::None => {
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::update", "RecEvent::None | MenuBar should process the event");

                assert!(
                    shell.is_event_captured() == false,
                    "MenuBarOverlay::update() | RecEvent::None | Returning"
                );
                // let the menu bar process the event
            }
        }

        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::update", "returning | shell.is_layout_invalid(): {:?}", shell.is_layout_invalid());
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::mouse_interaction", "");

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(bar_menu_state) = bar.menu_state.as_ref() else {
            return mouse::Interaction::default();
        };

        let Some(active) = bar_menu_state.active else {
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
                #[cfg(feature = "debug_log")]
                warn!(target:"menu::MenuBarOverlay::mouse_interaction", "menu exists, but menu_layout is None");
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
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::operate", "");

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(bar_menu_state) = bar.menu_state.as_ref() else {
            return;
        };
        let Some(active) = bar_menu_state.active else {
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
                #[cfg(feature = "debug_log")]
                warn!(target:"menu::MenuBarOverlay::operate", "menu exists, but menu_layout is None");
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
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::overlay", "");

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let bar_menu_state = bar.menu_state.as_ref()?;
        let active = bar_menu_state.active?;

        let mut lc = layout.children();
        let viewport = layout.bounds();
        let _bar_bounds = lc.next()?.bounds();
        let _roots_layout = lc.next()?;
        let menu_layouts_layout = lc.next()?; // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            items: &'b mut [Item<'a, Message, Theme, Renderer>],
            menu_tree: &'b mut Tree, // Tree{ menu_state, [item_tree...] }
            menu_layouts: &mut impl Iterator<Item = Layout<'b>>, // [menu_node...]
            overlays: &mut Vec<overlay::Element<'b, Message, Theme, Renderer>>,
            renderer: &Renderer,
            viewport: &Rectangle,
        ) {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBarOverlay::overlay", "rec");

            let menu_state = menu_tree.state.downcast_mut::<MenuState>();
            let menu_layout = menu_layouts.next().unwrap(); // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds]}
            let mut mlc = menu_layout.children();
            let slice_layout = mlc.next().unwrap(); // slice_node: Node{inf, [item_node...]}
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBarOverlay::overlay", "rec | slice_layout.children : {}", slice_layout.children().count());

            let slice = &menu_state.slice;

            if let Some(active) = menu_state.active {
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::overlay", "rec | active");

                let slice_index = active - slice.start_index;

                let mut next = None;

                for (i, ((item, item_tree), item_layout)) in items
                    [slice.start_index..=slice.end_index]
                    .iter_mut()
                    .zip(menu_tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
                    .zip(slice_layout.children())
                    .enumerate()
                {
                    #[cfg(feature = "debug_log")]
                    trace!(target:"menu::MenuBarOverlay::overlay", "rec | i: {i}");

                    let Item {
                        item: item_widget,
                        menu: item_menu,
                        ..
                    } = item;
                    let [item_widget_tree, item_menu_tree] = item_tree.children.as_mut_slice()
                    else {
                        continue;
                    };

                    if i == slice_index {
                        #[cfg(feature = "debug_log")]
                        trace!(target:"menu::MenuBarOverlay::overlay", "rec | i == slice_index {slice_index}");
                        next = Some((item_menu.as_mut().unwrap(), item_menu_tree));
                    }
                    if let Some(overlay) = item_widget.as_widget_mut().overlay(
                        item_widget_tree,
                        item_layout,
                        renderer,
                        viewport,
                        Vector::ZERO,
                    ) {
                        #[cfg(feature = "debug_log")]
                        debug!(target:"menu::MenuBarOverlay::overlay", "rec | active | loop: {i} | Some overlay");
                        overlays.push(overlay);
                    }
                }

                if let Some((next_menu, next_menu_tree)) = next {
                    #[cfg(feature = "debug_log")]
                    debug!(target:"menu::MenuBarOverlay::overlay", "rec | next");
                    rec(
                        &mut next_menu.items,
                        next_menu_tree,
                        menu_layouts,
                        overlays,
                        renderer,
                        viewport,
                    );
                }
            } else {
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::overlay", "rec | no active");

                #[cfg(feature = "debug_log")]
                let mut count = 0;

                for ((item, item_tree), item_layout) in items[slice.start_index..=slice.end_index]
                    .iter_mut()
                    .zip(menu_tree.children[slice.start_index..=slice.end_index].iter_mut()) // [item_tree...]
                    .zip(slice_layout.children())
                {
                    #[cfg(feature = "debug_log")]
                    trace!(target:"menu::MenuBarOverlay::overlay", "rec | loop: {count}");

                    let Item {
                        item: item_widget, ..
                    } = item;

                    if let Some(overlay) = item_widget.as_widget_mut().overlay(
                        &mut item_tree.children[0],
                        item_layout,
                        renderer,
                        viewport,
                        Vector::ZERO,
                    ) {
                        #[cfg(feature = "debug_log")]
                        debug!(target:"menu::MenuBarOverlay::overlay", "rec | no active | loop: {count} | Some overlay");
                        overlays.push(overlay);
                    }

                    #[cfg(feature = "debug_log")]
                    {
                        count += 1;
                    }
                }
            }
        }

        let mut overlays = vec![];
        let mut next = None;

        for (i, ((item, item_tree), item_layout)) in self
            .menu_bar
            .roots
            .iter_mut()
            .zip(self.tree.children.iter_mut()) // [item_tree...]
            .zip(self.layout.children())
            .enumerate()
        {
            #[cfg(feature = "debug_log")]
            trace!(target:"menu::MenuBarOverlay::overlay", "root | i: {i}");

            let Item {
                item: item_widget,
                menu: item_menu,
                ..
            } = item;
            let [item_widget_tree, item_menu_tree] = item_tree.children.as_mut_slice() else {
                continue;
            };

            if i == active {
                next = Some((item_menu.as_mut().unwrap(), item_menu_tree));
            }
            if let Some(overlay) = item_widget.as_widget_mut().overlay(
                item_widget_tree,
                item_layout,
                renderer,
                &viewport,
                self.translation,
            ) {
                #[cfg(feature = "debug_log")]
                debug!(target:"menu::MenuBarOverlay::overlay", "root | i: {i} | Some overlay");
                overlays.push(overlay);
            }
        }

        if let Some((next_menu, next_menu_tree)) = next {
            rec(
                &mut next_menu.items,
                next_menu_tree,
                &mut menu_layouts,
                &mut overlays,
                renderer,
                &viewport,
            );
        };

        if overlays.is_empty() {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBarOverlay::overlay", "return | None");
            None
        } else {
            #[cfg(feature = "debug_log")]
            debug!(target:"menu::MenuBarOverlay::overlay", "return | Some");
            Some(overlay::Group::with_children(overlays).overlay())
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
        #[cfg(feature = "debug_log")]
        debug!(target:"menu::MenuBarOverlay::draw", "");

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let MenuBarState {
            global_state,
            menu_state: bar_state,
        } = bar;

        let Some(bar_menu_state) = bar_state.as_ref() else {
            return;
        };
        let Some(active) = bar_menu_state.active else {
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
            global_state: &GlobalState,
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
                #[cfg(feature = "debug_log")]
                warn!(target:"menu::MenuBarOverlay::draw", "menu exists, but menu_layout is None");
                return;
            }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds]}

            let menu_state = menu_tree.state.downcast_ref::<MenuState>();

            menu.draw(
                global_state,
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
                        global_state,
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
            global_state,
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

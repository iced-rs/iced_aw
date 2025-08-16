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
    Event, Point, Rectangle, Size, Vector,
};

use super::{common::*, menu_bar::MenuBarState, menu_tree::*};
use crate::style::{menu_bar::*, Status};

pub(super) struct MenuBarOverlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    /// Tree{ bar_state, [item_tree...] }
    pub(super) translation: Vector,
    pub(super) tree: &'b mut Tree,

    pub(super) roots: &'b mut [Item<'a, Message, Theme, Renderer>],
    pub(super) init_bar_bounds: Rectangle,
    pub(super) init_root_bounds: Vec<Rectangle>,
    pub(super) check_bounds_width: f32,
    pub(super) draw_path: &'b DrawPath,
    pub(super) scroll_speed: ScrollSpeed,
    pub(super) class: &'b Theme::Class<'a>,
}
impl<'b, Message, Theme, Renderer> MenuBarOverlay<'_, 'b, Message, Theme, Renderer>
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
        let translation = self.translation;

        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let bar_bounds = self.init_bar_bounds;

        let bar_node = Node::with_children(bar_bounds.size(), [].into())
            .move_to(bar_bounds.position() + translation);

        let roots_node = Node::with_children(
            Size::ZERO,
            self.init_root_bounds
                .iter()
                .map(|r| Node::new(r.size()).move_to(r.position()))
                .collect(),
        )
        .translate(translation);

        let Some(active) = bar.active_root else {
            return Node::with_children(bounds, [bar_node, roots_node].into());
        };

        let active_root = &mut self.roots[active];
        let active_tree = &mut self.tree.children[active]; // item_tree: Tree{ stateless, [ widget_tree, menu_tree ] }
        let parent_bounds = self.init_root_bounds[active] + translation;

        fn rec<Message, Theme: Catalog, Renderer: renderer::Renderer>(
            renderer: &Renderer,
            item: &Item<'_, Message, Theme, Renderer>,
            tree: &mut Tree,
            menu_nodes: &mut Vec<Node>,
            check_bounds_width: f32,
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
                    check_bounds_width,
                    parent_bounds,
                    parent_direction,
                    viewport,
                );
                // Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}
                menu_nodes.push(menu_node);

                let menu_state = menu_tree.state.downcast_ref::<MenuState>();

                if let Some(active) = menu_state.active {
                    let next_item = &menu.items[active];
                    let next_tree = &mut menu_tree.children[active];
                    let next_parent_bounds = {
                        let slice_node = &menu_nodes.last().unwrap().children()[0];
                        let Some(node) = slice_node
                            .children()
                            .get(active - menu_state.slice.start_index)
                        else {
                            return;
                        };

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
            self.check_bounds_width,
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

        let active_root = &mut self.roots[active];
        let active_tree = &mut self.tree.children[active];
        let mut prev_bounds_list = vec![bar_bounds];

        #[rustfmt::skip]
        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
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
            scroll_speed: ScrollSpeed,
        ) -> RecEvent {
            let Some(ref mut menu) = item.menu else {
                return RecEvent::None;
            };
            let menu_tree = &mut tree.children[1];

            let Some(menu_layout) = layout_iter.next() else {
                return RecEvent::None;
            }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}

            let mut mc = menu_layout.children();
            let slice_layout = mc.next().unwrap(); // slice_node
            let prescroll = mc.next().unwrap().bounds();
            let offset_bounds = mc.next().unwrap().bounds();
            prev_bounds_list.push(prescroll);

            let menu_state = menu_tree.state.downcast_mut::<MenuState>();

            let rec_event = if let Some(active) = menu_state.active {
                let next_tree = &mut menu_tree.children[active];
                let next_item = &mut menu.items[active];
                let next_parent_bounds = {
                    let Some(layout) = slice_layout
                        .children()
                        .nth(active - menu_state.slice.start_index)
                    else {
                        prev_bounds_list.pop();
                        return RecEvent::Event;
                    };

                    layout.bounds()
                };

                rec(
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
                    scroll_speed,
                )
            } else {
                RecEvent::Close
            };

            prev_bounds_list.pop();

            match rec_event {
                RecEvent::Event => RecEvent::Event,
                RecEvent::Close => {
                    if menu_state.pressed || cursor.is_over(prescroll){
                        menu.update(menu_tree, event, menu_layout, cursor, renderer, clipboard, shell, viewport, scroll_speed);
                        menu.open_event(menu_tree, menu_layout, cursor, shell);
                        RecEvent::Event
                    } else if cursor.is_over(offset_bounds) {
                        RecEvent::Event
                    } else {
                        menu.close_event(menu_tree, menu_layout, cursor, shell, parent_bounds, prev_bounds_list, prev);
                        if prev.is_some() {
                            RecEvent::None
                        } else {
                            RecEvent::Close
                        }
                    }
                }
                RecEvent::None => {
                    if menu_state.pressed || cursor.is_over(prescroll){
                        menu.update(menu_tree, event, menu_layout, cursor, renderer, clipboard, shell, viewport, scroll_speed);
                        menu.open_event(menu_tree, menu_layout, cursor, shell);
                        RecEvent::Event
                    } else if cursor.is_over(offset_bounds) {
                        RecEvent::Event
                    } else {
                        RecEvent::None
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
            self.scroll_speed,
        );

        match re {
            RecEvent::Event => {
                shell.capture_event();
                shell.request_redraw();
            }
            RecEvent::Close | RecEvent::None => {
                if !cursor.is_over(bar_bounds) {
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
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

        let active_root = &self.roots[active];
        let active_tree = &self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            tree: &Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator<Item = Layout<'b>>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            let Some(ref menu) = item.menu else {
                return mouse::Interaction::default();
            };
            let menu_tree = &tree.children[1];

            let Some(menu_layout) = layout_iter.next() else {
                return mouse::Interaction::default();
            }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}

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

        let active_root = &self.roots[active];
        let active_tree = &mut self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            tree: &mut Tree,
            item: &Item<'a, Message, Theme, Renderer>,
            layout_iter: &mut impl Iterator<Item = Layout<'b>>,
            renderer: &Renderer,
            operation: &mut dyn Operation<()>,
        ) {
            if item.menu.is_none() {
                return;
            }

            let menu = item.menu.as_ref().expect("No menu defined in this item");
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
        let bar = self.tree.state.downcast_ref::<MenuBarState>();
        let active = bar.active_root?;
        let mut lc = layout.children();
        let _bar_bounds = lc.next()?.bounds();
        let _roots_layout = lc.next()?;
        let menu_layouts_layout = lc.next()?; // Node{0, [menu_node...]}
        let mut menu_layouts = menu_layouts_layout.children(); // [menu_node...]
        let active_root = &mut self.roots[active];
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

        let active_root = &self.roots[active];
        let active_tree = &self.tree.children[active];

        fn rec<'a, 'b, Message, Theme: Catalog, Renderer: renderer::Renderer>(
            draw_path: &DrawPath,
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
            if let Some(menu) = item.menu.as_ref() {
                let menu_tree = &tree.children[1];

                let Some(menu_layout) = layout_iter.next() else {
                    return;
                }; // menu_node: Node{inf, [ slice_node, prescroll, offset_bounds, check_bounds ]}

                let menu_state = menu_tree.state.downcast_ref::<MenuState>();

                menu.draw(
                    draw_path,
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
                            draw_path,
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
        }

        let theme_style = theme.style(self.class, Status::Active);

        rec(
            self.draw_path,
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

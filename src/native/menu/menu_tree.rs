//! doc
//!
use std::{any::Any, borrow::BorrowMut};

use super::types::*;
use super::{flex, menu_bar::MenuBarState};
use iced_widget::core::{
    alignment, event,
    layout::{self, Limits, Node},
    mouse, overlay, renderer, touch,
    widget::tree::{self, Tree},
    Alignment, Border, Clipboard, Color, Element, Event, Length, Padding, Point, Rectangle, Shell,
    Size, Vector, Widget,
};

/*

menu tree:
Item{
    widget
    Menu [
        Item{...}
        Item{...}
        Item{...}
        ...
    ]
}

state tree:
Tree{
    item state
    [
        Tree{widget state}
        Tree{
            menu state
            [
                Tree{item state [...]}
                Tree{item state [...]}
                Tree{item state [...]}
                ...
            ]
        }
    ]
}

*/

#[derive(Debug)]
pub(super) struct MenuState {
    scroll_offset: f32,
    pub(super) active: Index,
}
impl Default for MenuState{
    fn default() -> Self {
        Self {
            scroll_offset: 0.0,
            active: None,
        }
    }
}

/// menu
pub struct Menu<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) items: Vec<Item<'a, Message, Theme, Renderer>>,
    pub(super) spacing: f32,
    pub(super) padding: Padding,
    pub(super) max_width: f32,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) axis: Axis,
    pub(super) offset: f32,
    pub(super) open_condition: OpenCondition,
}
#[allow(missing_docs)]
impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(items: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            items,
            spacing: 0.0,
            padding: Padding::ZERO,
            max_width: f32::MAX,
            width: Length::Fill,
            height: Length::Shrink,
            axis: Axis::Horizontal,
            offset: 20.0,
            open_condition: OpenCondition::Click,
        }
    }

    pub fn tree(&self) -> Tree {
        Tree {
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }

    pub fn max_width(mut self, max_width: f32) -> Self{
        self.max_width = max_width;
        self
    }
}
impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    pub(super) fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuState>()
    }

    pub(super) fn state(&self) -> tree::State {
        tree::State::Some(Box::new(MenuState::default()))
    }

    /// out: \[item_tree...]
    pub(super) fn children(&self) -> Vec<Tree> {
        self.items.iter().map(|i| i.tree()).collect()
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    pub(super) fn diff(&self, tree: &mut Tree) {
        // tree.diff_children(&self.items.iter().map(|i| &i.item ).collect::<Vec<_>>());
        tree.diff_children_custom(
            &self.items,
            |tree, item| item.diff(tree),
            |item| item.tree(),
        )
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    ///
    /// out: Node{inf, \[ items_node, prescroll, offset_boundss, check_bounds ]}
    pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
        parent_bounds: Rectangle,
        parent_direction: (Direction, Direction),
        translation: Vector,
        viewport: &Rectangle,
    ) -> (layout::Node, (Direction, Direction)) {
        let limits = limits.max_width(self.max_width);

        let items_node = flex::resolve(
            flex::Axis::Vertical,
            renderer,
            &limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            alignment::Alignment::Center,
            &self.items.iter().map(|i| &i.item).collect::<Vec<_>>(),
            &mut tree
                .children
                .iter_mut()
                .map(|t| &mut t.children[0])
                .collect::<Vec<_>>(),
        );

        let aod = Aod::new(self.axis, viewport.size(), parent_bounds, parent_direction, self.offset);

        let children_size = items_node.bounds().size();
        let (children_position, offset_position, child_direction) = aod.resolve(parent_bounds, children_size, viewport.size());
        
        // calc offset bounds
        let delta = children_position - offset_position;
        let offset_size = if delta.x.abs() > delta.y.abs() {
            Size::new(self.offset, children_size.height)
        } else {
            Size::new(children_size.width, self.offset)
        };
        
        let offset_bounds = Rectangle::new(offset_position, offset_size);
        let children_bounds = Rectangle::new(children_position, children_size);
        let bounds_expand = 30.0;
        let check_bounds = pad_rectangle(children_bounds, [bounds_expand; 4].into());

        let menu_state = tree.state.downcast_ref::<MenuState>();

        (
            layout::Node::with_children(
                Size::INFINITY,
                [
                    items_node
                        .move_to(children_position)
                        .translate([0.0, menu_state.scroll_offset]), // items layout
                    layout::Node::new(children_size)
                        .move_to(children_position), // prescroll bounds
                    layout::Node::new(offset_bounds.size())
                        .move_to(offset_bounds.position()), // offset boundss
                    layout::Node::new(check_bounds.size())
                        .move_to(check_bounds.position()), // check bounds
                ]
                .into(),
            ),
            child_direction
        )
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    ///
    /// layout: Node{inf, \[ items_node, prescroll, offset_boundss, check_bounds ]}
    pub(super) fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // println!("menu event");
        let mut lc = layout.children();
        let items_layout = lc.next().unwrap();

        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let status = self
            .items // [item...]
            .iter_mut()
            .zip(tree.children.iter_mut()) // [item_tree...]
            .zip(items_layout.children()) // [item_layout...]
            .map(|((item, tree), layout)| {
                item.on_event(
                    tree,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge);

        let menu_state = tree.state.downcast_mut::<MenuState>();

        use event::Status::*;
        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(items_layout.bounds()) {
                    process_scroll_event(menu_state, prescroll, delta, viewport.size());
                    Captured
                } else if cursor.is_over(offset_bounds) || cursor.is_over(check_bounds) {
                    Captured
                } else {
                    Ignored
                }
            }
            _ => Ignored,
        }
        .merge(status)
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    ///
    /// layout: Node{inf, \[ items_node, prescroll, offset_bounds, check_bounds ]}
    pub(super) fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        parent_bounds: Rectangle,
    ) {
        let mut lc = layout.children();
        let items_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        // println!("prescroll: {:?}", prescroll);
        // println!("parent_bounds: {:?}", parent_bounds);

        renderer.fill_quad(
            renderer::Quad{
                bounds: check_bounds,
                border: Border{
                    // color: todo!(),
                    // width: todo!(),
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            Color::from([1.0, 0.0, 0.0, 0.1])
        );
        renderer.fill_quad(
            renderer::Quad{
                bounds: prescroll,
                border: Border{
                    // color: todo!(),
                    // width: todo!(),
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            Color::from([1.0, 1.0, 1.0, 1.0])
        );
        renderer.fill_quad(
            renderer::Quad{
                bounds: offset_bounds,
                border: Border{
                    // color: todo!(),
                    // width: todo!(),
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            Color::from([0.0, 0.0, 1.0, 0.3])
        );

        renderer.fill_quad(
            renderer::Quad{
                bounds: parent_bounds,
                border: Border{
                    // color: todo!(),
                    // width: todo!(),
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            Color::from([1.0, 1.0, 0.0, 0.5])
        );

        for ((item, tree), layout) in self
            .items // [item...]
            .iter()
            .zip(tree.children.iter()) // [item_tree...]
            .zip(items_layout.children())
        // [item_layout...]
        {
            item.draw(tree, renderer, theme, style, layout, cursor, &viewport);
        }
    }

    /// layout: Node{inf, \[ items_node, prescroll, offset_boundss, check_bounds ]}
    pub(super) fn open_or_close_event(
        &mut self,
        // process_oce: &mut bool,
        prev: &mut Index,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        parent_bounds: Rectangle,
        prev_bounds_list: &[Rectangle],
    ){
        let mut lc = layout.children();
        let items_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();
        
        let menu_state = tree.state.downcast_mut::<MenuState>();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let open = {
                    if cursor.is_over(prescroll)
                    || cursor.is_over(parent_bounds) 
                    || cursor.is_over(offset_bounds) {
                        true
                    } else if prev_bounds_list.iter().any(|r| cursor.is_over(*r)) {
                        false
                    } else if cursor.is_over(check_bounds) {
                        true
                    } else {
                        false
                    }
                };

                if open {
                    if !cursor.is_over(prescroll) {
                        menu_state.active = None
                    } else {
                        self.items
                            .iter()
                            .zip(items_layout.children())
                            .enumerate()
                            .for_each(|(i, (item, layout))|{
                                if item.menu.is_some() && cursor.is_over(layout.bounds()) {
                                    println!("new active: {}", i);
                                    menu_state.active = Some(i)
                                }
                            })
                    }
                } else {
                    // *process_oce = true;

                    println!("set prev none");
                    *prev = None;
                    menu_state.scroll_offset = 0.0
                }
            }
            _ => ()
        }
    }

    pub(super) fn open_event(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) -> event::Status {
        let mut lc = layout.children();
        let items_layout = lc.next().unwrap();
        // let prescroll = lc.next().unwrap().bounds();
        // let offset_bounds = lc.next().unwrap().bounds();
        // let check_bounds = lc.next().unwrap().bounds();

        let menu_state = tree.state.downcast_mut::<MenuState>();
        menu_state.active = None;

        for (i, (item, layout)) in self.items
            .iter()
            .zip(items_layout.children())
            .enumerate()
        {
            if item.menu.is_some() && cursor.is_over(layout.bounds()) {
                println!("new active: {}", i);
                menu_state.active = Some(i);
                return event::Status::Captured;
            }
        }
        event::Status::Ignored
    }

    pub(super) fn close_event(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        parent_bounds: Rectangle,
        prev_bounds_list: &[Rectangle],
        prev: &mut Index
    ) {
        let mut lc = layout.children();
        let _items_layout = lc.next().unwrap();
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let open = {
            if cursor.is_over(prescroll)
            || cursor.is_over(parent_bounds) 
            || cursor.is_over(offset_bounds) {
                true
            } else if prev_bounds_list.iter().any(|r| cursor.is_over(*r)) {
                false
            } else if cursor.is_over(check_bounds) {
                true
            } else {
                false
            }
        };

        if !open {
            *prev = None;
            let menu_state = tree.state.downcast_mut::<MenuState>();
            menu_state.scroll_offset = 0.0
        }
    }
    
}

// #[derive(Debug, Default)]
// pub(super) struct ItemState{
//     pub(super) open: bool
// }

/// menu item
pub struct Item<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) item: Element<'a, Message, Theme, Renderer>,
    pub(super) menu: Option<Box<Menu<'a, Message, Theme, Renderer>>>,
}
#[allow(missing_docs)]
impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(item: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            item: item.into(),
            menu: None,
        }
    }

    pub fn with_menu(
        item: impl Into<Element<'a, Message, Theme, Renderer>>,
        menu: Menu<'a, Message, Theme, Renderer>,
    ) -> Self {
        Self {
            item: item.into(),
            menu: Some(Box::new(menu)),
        }
    }

    pub fn menu(mut self, menu: Menu<'a, Message, Theme, Renderer>) -> Self {
        self.menu = Some(Box::new(menu));
        self
    }

    pub(super) fn tree(&self) -> Tree {
        Tree {
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn size(&self) -> Size<Length> {
        self.item.as_widget().size()
    }

    pub(super) fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    pub(super) fn state(&self) -> tree::State {
        // tree::State::Some(Box::new(ItemState::default()))
        tree::State::None
    }

    /// out: \[widget_tree, menu_tree]
    pub(super) fn children(&self) -> Vec<Tree> {
        self.menu
            .as_ref()
            .map_or([Tree::new(&self.item)].into(), |m| {
                [Tree::new(&self.item), m.tree()].into()
            })
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    pub(super) fn diff(&self, tree: &mut Tree) {
        tree.children[0].diff(&self.item);
        self.menu
            .as_ref()
            .map_or({}, |m| m.diff(&mut tree.children[1]))
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
        // translation: Vector,
    ) -> layout::Node {
        // println!("Item layout");
        self.item
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // println!("item event");
        self.item.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    /// tree: Tree{stateless, \[widget_tree, menu_tree]}
    ///
    pub(super) fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.item.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }
}

/// Adaptive open direction
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
struct Aod {
    // whether or not to use aod
    horizontal: bool,
    vertical: bool,

    // whether or not to use overlap
    horizontal_overlap: bool,
    vertical_overlap: bool,

    // default direction
    horizontal_direction: Direction,
    vertical_direction: Direction,

    // Offset of the child in the default direction
    horizontal_offset: f32,
    vertical_offset: f32,
}
impl Aod {
    /// Returns child position. offset position, child direction
    #[allow(clippy::too_many_arguments)]
    fn adaptive(
        parent_pos: f32,
        parent_size: f32,
        child_size: f32,
        max_size: f32,
        offset: f32,
        on: bool,
        overlap: bool,
        direction: Direction,
    ) -> (f32, f32, Direction) {
        /*
        Imagine there're two sticks, parent and child
        parent: o-----o
        child:  o----------o

        Now we align the child to the parent in one dimension
        There are 4 possibilities:

        1. to the right
                    o-----oo----------o

        2. to the right with overlaping
                    o-----o
                    o----------o

        3. to the left
        o----------oo-----o

        4. to the left with overlaping
                    o-----o
               o----------o

        The child goes to the default direction by default,
        if the space on the default direction runs out it goes to the the other,
        whether to use overlap is the caller's decision

        This can be applied to any direction
        */

        match direction {
            Direction::Positive => {
                let space_negative = parent_pos;
                let space_positive = max_size - parent_pos - parent_size;

                if overlap {
                    let overshoot = child_size - parent_size;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos - overshoot, parent_pos - overshoot, direction.flip())
                    } else {
                        (parent_pos, parent_pos, direction)
                    }
                } else {
                    let overshoot = child_size + offset;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos - overshoot, parent_pos - offset, direction.flip())
                    } else {
                        (parent_pos + parent_size + offset, parent_pos + parent_size, direction)
                    }
                }
            }
            Direction::Negative => {
                let space_positive = parent_pos;
                let space_negative = max_size - parent_pos - parent_size;

                if overlap {
                    let overshoot = child_size - parent_size;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos, parent_pos, direction.flip())
                    } else {
                        (parent_pos - overshoot, parent_pos - overshoot, direction)
                    }
                } else {
                    let overshoot = child_size + offset;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos + parent_size + offset, parent_pos + parent_size, direction.flip())
                    } else {
                        (parent_pos - overshoot, parent_pos - offset, direction)
                    }
                }
            }
        }
    }

    /// Returns child position and offset position
    fn resolve(
        &self,
        parent_bounds: Rectangle,
        children_size: Size,
        viewport_size: Size,
    ) -> (Point, Point, (Direction, Direction)) {
        let (x, ox, dx) = Self::adaptive(
            parent_bounds.x,
            parent_bounds.width,
            children_size.width,
            viewport_size.width,
            self.horizontal_offset,
            self.horizontal,
            self.horizontal_overlap,
            self.horizontal_direction,
        );
        let (y, oy, dy) = Self::adaptive(
            parent_bounds.y,
            parent_bounds.height,
            children_size.height,
            viewport_size.height,
            self.vertical_offset,
            self.vertical,
            self.vertical_overlap,
            self.vertical_direction,
        );

        ([x, y].into(), [ox, oy].into(), (dx, dy))
    }

    fn new(
        axis: Axis,
        viewport: Size,
        parent_bounds: Rectangle,
        parent_direction: (Direction, Direction),
        offset: f32,
    ) -> Self{
        let hcenter = viewport.width / 2.0;
        let vcenter = viewport.height / 2.0;

        let phcenter = parent_bounds.x + parent_bounds.width / 2.0;
        let pvcenter = parent_bounds.y + parent_bounds.height / 2.0;

        let (pdx, pdy) = parent_direction;
        match axis {
            Axis::Horizontal =>{
                let horizontal_direction = pdx;
                let vertical_direction = if pvcenter < vcenter {
                    Direction::Positive
                } else {
                    Direction::Negative
                };
                Aod {
                    horizontal: true,
                    vertical: true,
                    horizontal_overlap: false,
                    vertical_overlap: true,
                    horizontal_direction,
                    vertical_direction,
                    horizontal_offset: offset,
                    vertical_offset: 0.0,
                }
            },
            Axis::Vertical => {
                let horizontal_direction = if phcenter < hcenter {
                    Direction::Positive
                } else {
                    Direction::Negative
                };
                let vertical_direction = pdy;
                Aod {
                    horizontal: true,
                    vertical: true,
                    horizontal_overlap: true,
                    vertical_overlap: false,
                    horizontal_direction,
                    vertical_direction,
                    horizontal_offset: 0.0,
                    vertical_offset: offset,
                }
            },
        }
    }
}

fn pad_rectangle(rect: Rectangle, padding: Padding) -> Rectangle {
    Rectangle {
        x: rect.x - padding.left,
        y: rect.y - padding.top,
        width: rect.width + padding.horizontal(),
        height: rect.height + padding.vertical(),
    }
}

fn search_bound(
    default: usize,
    default_left: usize,
    default_right: usize,
    bound: f32,
    positions: &[f32],
    sizes: &[Size],
) -> usize {
    // binary search
    let mut left = default_left;
    let mut right = default_right;

    let mut index = default;
    while left != right {
        let m = ((left + right) / 2) + 1;
        if positions[m] > bound {
            right = m - 1;
        } else {
            left = m;
        }
    }
    let height = sizes[left].height;
    if positions[left] + height > bound {
        index = left;
    }
    index
}

fn process_scroll_event(
    menu_state: &mut MenuState,
    prescroll_children_bounds: Rectangle,
    delta: mouse::ScrollDelta,
    viewport_size: Size,
){
    use mouse::ScrollDelta;

    let pcb = prescroll_children_bounds;

    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => y,
    };

    let max_offset = (0.0 - pcb.y).max(0.0);
    let min_offset = (viewport_size.height - (pcb.y + pcb.height)).min(0.0);
    menu_state.scroll_offset = (menu_state.scroll_offset + delta_y).clamp(min_offset, max_offset);
}

//! doc
//!
use std::borrow::BorrowMut;

use iced_widget::core::{
    alignment, event, layout::{self, Limits, Node}, mouse, overlay, renderer, touch, widget::tree::{self, Tree}, Alignment, Border, Clipboard, Color, Element, Event, Length, Padding, Point, Rectangle, Shell, Size, Vector, Widget
};
use super::types::*;
use super::{flex, menu_bar::MenuBarState};

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

layout tree
Node{
    item node
    [
        Node{widget node}
        Node{
            menu node
            [
                Node{item node [...]}
                Node{item node [...]}
                Node{item node [...]}
                ...
            ]
        }
    ]
}

*/

#[derive(Debug, Default)]
pub(super) struct MenuState{
    scroll_offset:f32,
}

/// menu
pub struct Menu<'a, Message, Theme, Renderer>
where
    Renderer:renderer::Renderer
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
    Renderer:renderer::Renderer
{
    pub fn new(items: Vec<Item<'a, Message, Theme, Renderer>>) -> Self{
        Self{
            items,
            spacing: 0.0,
            padding: Padding::ZERO,
            max_width: f32::MAX,
            width: Length::Shrink,
            height: Length::Shrink,
            axis: Axis::Vertical,
            offset: 0.0,
            open_condition: OpenCondition::Click,
        }
    }

    pub fn tree(&self) -> Tree{
        Tree{
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<'a, Message, Theme, Renderer> 
    // Widget<Message, Theme, Renderer> for
    Menu<'a, Message, Theme, Renderer>
where
    Renderer:renderer::Renderer
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
        self.items.iter().map(|i| i.tree() ).collect()
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    pub(super) fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.items.iter().map(|i| &i.item ).collect::<Vec<_>>());
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    /// 
    /// out: Node{size:inf, \[items_layout, prescroll, offset_bounds, check_bounds]}
    pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
        parent_bounds: Rectangle,
        // translation: Vector,
        viewport: &Rectangle,
    ) -> layout::Node {
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
            &self.items.iter().map(|i| &i.item ).collect::<Vec<_>>(),
            &mut tree.children.iter_mut().map(|t| &mut t.children[0] ).collect::<Vec<_>>(),
        );

        let items_node = Node::with_children(
            items_node.bounds().size(), 
            self.items
                .iter()
                .zip(tree.children.iter_mut())
                .zip(items_node.children().into_iter())
                .map(|((item, tree), node)|{
                    Node::with_children(
                        Size::ZERO, 
                        [
                            node.clone(),
                            item.layout(tree, renderer, &Limits::NONE, viewport, 
                                // translation
                            )
                        ].into()
                    )
                })
                .collect()
        );


        // for ((item, tree), node) in self.items
        //     .iter()
        //     .zip(tree.children.iter_mut())
        //     .zip(items_node.children().iter_mut())
        // {
        //     *node = Node::with_children(
        //         Size::ZERO, 
        //         [
        //             node.clone(),
        //             item.layout(tree, renderer, &Limits::NONE, viewport, 
        //                 // translation
        //             )
        //                 .children()[1]
        //         ].into()
        //     )
        // }

        let bounds = viewport.size();
        // let vpb = parent_bounds + translation; // viewport space parent bounds
        let vpb = parent_bounds;
        let aod = {
            let hcenter = bounds.width / 2.0;
            let vcenter = bounds.height / 2.0;
    
            let phcenter = vpb.x + vpb.width / 2.0;
            let pvcenter = vpb.y + vpb.height / 2.0;
    
            let horizontal_direction = if phcenter < hcenter { Direction::Positive } else { Direction::Negative };
            let vertical_direction   = if pvcenter < vcenter { Direction::Positive } else { Direction::Negative };
    
            match self.axis {
                Axis::Horizontal => Aod{
                    horizontal: false,
                    vertical: true,
                    horizontal_overlap: false,
                    vertical_overlap: true,
                    horizontal_direction,
                    vertical_direction,
                    horizontal_offset: self.offset,
                    vertical_offset: 0.0,
                },
                Axis::Vertical => Aod{
                    horizontal: true,
                    vertical: false,
                    horizontal_overlap: true,
                    vertical_overlap: false,
                    horizontal_direction,
                    vertical_direction,
                    horizontal_offset: 0.0,
                    vertical_offset: self.offset,
                }
            }
        };

        let children_size = items_node.bounds().size();
        let (children_position, offset_position) = aod.resolve(
            vpb,
            children_size,
            bounds
        );

        // calc offset bounds
        let delta = children_position - offset_position;
        let offset_size = if delta.x.abs() > delta.y.abs() {
            Size::new(delta.x, children_size.height)
        } else {
            Size::new(children_size.width, delta.y)
        };
        let offset_bounds = Rectangle::new(offset_position, offset_size);
        let children_bounds = Rectangle::new(children_position, children_size);
        let bounds_expand = 30.0;
        let check_bounds = pad_rectangle(children_bounds, [bounds_expand; 4].into());
        
        let menu_state = tree.state.downcast_ref::<MenuState>();

        layout::Node::with_children(Size::INFINITY, [
            items_node
                .move_to(children_position)
                .translate([0.0, menu_state.scroll_offset]), // items layout
            layout::Node::new(children_size)
                .move_to(children_position), // prescroll menu bounds
            
            layout::Node::new(offset_bounds.size())
                .move_to(offset_bounds.position())
                .translate([0.0, menu_state.scroll_offset]), // offset bounds
            layout::Node::new(check_bounds.size())
                .move_to(check_bounds.position())
                .translate([0.0, menu_state.scroll_offset]), // check bounds
        ].into())
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    /// 
    /// layout: Node{size:inf, \[items_layout, prescroll, offset_bounds, check_bounds]}
    pub(super) fn on_event(
        &mut self,
        // bar: &mut MenuBarState,
        item_state: &mut ItemState,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
        parent_bounds: &Rectangle,
        items_bounds_list: &[Rectangle],
    ) -> event::Status {
        let mut lc = layout.children();
        let items_layout = lc.next().unwrap();
        
        let prescroll = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let status = self.items // [item...]
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
                if cursor.is_over(items_layout.bounds()){
                    process_scroll_event(menu_state, prescroll, delta, viewport.size())
                }else{
                    Ignored
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let open = {
                    if prescroll.contains(position){
                        true
                    }else if items_bounds_list
                        .iter()
                        .any(|r| r.contains(position))
                    {
                        false
                    }else if parent_bounds.contains(position)
                        || offset_bounds.contains(position)
                        || check_bounds.contains(position)
                    {
                        true
                    }else{
                        false
                    }
                };

                if item_state.open == true && open == false {
                    item_state.open = false;
                    // menu_state.scroll_offset = 0.0;
                }
                Captured
            }
            _ => Ignored
        }.merge(status)
    }

    /// tree: Tree{menu_state, \[item_tree...]}
    /// 
    /// layout: Node{size:inf, \[items_layout, prescroll, offset_bounds, check_bounds]}
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
        let mut lc = layout.children();
        let items_layout = lc.next().unwrap();

        for ((item, tree), layout) in self.items // [item...]
            .iter()
            .zip(tree.children.iter()) // [item_tree...]
            .zip(items_layout.children()) // [item_layout...]
        {
            item.draw(
                tree, renderer, theme, style, layout, cursor, &viewport,
            );
        }
    }
}


#[derive(Debug, Default)]
pub(super) struct ItemState{
    pub(super) open: bool
}

/// menu item
pub struct Item<'a, Message, Theme, Renderer>
where
    Renderer:renderer::Renderer
{
    pub(super) item: Element<'a, Message, Theme, Renderer>,
    pub(super) menu: Option<Box<Menu<'a, Message, Theme, Renderer>>>,
}
#[allow(missing_docs)]
impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Renderer:renderer::Renderer
{
    pub fn new(item: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self{
        Self{
            item: item.into(),
            menu: None,
        }
    }

    pub fn with_menu(item: impl Into<Element<'a, Message, Theme, Renderer>>, menu: Menu<'a, Message, Theme, Renderer>) -> Self{
        Self{
            item: item.into(),
            menu: Some(Box::new(menu)),
        }
    }

    pub fn menu(mut self, menu: Menu<'a, Message, Theme, Renderer>) -> Self{
        self.menu = Some(Box::new(menu));
        self
    }

    pub(super) fn tree(&self) -> Tree{
        Tree{
            tag: self.tag(),
            state: self.state(),
            children: self.children(),
        }
    }
}
impl<'a, Message, Theme, Renderer>
    // Widget<Message, Theme, Renderer> for
    Item<'a, Message, Theme, Renderer>
where
    Renderer:renderer::Renderer
{
    pub(super) fn size(&self) -> Size<Length> {
        self.item.as_widget().size()
    }
    
    pub(super) fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ItemState>()
    }

    pub(super) fn state(&self) -> tree::State {
        tree::State::Some(Box::new(ItemState::default()))
    }

    /// out: \[widget_tree, menu_tree]
    pub(super) fn children(&self) -> Vec<Tree> {
        self.menu.as_ref().map_or(
            [
                Tree::new(&self.item),
            ].into(),
            |m|[
                Tree::new(&self.item),
                m.tree()
            ].into(),
        )
    }

    /// tree: Tree{item_state, \[widget_tree, menu_tree]}
    pub(super) fn diff(&self, tree: &mut Tree) {
        tree.children[0].diff(&self.item);
        self.menu.as_ref().map_or(
            {}, 
            |m| {
                m.diff(&mut tree.children[1])
            }
        )
    }

    /// tree: Tree{item_state, \[widget_tree, menu_tree]}
    /// 
    /// out: Node{size:0, \[widget_node, menu_node]}
    pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
        viewport: &Rectangle,
        // translation: Vector,
    ) -> layout::Node {
        let state = tree.state.downcast_ref::<ItemState>();
        let widget_node = self.item.as_widget().layout(&mut tree.children[0], renderer, limits);
        let parent_bounds = widget_node.bounds();
        Node::with_children(
            Size::ZERO, 
            [
                widget_node,
                if state.open{
                    self.menu.as_ref().unwrap().layout(
                        &mut tree.children[1], 
                        renderer, 
                        &Limits::NONE, 
                        parent_bounds,
                        // translation,
                        viewport,
                    )
                }else{
                    Node::default()
                }
            ].into()
        )
    }

    /// tree: Tree{item_state, \[widget_tree, menu_tree]}
    /// 
    /// layout: Node{size:0, \[widget_node, menu_node]}
    pub(super) fn on_event(
        &mut self,
        // index: usize, // within parent menu
        // bar: &mut MenuBarState,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut lc = layout.children();
        let widget_layout = lc.next().unwrap();
        let status = self.item.as_widget_mut().on_event(
            &mut tree.children[0], 
            event.clone(), 
            widget_layout, 
            cursor, 
            renderer, 
            clipboard, 
            shell, 
            viewport
        );

        let item_state = tree.state.downcast_mut::<ItemState>();

        let Some(menu) = self.menu.as_ref() else {
            item_state.open = false;
            return status
        };
        let menu_state = tree.children[1].state.downcast_mut::<MenuState>();
        let widget_bounds = widget_layout.bounds();

        use event::Status::*;
        match menu.open_condition{
            OpenCondition::Click => match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if cursor.is_over(widget_bounds) {
                        if item_state.open == false {
                            item_state.open = true;
                            menu_state.scroll_offset = 0.0;
                            // bar.indices.push(index);
                        }
                        Captured
                    }else{
                        Ignored
                    }
                }
                _ => Ignored
            }
            OpenCondition::Hover => match event {
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    if widget_bounds.contains(position) {
                        if item_state.open == false {
                            item_state.open = true;
                            menu_state.scroll_offset = 0.0;
                            // bar.indices.push(index);
                        }
                        Captured
                    }else{
                        Ignored
                    }
                }
                _ => Ignored
            }
        }
        .merge(status)
    }

    /// tree: Tree{item_state, \[widget_tree, menu_tree]}
    /// 
    /// layout: Node{size:0, \[widget_node, menu_node]}
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
        let item_layout = layout.children().next().unwrap();
        self.item.as_widget().draw(
            tree, 
            renderer, 
            theme, 
            style, 
            item_layout, 
            cursor, 
            viewport
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
    /// Returns child position and offset position
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
    ) -> (f32, f32) {
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
                        (parent_pos - overshoot, parent_pos - overshoot)
                    } else {
                        (parent_pos, parent_pos)
                    }
                } else {
                    let overshoot = child_size + offset;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos - overshoot, parent_pos - offset)
                    } else {
                        (parent_pos + parent_size + offset, parent_pos + parent_size)
                    }
                }
            }
            Direction::Negative => {
                let space_positive = parent_pos;
                let space_negative = max_size - parent_pos - parent_size;

                if overlap {
                    let overshoot = child_size - parent_size;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos, parent_pos)
                    } else {
                        (parent_pos - overshoot, parent_pos - overshoot)
                    }
                } else {
                    let overshoot = child_size + offset;
                    if on && space_negative > space_positive && overshoot > space_positive {
                        (parent_pos + parent_size + offset, parent_pos + parent_size)
                    } else {
                        (parent_pos - overshoot, parent_pos - offset)
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
    ) -> (Point, Point) {
        let (x, ox) = Self::adaptive(
            parent_bounds.x,
            parent_bounds.width,
            children_size.width,
            viewport_size.width,
            self.horizontal_offset,
            self.horizontal,
            self.horizontal_overlap,
            self.horizontal_direction,
        );
        let (y, oy) = Self::adaptive(
            parent_bounds.y,
            parent_bounds.height,
            children_size.height,
            viewport_size.height,
            self.vertical_offset,
            self.vertical,
            self.vertical_overlap,
            self.vertical_direction,
        );

        ([x, y].into(), [ox, oy].into())
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
) -> event::Status{
    use mouse::ScrollDelta;

    let pcb = prescroll_children_bounds;

    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => y,
    };
 
    let max_offset = (0.0 - pcb.y).max(0.0);
    let min_offset = (viewport_size.height - (pcb.y + pcb.height)).min(0.0);
    menu_state.scroll_offset = (menu_state.scroll_offset + delta_y).clamp(min_offset, max_offset);

    event::Status::Captured
}

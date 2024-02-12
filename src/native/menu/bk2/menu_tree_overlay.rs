use iced_widget::core::{
    event, layout::{self, Node}, mouse, overlay, renderer, touch, widget::tree::{self, Tree}, 
    Alignment, Border, Clipboard, Color, Element, Event, Length, 
    Padding, Point, Vector, Rectangle, Shell, Size, Widget,
};
use super::types::*;
use super::menu_tree::{MenuTree, MenuTreeState};

pub(super) struct MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer
{
    pub(super) state: &'b mut MenuTreeState,
    pub(super) tree: &'b mut Tree,
    pub(super) menu_tree: &'b MenuTree<'a, Message, Theme, Renderer>,
    pub(super) parent_bounds: Rectangle,
}
impl<'a, 'b, Message, Theme, Renderer> MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer
{
    // pub(super) fn overlay_element(self) -> overlay::Element<'b, Message, Theme, Renderer>{
        // overlay::Element::new(Point::ORIGIN, Box::new(self))
    // }
}
impl<'a, 'b, Message, Theme, Renderer> 
    // overlay::Overlay<Message, Theme, Renderer> for 
    MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
        translation: Vector,
    ) -> layout::Node {
        println!();
        println!("mxo layout");
        println!("bounds: {:?}", bounds);
        println!("position: {:?}", position);
        println!("translation: {:?}", translation);
        println!();

        let limits = layout::Limits::NONE.max_width(self.menu_tree.max_width);
        let children = &self.menu_tree.children.iter().map(|mt| mt.parent).collect::<Vec<_>>();
        let layout = layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            self.menu_tree.width,
            self.menu_tree.height,
            self.menu_tree.padding,
            self.menu_tree.spacing,
            Alignment::Center,
            children,
            &mut self.tree.children
        );

        let vpb = self.parent_bounds + translation; // viewport space parent bounds

        let hcenter = bounds.width / 2.0;
        let vcenter = bounds.height / 2.0;

        let phcenter = vpb.x + vpb.width / 2.0;
        let pvcenter = vpb.y + vpb.height / 2.0;

        let horizontal_direction = if phcenter < hcenter { Direction::Positive } else { Direction::Negative };
        let vertical_direction   = if pvcenter < vcenter { Direction::Positive } else { Direction::Negative };

        let aod = match self.menu_tree.axis {
            Axis::Horizontal => Aod{
                horizontal: false,
                vertical: true,
                horizontal_overlap: false,
                vertical_overlap: true,
                horizontal_direction,
                vertical_direction,
                horizontal_offset: self.menu_tree.offset,
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
                vertical_offset: self.menu_tree.offset,
            }
        };

        let children_size = layout.bounds().size();
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
        
        layout::Node::with_children(Size::INFINITY, [
            layout.move_to(children_position)
            .translate([0.0, self.state.scroll_offset]), // children layout
            layout::Node::new(children_size)
                .move_to(children_position), // prescroll children bounds
            layout::Node::new(bounds), // viewport
            layout::Node::new(offset_bounds.size())
                .move_to(offset_bounds.position())
                .translate([0.0, self.state.scroll_offset]), // offset bounds
            layout::Node::new(check_bounds.size())
                .move_to(check_bounds.position())
                .translate([0.0, self.state.scroll_offset]), // check bounds
        ].into())
    }

    pub(super) fn on_event(
        &mut self,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        println!("mxo event");
        use event::Status::*;

        let mut lc = layout.children();
        let children_layout = lc.next().unwrap();
        let prescroll_children_bounds = lc.next().unwrap().bounds();
        let viewport = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let children_bounds = children_layout.bounds();

        let status = self.menu_tree.children
            .iter_mut()
            .zip(&mut self.tree.children)
            .zip(children_layout.children())
            .map(|((child, tree), layout)| 
                child.parent.as_widget_mut().on_event(
                    tree,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    &viewport,
                )
            )
            .fold(event::Status::Ignored, event::Status::merge);
    
        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(children_bounds){
                    // self.state.scroll_offset += match delta{
                    //     mouse::ScrollDelta::Lines { x, y } => y,
                    //     mouse::ScrollDelta::Pixels { x, y } => y,
                    // };
                    // Ignored
                    process_scroll_event(&mut self.state, prescroll_children_bounds, delta, viewport.size())
                }else{
                    Ignored
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                self.state.open = 
                    self.parent_bounds.contains(position)
                    || offset_bounds.contains(position)
                    || check_bounds.contains(position);
                Captured
            }
            _ => Ignored
        }.merge(status)
    }

    pub(super) fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        println!("mxo draw");
        let mut lc = layout.children();
        let children_layout = lc.next().unwrap();
        let prescroll_children_bounds = lc.next().unwrap().bounds();
        let viewport = lc.next().unwrap().bounds();
        // let offset_bounds = lc.next().unwrap().bounds();
        // let check_bounds = lc.next().unwrap().bounds();

        renderer.fill_quad(
            renderer::Quad{
                bounds: self.parent_bounds,
                ..Default::default()
            }, 
            Color::from([1.0, 0.0, 0.0, 0.5])
        );

        renderer.fill_quad(
            renderer::Quad{
                bounds: pad_rectangle(prescroll_children_bounds, 4.0.into()),
                border: Border{
                    color: [0.5; 3].into(),
                    width: 1.5,
                    radius: 6.0.into(),
                    ..Default::default()
                }, 
                ..Default::default()
            }, 
            Color::from([1.0; 3])
        );

        if let Some(viewport) = children_layout.bounds().intersection(&viewport) {
            for ((child, tree), layout) in self
                .menu_tree.children
                .iter()
                .zip(&self.tree.children)
                .zip(children_layout.children())
            {
                child.parent.as_widget().draw(
                    tree, renderer, theme, style, layout, cursor, &viewport,
                );
            }
        }
    }

    // fn overlay<'c>(
    //     &'c mut self,
    //     extra_input:f32,
    //     layout: layout::Layout<'_>,
    //     renderer: &Renderer,
    // ) -> Option<MenuOverlayElement<'b, 'c, Message, Theme, Renderer>> {
    // // ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
    // // ) -> Option<MenuTreeOverlay<'c, 'b, Message, Theme, Renderer>> {
    //     println!("mxo overlay");
    //     // overlay::from_children(
    //     //     self.children, 
    //     //     self.tree, 
    //     //     layout.children().next().unwrap(), 
    //     //     renderer
    //     // )
    //     let layout = layout.children().next().unwrap();
    //     let children = self.children
    //         .iter_mut()
    //         .zip(&mut self.tree.children)
    //         .zip(layout.children())
    //         .filter_map(|((child, state), layout)| {
    //             child.overlay(0.0, state, layout, renderer)
    //         })
    //         .collect::<Vec<_>>();

    //     (!children.is_empty()).then(|| overlay::Group::with_children(children).overlay())
    //     // None
    // }

    pub(super) fn is_over(
        &self,
        layout: layout::Layout<'_>,
        _renderer: &Renderer,
        cursor_position: Point,
    ) -> bool {
        let mut lc = layout.children();
        let children_layout = lc.next().unwrap();
        children_layout.bounds().contains(cursor_position)
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


/// A part of a menu where items are displayed.
///
/// When the bounds of a menu exceed the viewport,
/// only items inside the viewport will be displayed,
/// when scrolling happens, this should be updated
#[derive(Debug, Clone, Copy)]
struct MenuSlice {
    start_index: usize,
    end_index: usize,
    lower_bound_rel: f32,
    upper_bound_rel: f32,
}

fn slice(
    children_bounds: Rectangle, // offset + unsrolled
    child_positions: Vec<f32>,
    child_sizes: Vec<Size>,
    scroll_offset: f32,
    viewport_size: Size,
    // overlay_offset: Vector,
) -> MenuSlice {
    // viewport space children bounds
    // let children_bounds = children_bounds + overlay_offset;

    let max_index = child_positions.len().saturating_sub(1);

    // viewport space absolute bounds
    let lower_bound = children_bounds.y.max(0.0);
    let upper_bound = (children_bounds.y + children_bounds.height).min(viewport_size.height);

    // menu space relative bounds
    let lower_bound_rel = lower_bound - (children_bounds.y + scroll_offset);
    let upper_bound_rel = upper_bound - (children_bounds.y + scroll_offset);

    // index range
    let positions = &child_positions;
    let sizes = &child_sizes;

    let start_index = search_bound(0, 0, max_index, lower_bound_rel, positions, sizes);
    let end_index = 
        search_bound(max_index, start_index, max_index, upper_bound_rel, positions, sizes)
        .min(max_index);

    MenuSlice {
        start_index,
        end_index,
        lower_bound_rel,
        upper_bound_rel,
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
    state: &mut MenuTreeState,
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
    state.scroll_offset = (state.scroll_offset + delta_y).clamp(min_offset, max_offset);

    event::Status::Captured
}

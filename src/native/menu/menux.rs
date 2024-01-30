//! doc
//!
use iced_widget::core::{
    event, layout, mouse, overlay, renderer, touch, widget::tree::{self, Tree}, 
    Alignment, Border, Clipboard, Color, Element, Event, Length, 
    Padding, Point, Vector, Rectangle, Shell, Size, Widget,
};
use super::types::*;

struct MenuxState{
    open: bool,
    scroll_offset: f32,
}
impl Default for MenuxState{
    fn default() -> Self {
        Self { open: false, scroll_offset: 0.0 }
    }
}

/// doc
#[allow(clippy::missing_docs_in_private_parents)]
pub struct Menux<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    parent: Element<'a, Message, Theme, Renderer>,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    axis: Axis,
    offset: f32,
    open_condition: OpenCondition,
}
impl<'a, Message, Theme, Renderer> Menux<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    #[allow(missing_docs)]
    pub fn new(parent: Element<'a, Message, Theme, Renderer>, children: Vec<Element<'a, Message, Theme, Renderer>>) -> Self{
        Self{
            parent,
            children,
            spacing: 4.0,
            padding: [0.0;4].into(),
            width: Length::Shrink,
            height: Length::Shrink,
            axis: Axis::Vertical,
            offset: 0.0,
            open_condition: OpenCondition::Hover,
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the [`Padding`] of the [`Menux`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Menux`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Menux`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the axis of the [`Menux`].
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis.into();
        self
    }

    /// Sets the open condition of the [`Menux`].
    pub fn open_condition(mut self, open_condition: OpenCondition) -> Self {
        self.open_condition = open_condition;
        self
    }
}
impl <'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Menux<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn state(&self) -> tree::State {
        tree::State::new(MenuxState::default())
    }

    fn children(&self) -> Vec<Tree> {
        println!("mx children");
        [
            Tree::new(&self.parent),
            Tree{
                children: self.children.iter().map(Tree::new).collect::<Vec<_>>(),
                ..Tree::empty()
            }
        ].into()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        println!("mx layout");
        self.parent.as_widget().layout(&mut tree.children[0], renderer, limits)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        println!("mx event");
        let status = self.parent.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let state = tree.state.downcast_mut::<MenuxState>();
        let bounds = layout.bounds();

        use event::Status::*;
        match self.open_condition{
            OpenCondition::Click => match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if cursor.is_over(bounds) {
                        state.open = true;
                        state.scroll_offset = 0.0;
                        Captured
                    }else{
                        Ignored
                    }
                }
                _ => Ignored
            }
            OpenCondition::Hover => match event {
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    if bounds.contains(position) {
                        state.open = true;
                        state.scroll_offset = 0.0;
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

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        println!("mx draw");
        self.parent.as_widget().draw(&tree.children[0], renderer, theme, style, layout, cursor, viewport)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        println!("mx overlay");
        let mut og = overlay::Group::new();
        let Tree { tag, state, children } = tree;
        
        let [parent_tree, children_tree] = children.as_mut_slice() else { panic!("Tree Error") };
        
        if let Some(c) = self.parent
            .as_widget_mut()
            .overlay(parent_tree, layout, renderer)
        {
            og = og.push(c);
        }

        let ms = state.downcast_mut::<MenuxState>();
        if !ms.open {
            Some(og.overlay())
        }else{
            Some(og.push(
                MenuxOverlay{
                    state: ms,
                    tree: children_tree,
                    children: &mut self.children,
                    parent_bounds: layout.bounds(),
                    max_width: 1000.0,
                    spacing: self.spacing,
                    padding: self.padding,
                    width: self.width,
                    height: self.height,
                    axis: self.axis,
                    offset: self.offset,
                }.overlay()
            ).overlay())
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Menux<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(m: Menux<'a, Message, Theme, Renderer>) -> Self {
        Self::new(m)
    }
}


struct MenuxOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer
{
    state: &'b mut MenuxState,
    tree: &'b mut Tree,
    children: &'b mut [Element<'a, Message, Theme, Renderer>],
    parent_bounds: Rectangle,
    max_width: f32,
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    axis: Axis,
    offset: f32,
}
impl<'a, 'b, Message, Theme, Renderer> MenuxOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer
{
    fn overlay(self) -> overlay::Element<'b, Message, Theme, Renderer>{
        overlay::Element::new(Point::ORIGIN, Box::new(self))
    }
}
impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer> for MenuxOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
        position: iced_widget::core::Point,
        translation: iced_widget::core::Vector,
    ) -> layout::Node {
        println!("mxo layout");
        let limits = layout::Limits::NONE.max_width(self.max_width);
        let layout = layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            Alignment::Center,
            &self.children,
            &mut self.tree.children
        );

        let hcenter = bounds.width / 2.0;
        let vcenter = bounds.height / 2.0;
        
        let phcenter = self.parent_bounds.x + self.parent_bounds.width / 2.0;
        let pvcenter = self.parent_bounds.y + self.parent_bounds.height / 2.0;

        let horizontal_direction = if phcenter < hcenter { Direction::Positive } else { Direction::Negative };
        let vertical_direction   = if pvcenter < vcenter { Direction::Positive } else { Direction::Negative };

        let aod = match self.axis {
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
        };

        let children_size = layout.bounds().size();
        let (children_position, offset_position) = aod.resolve(
            self.parent_bounds + translation, 
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
            layout.move_to(children_position),
            layout::Node::new(bounds),
            layout::Node::new(offset_bounds.size()).move_to(offset_bounds.position()),
            layout::Node::new(check_bounds.size()).move_to(check_bounds.position()),
        ].into()).translate([0.0, self.state.scroll_offset])
    }

    fn on_event(
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
        let viewport = lc.next().unwrap().bounds();
        let offset_bounds = lc.next().unwrap().bounds();
        let check_bounds = lc.next().unwrap().bounds();

        let children_bounds = children_layout.bounds();

        let status = self.children
            .iter_mut()
            .zip(&mut self.tree.children)
            .zip(children_layout.children())
            .map(|((child, tree), layout)| 
                child.as_widget_mut().on_event(
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
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                self.state.open = 
                    self.parent_bounds.contains(position)
                    || offset_bounds.contains(position)
                    || check_bounds.contains(position);
                Captured
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(children_bounds){
                    self.state.scroll_offset += match delta{
                        mouse::ScrollDelta::Lines { x, y } => y,
                        mouse::ScrollDelta::Pixels { x, y } => y,
                    };
                }
                Ignored
            }
            _ => Ignored
        }.merge(status)
    }

    fn draw(
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
        let viewport = lc.next().unwrap().bounds();

        renderer.fill_quad(
            renderer::Quad{
                bounds: self.parent_bounds,
                ..Default::default()
            }, 
            Color::from([1.0, 0.0, 0.0, 0.5])
        );

        renderer.fill_quad(
            renderer::Quad{
                bounds: pad_rectangle(children_layout.bounds(), 4.0.into()),
                border: Border{
                    // color: todo!(),
                    // width: 4.0,
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }, 
            Color::from([1.0; 3])
        );

        if let Some(viewport) = children_layout.bounds().intersection(&viewport) {
            for ((child, tree), layout) in self
                .children
                .iter()
                .zip(&self.tree.children)
                .zip(children_layout.children())
            {
                child.as_widget().draw(
                    tree, renderer, theme, style, layout, cursor, &viewport,
                );
            }
        }
    }

    fn overlay<'c>(
        &'c mut self,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        println!("mxo overlay");
        overlay::from_children(
            self.children, 
            self.tree, 
            layout.children().next().unwrap(), 
            renderer
        )
    }

    fn is_over(
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

enum CheckBound{
    In(Rectangle),
    Out(Rectangle),
}

struct CheckBounds{
    bounds: Vec<Vec<CheckBound>>
}
impl CheckBounds{
    fn check(&self, point:Point, default: bool) -> bool{
        let mut cb = None;
        for bounds in self.bounds.iter().rev(){
            for bound in bounds.iter(){
                match bound {
                    CheckBound::In(rect) | CheckBound::Out(rect) => {
                        if rect.contains(point){
                            cb = Some(bound)
                        }
                    }
                }
            }
        }
        match cb {
            Some(x) => match x {
                CheckBound::In(_) => true,
                CheckBound::Out(_) => false
            }
            None => default
        }
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
    children_bounds: Rectangle,
    child_positions: Vec<f32>,
    child_sizes: Vec<Size>,
    scroll_offset: f32,
    viewport_size: Size,
    overlay_offset: Vector,
    item_height: ItemHeight,
) -> MenuSlice {
    // viewport space children bounds
    let children_bounds = children_bounds + overlay_offset;

    let max_index = child_positions.len().saturating_sub(1);

    // viewport space absolute bounds
    let lower_bound = children_bounds.y.max(0.0);
    let upper_bound = (children_bounds.y + children_bounds.height).min(viewport_size.height);

    // menu space relative bounds
    let lower_bound_rel = lower_bound - (children_bounds.y + scroll_offset);
    let upper_bound_rel = upper_bound - (children_bounds.y + scroll_offset);

    // index range
    let (start_index, end_index) = match item_height {
        ItemHeight::Uniform(u) => {
            let start_index = (lower_bound_rel / f32::from(u)).floor() as usize;
            let end_index = ((upper_bound_rel / f32::from(u)).floor() as usize).min(max_index);
            (start_index, end_index)
        }
        ItemHeight::Static(_) | ItemHeight::Dynamic(_) => {
            let positions = &child_positions;
            let sizes = &child_sizes;

            let start_index = search_bound(0, 0, max_index, lower_bound_rel, positions, sizes);
            let end_index = search_bound(
                max_index,
                start_index,
                max_index,
                upper_bound_rel,
                positions,
                sizes,
            )
            .min(max_index);

            (start_index, end_index)
        }
    };

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

/* fn process_scroll_events<Message, Theme, Renderer>(
    menu: &mut Menu<'_, '_, Message, Theme, Renderer>,
    delta: mouse::ScrollDelta,
    overlay_cursor: Point,
    viewport_size: Size,
    overlay_offset: Vector,
) -> event::Status
where
    Renderer: renderer::Renderer,
    Theme: StyleSheet,
{
    use event::Status::{Captured, Ignored};
    use mouse::ScrollDelta;

    let state = menu.tree.state.downcast_mut::<MenuBarState>();

    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } => y * 60.0,
        ScrollDelta::Pixels { y, .. } => y,
    };

    let calc_offset_bounds = |menu_state: &MenuState, viewport_size: Size| -> (f32, f32) {
        // viewport space children bounds
        let children_bounds = menu_state.menu_bounds.children_bounds + overlay_offset;

        let max_offset = (0.0 - children_bounds.y).max(0.0);
        let min_offset =
            (viewport_size.height - (children_bounds.y + children_bounds.height)).min(0.0);
        (max_offset, min_offset)
    };

    // update
    if state.menu_states.is_empty() {
        return Ignored;
    } else if state.menu_states.len() == 1 {
        let last_ms = &mut state.menu_states[0];

        if last_ms.index.is_none() {
            return Captured;
        }

        let (max_offset, min_offset) = calc_offset_bounds(last_ms, viewport_size);
        last_ms.scroll_offset = (last_ms.scroll_offset + delta_y).clamp(min_offset, max_offset);
    } else {
        // >= 2
        let max_index = state.menu_states.len() - 1;
        let last_two = &mut state.menu_states[max_index - 1..=max_index];

        if last_two[1].index.is_some() {
            // scroll the last one
            let (max_offset, min_offset) = calc_offset_bounds(&last_two[1], viewport_size);
            last_two[1].scroll_offset =
                (last_two[1].scroll_offset + delta_y).clamp(min_offset, max_offset);
        } else {
            if !last_two[0]
                .menu_bounds
                .children_bounds
                .contains(overlay_cursor)
            {
                return Captured;
            }

            // scroll the second last one
            let (max_offset, min_offset) = calc_offset_bounds(&last_two[0], viewport_size);
            let scroll_offset = (last_two[0].scroll_offset + delta_y).clamp(min_offset, max_offset);
            let clamped_delta_y = scroll_offset - last_two[0].scroll_offset;
            last_two[0].scroll_offset = scroll_offset;

            // update the bounds of the last one
            last_two[1].menu_bounds.parent_bounds.y += clamped_delta_y;
            last_two[1].menu_bounds.children_bounds.y += clamped_delta_y;
            last_two[1].menu_bounds.check_bounds.y += clamped_delta_y;
        }
    }
    Captured
}
*/


use std::f32::consts::E;

use iced_native::{
    Element, Widget, renderer, overlay, 
    layout, Length, Color, Point, Size, 
    Padding, Rectangle, Alignment, event,
    mouse, touch, Shell, Clipboard,
};
use iced_native::widget::{tree, Tree, };
use super::menu_tree::MenuTree;
use crate::style::menu_bar::StyleSheet;

/// Methods for drawing path highlight
#[derive(Debug, Clone)]
pub enum PathHighlight{
    /// Draw the full path,
    Full,
    /// Omit the active item
    OmitActive,
    /// Omit the active item if it's not a menu
    MenuActive
}

/* TODO
pub enum ItemHeight{
    Uniform(f32),
    Static,
    // Flex,
}

pub enum ItemWidth{
    Uniform(f32),
    Static,
}
*/

#[derive(Debug, Clone, Copy)]
struct MenuSlice{
    start_index: usize,
    end_index: usize,
    lower_bound_rel: f32,
    upper_bound_rel: f32,
}

struct MenuBounds{
    child_positions: Vec<f32>,
    children_bounds: Rectangle,
    parent_bounds: Rectangle,
    check_bounds: Rectangle,
}
impl MenuBounds{
    fn new(
        children_count: usize,
        item_size: Size,
        viewport: Size,
        aod_settings: [bool;4],
        bounds_expand: u16,
        parent_bounds: Rectangle,
    ) -> Self{
        let children_size = Size::new(
            item_size.width, 
            item_size.height * children_count as f32
        );

        let (children_position, mask) = adaptive_open_direction(parent_bounds, children_size, viewport, aod_settings);
        
        let children_bounds = Rectangle::new(children_position, children_size);

        let mut padding = [0;4];
        padding.iter_mut().enumerate().for_each(|(i, p)| {
            *p = mask[i] * bounds_expand;
        });
        
        let child_positions = (0..children_count).map(|i|{
            (i as f32) * item_size.height
        }).collect();

        let check_bounds = pad_rectangle(children_bounds, padding.into());
        Self{
            children_bounds,
            child_positions,
            parent_bounds,
            check_bounds,
        }
    }
}

struct MenuState{
    index: Option<usize>,
    scroll_offset: f32,
    menu_bounds: MenuBounds,
}
impl MenuState{
    fn layout<'a, Message, Renderer>(
        &self, 
        slice: MenuSlice,
        item_size: Size,
        renderer: &Renderer,
        menu_tree: &MenuTree<'a, Message, Renderer>,
    ) -> layout::Node
    where
        Renderer: renderer::Renderer,
    {
        let MenuSlice{
            start_index,
            end_index,
            lower_bound_rel,
            upper_bound_rel,
        } = slice;

        let child_nodes = self.menu_bounds.child_positions[start_index..=end_index].iter()
        .zip(menu_tree.children[start_index..=end_index].iter())
        .map(|(cp, mt)|{
            let mut position = *cp;
            let mut size = item_size;

            if position < lower_bound_rel
            && (position + size.height) > lower_bound_rel{
                size.height = position + size.height - lower_bound_rel;
                position = lower_bound_rel;
            }else if position <= upper_bound_rel
            && (position + size.height) > upper_bound_rel{
                size.height = upper_bound_rel - position;
            }

            let limits = layout::Limits::new(Size::ZERO, size);

            let mut node = mt.item.as_widget().layout(renderer, &limits);
            node.move_to(Point::new(
                0.0,
                position + self.scroll_offset,
            ));
            node
        }).collect::<Vec<_>>();

        let mut node = layout::Node::with_children(self.menu_bounds.children_bounds.size(), child_nodes);
        node.move_to(self.menu_bounds.children_bounds.position());
        node
    }

    fn layout_single<'a, Message, Renderer>(
        &self, 
        index: usize,
        item_size: Size,
        renderer: &Renderer,
        menu_tree: &MenuTree<'a, Message, Renderer>,
    ) -> layout::Node
    where
        Renderer: renderer::Renderer,
    {
        let mut position = self.menu_bounds.child_positions[index];
        let mut size = item_size;

        let limits = layout::Limits::new(Size::ZERO, size);
        let parent_offset = self.menu_bounds.children_bounds.position() - Point::ORIGIN;
        let mut node = menu_tree.item.as_widget().layout(renderer, &limits);
        node.move_to(Point::new(
            parent_offset.x,
            parent_offset.y + position + self.scroll_offset,
        ));
        node
    }

    fn slice(&self, viewport: Size, item_size: Size) -> MenuSlice{
        let children_bounds = self.menu_bounds.children_bounds;
        let max_index = self.menu_bounds.child_positions.len().saturating_sub(1);
        
        // absolute bounds
        let lower_bound = children_bounds.y.max(0.0);
        let upper_bound = (children_bounds.y + children_bounds.height).min(viewport.height);

        // relative bounds
        let lower_bound_rel = lower_bound - (children_bounds.y + self.scroll_offset);
        let upper_bound_rel = upper_bound - (children_bounds.y + self.scroll_offset);

        // index range
        let start_index = (lower_bound_rel / item_size.height).floor() as usize;
        let end_index = ((upper_bound_rel / item_size.height).floor() as usize).min(max_index);
        
        MenuSlice{
            start_index,
            end_index,
            lower_bound_rel,
            upper_bound_rel,
        }
    }
}

struct MenuBarState{
    pressed: bool,
    cursor: Point,
    open: bool,
    active_root: Option<usize>,
    menu_states: Vec<MenuState>,
}
impl MenuBarState{
    fn get_trimmed_indices(&self) -> impl Iterator<Item = usize> + '_{
        self.menu_states.iter()
            .take_while(|ms| ms.index.is_some() )
            .map(|ms| ms.index.unwrap() )
    }

    fn get_trimmed_ref(&self) -> impl Iterator<Item = &MenuState> + '_{
        self.menu_states.iter()
            .take_while(|ms| ms.index.is_some())
    }

    fn get_trimmed_mut(&mut self) -> impl Iterator<Item = &mut MenuState> + '_{
        self.menu_states.iter_mut()
            .take_while(|ms| ms.index.is_some())
    }

    fn reset(&mut self){
        self.open = false;
        self.active_root = None;
        self.menu_states.clear();
    }
}
impl Default for MenuBarState{
    fn default() -> Self {
        Self{
            pressed: false,
            cursor: Point::new(-0.5, -0.5),
            open: false,
            active_root: None,
            menu_states: Vec::new(),
        }
    }
}


/// A `MenuBar` collects `MenuTree`s and handles 
/// all the layout, event processing and drawing
#[allow(missing_debug_implementations)]
pub struct MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    width: Length,
    height: Length,
    spacing: u16,
    padding: Padding,
    bounds_expand: u16,
    item_size: Size,
    path_highlight: Option<PathHighlight>,
    menu_roots: Vec<MenuTree<'a, Message, Renderer>>,
    style: <Renderer::Theme as StyleSheet>::Style,
}
impl<'a, Message, Renderer> MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`MenuBar`] with the given menu roots
    pub fn new(menu_roots: Vec<MenuTree<'a, Message, Renderer>>) -> Self{
        let mut menu_roots = menu_roots;
        menu_roots.iter_mut().for_each(|mr| mr.set_index() );

        Self{
            width: Length::Shrink,
            height: Length::Shrink,
            spacing: 0,
            padding: Padding::ZERO,
            bounds_expand: 15,
            item_size: [150.0, 30.0].into(),
            path_highlight: Some(PathHighlight::MenuActive),
            menu_roots,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the width of the [`MenuBar`]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`MenuBar`]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the spacing between menu roots
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the expand value for each menu's check bounds
    /// 
    /// When the cursor goes outside of a menu's check bounds, 
    /// the menu will be closed automatically, this value expands
    /// the check bounds
    pub fn bounds_expand(mut self, value: u16) -> Self {
        self.bounds_expand = value;
        self
    }

    /// Sets the [`Padding`] of the [`MenuBar`]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the [`Size`] of each menu item
    pub fn item_size(mut self, item_size: impl Into<Size>) -> Self{
        self.item_size = item_size.into();
        self
    }

    /// Sets the method for drawing path highlight
    pub fn path_highlight(mut self, path_highlight: Option<PathHighlight>) -> Self{
        self.path_highlight = path_highlight;
        self
    }

    /// Sets the style of the menu bar and its menus
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self{
        self.style = style.into();
        self
    }

} 
impl<'a, Message, Renderer> Widget<Message, Renderer> for MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }
    
    fn state(&self) -> tree::State {
        tree::State::new(MenuBarState::default())
    }

    fn children(&self) -> Vec<Tree> {
        /*
        menu bar
            menu root 1 (stateless)
                flat tree
            menu root 2 (stateless)
                flat tree
            ...
        */
        
        self.menu_roots.iter().map(|root|{
            let mut tree = Tree::empty();
            let flat = root.flattern().iter().map(|mt|{
                Tree::new(mt.item.as_widget())
            }).collect();
            tree.children = flat;
            tree
        }).collect()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        use super::flex;

        let limits = limits.width(self.width).height(self.height);
        let children = self.menu_roots.iter().map(|root| &root.item ).collect::<Vec<_>>();
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            &limits,
            self.padding,
            self.spacing as f32,
            Alignment::Center,
            &children,
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        use event::{Event::*, Status::*};
        use mouse::{Event::*, Button::Left};
        use touch::{Event::*, };

        let root_status = process_root_events(
            &mut self.menu_roots, 
            cursor_position, 
            tree, 
            event.clone(), 
            layout, 
            renderer, 
            clipboard, 
            shell
        );

        let state = tree.state.downcast_mut::<MenuBarState>();

        match event {
            Mouse(ButtonReleased(Left)) |
            Touch(FingerLifted {..}) |
            Touch(FingerLost {..})  => {
                if state.menu_states.is_empty()
                && layout.bounds().contains(cursor_position){
                    state.cursor = cursor_position;
                    state.open = true;
                }
            },
            _ => (),
        }
        root_status
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as renderer::Renderer>::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<MenuBarState>();

        let position = if state.open
        && (cursor_position.x < 0.0
        || cursor_position.y < 0.0){
            state.cursor
        }else{
            cursor_position
        };

        // draw path highlight
        if let Some(_) = self.path_highlight{
            let styling = theme.appearance(&self.style);
            if let Some(active) = state.active_root{
                let active_bounds = layout.clone().children()
                    .skip(active).next().unwrap().bounds();
                let path_quad = renderer::Quad{
                    bounds: active_bounds,
                    border_radius: styling.border_radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                };
                let path_color = styling.path;
                renderer.fill_quad(path_quad, path_color);
            }
        }

        self.menu_roots.iter()
        .zip(&tree.children)
        .zip(layout.children())
        .for_each(|((root, t), lo)|{
            root.item.as_widget().draw(
                &t.children[root.index], 
                renderer, 
                theme, 
                style, 
                lo,
                position,
                viewport
            );
        })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let state = tree.state.downcast_ref::<MenuBarState>();
        if !state.open { return None; }

        Some(Menu{
            tree,
            menu_roots: &mut self.menu_roots,
            bounds_expand: self.bounds_expand,
            item_size: self.item_size,
            bar_bounds: layout.bounds(),
            root_bounds_list: layout.children()
                .map(|lo| lo.bounds())
                .collect(),
            path_highlight: self.path_highlight.clone(),
            style: &self.style,
        }.overlay())
    }
}
impl<'a, Message, Renderer> From<MenuBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(value: MenuBar<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}


struct Menu<'a, 'b, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    tree: &'b mut Tree,
    menu_roots: &'b mut Vec<MenuTree<'a, Message, Renderer>>,
    bounds_expand: u16,
    item_size: Size,
    bar_bounds: Rectangle,
    root_bounds_list: Vec<Rectangle>,
    path_highlight: Option<PathHighlight>,
    style: &'b <Renderer::Theme as StyleSheet>::Style,
}
impl<'a, 'b, Message, Renderer> Menu<'a, 'b, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn overlay(self) -> overlay::Element<'b, Message, Renderer>{
        overlay::Element::new(
            Point::ORIGIN,
            Box::new(self)
        )
    }
}
impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer> for Menu<'a, 'b, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(
        &self,
        _renderer: &Renderer,
        bounds: Size,
        _position: Point,
    ) -> layout::Node {
        layout::Node::new(bounds)
    }

    fn on_event(
        &mut self,
        event: event::Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        use event::{Event::*, Status::*};
        use mouse::{Event::*, ScrollDelta, Button::Left};
        use touch::{Event::*, };

        if !self.tree.state.downcast_ref::<MenuBarState>().open { return Ignored; };

        let viewport = layout.bounds().size();

        let menu_status = process_menu_events(
            &mut self.tree, 
            &mut self.menu_roots, 
            self.item_size,
            event.clone(), 
            cursor_position, 
            renderer, 
            clipboard, 
            shell
        );

        init_root_menu(
            self, 
            cursor_position, 
            viewport, 
            self.bar_bounds
        );

        match event {
            Mouse(WheelScrolled { delta }) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();

                let delta_y = match delta{
                    ScrollDelta::Lines { y, .. } => y * 60.0,
                    ScrollDelta::Pixels { y, .. } => y,
                };

                // get menu
                let last_ms = state.get_trimmed_mut().last().unwrap();
                let children_bounds = last_ms.menu_bounds.children_bounds;
                
                let max_offset = (0.0 - children_bounds.y).max(0.0);
                let min_offset = (viewport.height - (children_bounds.y + children_bounds.height)).min(0.0);
                
                // println!("max: {max_offset}, min: {min_offset}");
                last_ms.scroll_offset = (last_ms.scroll_offset + delta_y).clamp(min_offset, max_offset);
                
                Captured
            },

            Mouse(CursorMoved { position }) |
            Touch(FingerMoved { position,.. }) => {
                process_overlay_events(self, viewport, position).merge(menu_status)
            },
            
            Mouse(ButtonPressed(Left)) |
            Touch(FingerPressed {..}) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();
                state.pressed = true;
                Captured
            },

            Mouse(ButtonReleased(Left)) |
            Touch(FingerLifted {..}) => {
                let state = self.tree.state.downcast_mut::<MenuBarState>();
                state.pressed = false;

                if self.bar_bounds.contains(cursor_position){
                    state.reset();
                    state.cursor = cursor_position;
                    Captured
                }else{
                    menu_status
                }
            },
            _ => menu_status
        }
    }
    
    #[allow(unused_results)]
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor_position: Point,
    ) {
        let styling = theme.appearance(self.style);

        let state = self.tree.state.downcast_ref::<MenuBarState>();
        let Some(active_root) = state.active_root else{ return; };

        let tree = &self.tree.children[active_root].children;
        let root = &self.menu_roots[active_root];
        
        let indices = state.get_trimmed_indices().collect::<Vec<_>>();

        state.menu_states.iter().enumerate()
        .fold(root, |menu_root, (i, ms)|{

            let draw_path = match &self.path_highlight{
                Some(ph) => match ph{
                    PathHighlight::Full => true,
                    PathHighlight::OmitActive => {
                        indices.len() > 0 && i < indices.len() - 1
                    },
                    PathHighlight::MenuActive => {
                        i < state.menu_states.len() - 1
                    }
                },
                None => false,
            };

            let draw_menu = |r: &mut Renderer|{
                // calc slice
                let slice = ms.slice(layout.bounds().size(), self.item_size);
                let start_index = slice.start_index;
                let end_index = slice.end_index;

                // calc layout
                let children_node = ms.layout(
                    slice,
                    self.item_size, 
                    r, 
                    menu_root
                );
                let children_layout = layout::Layout::new(&children_node);
                let children_bounds = children_layout.bounds();


                // draw menu background
                let menu_quad = renderer::Quad{
                    bounds: pad_rectangle(children_bounds, styling.background_expand.into()),
                    border_radius: styling.border_radius.into(),
                    border_width: styling.border_width,
                    border_color: styling.border_color,
                };
                let menu_color = styling.background;
                r.fill_quad(menu_quad, menu_color);


                // draw path hightlight
                if let (true, Some(active)) = (draw_path, ms.index){
                    let active_bounds = children_layout.children()
                        .skip(active - start_index).next().unwrap().bounds();
                    let path_quad = renderer::Quad{
                        bounds: active_bounds,
                        border_radius: styling.border_radius.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    };
                    let path_color = styling.path;
                    r.fill_quad(path_quad, path_color);
                }


                // draw item
                menu_root.children[start_index..=end_index].iter()
                .zip(children_layout.children())
                .for_each(|(mt, clo)|{
                    mt.item.as_widget().draw(
                        &tree[mt.index], 
                        r, 
                        theme, 
                        style, 
                        clo, 
                        cursor_position, 
                        &children_layout.bounds()
                    );
                });
            };

            renderer.with_layer(layout.bounds(), draw_menu);

            // only the last menu can have a None active index
            if let Some(active) = ms.index {
                &menu_root.children[active]
            }else{
                menu_root
            }
        });
    }
    
}


fn pad_rectangle(rect: Rectangle, padding: Padding) -> Rectangle{
    Rectangle{
        x: rect.x - padding.left as f32,
        y: rect.y - padding.top as f32,
        width: rect.width + padding.horizontal() as f32,
        height: rect.height + padding.vertical() as f32,
    }
}

#[allow(unused_results)]
fn process_root_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    menu_roots: &mut Vec<MenuTree<'a, Message, Renderer>>,
    position: Point,
    tree: &mut Tree,
    event: event::Event,
    layout: layout::Layout<'_>,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) -> event::Status {
    menu_roots.iter_mut()
        .zip(&mut tree.children)
        .zip(layout.children())
        .map(|((root, t), lo)|{
            // assert!(t.tag == tree::Tag::stateless());
            root.item.as_widget_mut().on_event(
                &mut t.children[root.index], 
                event.clone(), 
                lo, 
                position, 
                renderer, 
                clipboard, 
                shell
            )
        }).fold(event::Status::Ignored, event::Status::merge)
}

fn init_root_menu<'a, 'b, Message, Renderer>(
    menu: &mut Menu<'a, 'b, Message, Renderer>,
    position: Point,
    viewport: Size,
    bar_bounds: Rectangle,
)
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    let state = menu.tree.state.downcast_mut::<MenuBarState>();
    if !(state.menu_states.is_empty()
    && bar_bounds.contains(position)){ return; }

    for (i, (&parent_bounds, mt)) in menu.root_bounds_list.iter()
    .zip(menu.menu_roots.iter())
    .enumerate(){
        if parent_bounds.contains(position){
            let menu_bounds = MenuBounds::new(
                mt.children.len(),
                menu.item_size,
                viewport, 
                [false, true, true, false], 
                menu.bounds_expand,
                parent_bounds
            );

            state.active_root = Some(i);
            state.menu_states.push(MenuState{
                index: None,
                scroll_offset: 0.0,
                menu_bounds,
            });

            break;
        }
    }
}

fn process_menu_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    tree: &'b mut Tree,
    menu_roots: &'b mut Vec<MenuTree<'a, Message, Renderer>>,
    item_size: Size,
    event: event::Event,
    cursor_position: Point,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) -> event::Status {
    use event::Status;

    let state = tree.state.downcast_mut::<MenuBarState>();
    let Some(active_root) = state.active_root else { return Status::Ignored; };
    
    let indices = state.get_trimmed_indices().collect::<Vec<_>>();

    if indices.is_empty() { return Status::Ignored; }
    
    // get active item
    let mt = indices.iter()
        .fold(&mut menu_roots[active_root], |mt, &i|{
            &mut mt.children[i]
        });
    
    // get layout
    let last_ms = &state.menu_states[indices.len() - 1];
    let child_node = last_ms.layout_single(
        last_ms.index.unwrap(), 
        item_size, 
        renderer, 
        mt,
    );
    let child_layout = layout::Layout::new(&child_node);
    
    // widget tree
    let tree = &mut tree.children[active_root].children[mt.index];

    // process only the last widget
    mt.item.as_widget_mut().on_event(
        tree, 
        event, 
        child_layout,
        cursor_position, 
        renderer, 
        clipboard, 
        shell
    )
}

#[allow(unused_results)]
fn process_overlay_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    menu: &mut Menu<'a, 'b, Message, Renderer>,
    viewport: Size,
    position: Point,
) -> event::Status
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    use event::Status::*;
    /* 
    if no active root  || pressed:
        return
    else:
        remove invalid menu
        update active item
        if active item is a menu:
            add menu
    */

    let state = menu.tree.state.downcast_mut::<MenuBarState>();

    /* When overlay is running, cursor_position in any widget method will go negative
    but I still want Widget::draw() to react to cursor movement */
    state.cursor = position;

    
    let Some(active_root) = state.active_root else{
        return Ignored;
    };

    if state.pressed { return Ignored; }
    

    // remove invalid menus
    for i in (0..state.menu_states.len()).rev(){
        let mb = &state.menu_states[i].menu_bounds;

        if mb.parent_bounds.contains(position)
        || mb.check_bounds.contains(position){
            break;
        }
        state.menu_states.pop();
    }


    // update active item
    let Some(last_ms) = state.menu_states.last_mut() else{
        // no menus left
        state.active_root = None;

        // keep state.open when the cursor is still inside the menu bar
        // this allows the overlay to keep drawing when the cursor is
        // moving aroung the menu bar
        if !menu.bar_bounds.contains(position){
            state.open = false;
        }
        return Captured;
    };

    let last_mb = &last_ms.menu_bounds;
    let last_pb = last_mb.parent_bounds;
    let last_cb = last_mb.children_bounds;

    if last_pb.contains(position){
        // cursor is in the parent part
        last_ms.index = None;
        return Captured;
    }
    // cursor is in the children part

    // calc new index
    let height_diff = (position.y - (last_cb.y + last_ms.scroll_offset))
        .clamp(0.0, last_cb.height - 0.001);
    let new_index = (height_diff / menu.item_size.height).floor() as usize;
    
    // set new index
    last_ms.index = Some(new_index);

    // get new item bounds
    // let item_bounds = last_mb.child_bounds[new_index] + 
    // (last_mb.children_bounds.position() - Point::ORIGIN);
    let item_bounds = Rectangle::new(
        Point::new(0.0, last_mb.child_positions[new_index] + last_ms.scroll_offset),
        menu.item_size
    ) + (last_mb.children_bounds.position() - Point::ORIGIN);
    
    // get new active item
    let active_menu_root = &menu.menu_roots[active_root];
    let item = state.get_trimmed_indices()
        .fold(active_menu_root, |mt, i|{
            &mt.children[i]
        });
    
    // add new menu if the new item is a menu
    if !item.children.is_empty(){
        state.menu_states.push(MenuState{
            index: None,
            scroll_offset: 0.0,
            menu_bounds: MenuBounds::new(
                item.children.len(),
                menu.item_size,
                viewport, 
                [true, true, false, true], 
                menu.bounds_expand, 
                item_bounds,
            ),
        });
    }

    Captured
}

fn adaptive_open_direction(
    parent_bounds: Rectangle,
    children_size: Size,
    bounds: Size,
    setttings: [bool;4],
) -> (Point, [u16;4]) {
    /*
    Imagine there're two sticks, parent and child
    parent: o-----o
    child:  o----------o

    Now we align the child to the parent in one dimension
    There are 4 possibilities:

    1. to the right
                o-----oo----------o

    2. to the right but allow overlaping
                o-----o
                o----------o

    3. to the left
    o----------oo-----o

    4. to the left but allow overlaping
                o-----o
           o----------o

    The child goes to the right by default, if the right space runs out
    it goes to the left, whether to use overlap is the caller's decision

    This can be applied to any direction
    */

    let [horizontal, vertical, horizontal_overlap, vertical_overlap] = setttings;

    let calc_adaptive = |parent_pos, parent_size, child_size, max_size, on, overlap|{
        let mut padding_mask = [1,1];

        if on{
            let space_left = parent_pos;
            let space_right = max_size - parent_pos - parent_size;
    
            if space_left > space_right && child_size > space_right{
                let value = if overlap{
                    parent_pos - child_size + parent_size
                }else{
                    padding_mask[1] = 0;
                    parent_pos - child_size
                };
                return (value, padding_mask)
            }
        }

        let value = if overlap{
            parent_pos
        }else{
            padding_mask[0] = 0;
            parent_pos + parent_size
        };
        return (value, padding_mask)
    };

    let (x, px) = calc_adaptive(
        parent_bounds.x, parent_bounds.width, 
        children_size.width, bounds.width, 
        horizontal, horizontal_overlap
    );
    let (y, py) = calc_adaptive(
        parent_bounds.y, parent_bounds.height, 
        children_size.height, bounds.height, 
        vertical, vertical_overlap
    );

    let padding_mask = [py[0], px[1], py[1], px[0]];

    ([x,y].into(), padding_mask)

}


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


struct MenuBarState{
    pressed: bool,
    cursor: Point,
    open: bool,
    active_root: Option<usize>,
    indices: Vec<Option<usize>>,
}
impl MenuBarState{
    fn get_flat_indices(&self) -> impl Iterator<Item = usize> + '_{
        self.indices.iter()
            .take_while(|i| i.is_some() )
            .map(|i| i.unwrap() )
    }
}
impl Default for MenuBarState{
    fn default() -> Self {
        Self{
            pressed: false,
            cursor: Point::new(-0.5, -0.5),
            open: false,
            active_root: None,
            indices: Vec::new(),
        }
    }
}


/// A `MenuBar` collects `MenuTree`s and handles 
/// all the layout, event processing and drawing
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

        process_root_events(
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
                if state.indices.is_empty()
                && layout.bounds().contains(cursor_position){
                    init_root_menu(
                        state, 
                        layout.children().map(|lo| lo.bounds() ), 
                        cursor_position
                    );
                    state.cursor = cursor_position;
                    state.open = true;
                }
            },
            _ => (),
        }
        Ignored
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
        renderer: &Renderer,
        bounds: Size,
        _position: Point,
    ) -> layout::Node {
        /* 
        Each menu_node corresponds to a children_node, a parent_node, and a check_node
        
        A children node is a rectangle of the children part, a bounding box of its child nodes

        A parent node is a rectangle of the parent part.

        A check node is a rectangle for iniside checks, usually an expanded version of 
        the children_node

        These are info needed by on_event() and draw()
        */

        /* 
        root_node
            menu_node_a
                children_node
                    child_node
                    child_node
                    ...
                parent_node
                check_node
            menu_node_b
                children_node
                    child_node
                    child_node
                    ...
                parent_node
                check_node
            ...
        */
        
        let state = self.tree.state.downcast_ref::<MenuBarState>();

        let Some(active_root) = state.active_root else{
            return layout::Node::new(Size::INFINITY);
        };
        let root_bounds = self.root_bounds_list[active_root];
        let item_width = self.item_size.width;
        let item_height = self.item_size.height;

        // init
        let mut menu_root = &self.menu_roots[active_root];
        let mut parent_bounds = root_bounds;
        let mut aop_settings = [false, true, true, false];

        // calc menu nodes
        let menu_nodes = state.indices.iter().map(|i|{
            let children_count = menu_root.children.len() as f32;
            let children_size = Size::new(item_width, item_height * children_count);
            let (pos, mask) = adaptive_open_direction(parent_bounds, children_size, bounds, aop_settings);
            let children_bounds = Rectangle::new(
                pos,
                children_size,
            );

            // calc child nodes
            let child_limits = layout::Limits::new(Size::ZERO, self.item_size);
            let child_nodes = menu_root.children.iter().enumerate().map(|(j, mt)|{
                let mut node = mt.item.as_widget().layout(renderer, &child_limits);
                let center_offset = (self.item_size.height - node.size().height)*0.5;
                node.move_to(Point::new(0.0, (j as f32) * item_height + center_offset));
                node
            }).collect::<Vec<_>>();

            // calc parent node
            let mut parent_node = layout::Node::new(parent_bounds.size());
            parent_node.move_to(parent_bounds.position());

            // calc check node
            let mut padding = [0;4];
            padding.iter_mut().enumerate().for_each(|(i, p)| {
                *p = mask[i] * self.bounds_expand;
            });
            
            let check_bounds = pad_rectangle(children_bounds, padding.into());
            let mut check_node = layout::Node::new(check_bounds.size());
            check_node.move_to(check_bounds.position());

            // update for next iteration
            if let Some(active) = i{
                menu_root = &menu_root.children[*active];
                parent_bounds = child_nodes[*active].bounds() + 
                    (children_bounds.position() - Point::ORIGIN);
                aop_settings = [true, true, false, true];
            }

            // calc children node
            let mut children_node = layout::Node::with_children(children_bounds.size(), child_nodes);
            children_node.move_to(children_bounds.position());

            // menu node
            layout::Node::with_children(
                Size::INFINITY, 
                vec![children_node, parent_node, check_node]
            )
        }).collect::<Vec<_>>();

        let root_node = layout::Node::with_children(Size::INFINITY, menu_nodes);
        root_node
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
        use mouse::{Event::*, Button::Left};
        use touch::{Event::*, };

        process_menu_events(
            &mut self.tree, 
            &mut self.menu_roots, 
            event.clone(), 
            layout, 
            cursor_position, 
            renderer, 
            clipboard, 
            shell
        );

        match event {
            Mouse(CursorMoved { position }) |
            Touch(FingerMoved { position,.. }) => process_overlay_events(self, layout, position),
            
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
                    state.open = false;
                    state.indices.clear();
                    state.active_root = None;
                    state.cursor = cursor_position;
                    Captured
                }else{
                    Ignored
                }
            },
            _ => Ignored
        }
    }

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
        
        let indices = state.get_flat_indices().collect::<Vec<_>>();

        state.indices.iter().enumerate()
            .zip(layout.children())
            .fold(root, |menu_root, ((e, i), menu_layout)|{
            
            let mut c = menu_layout.children();
            let children_layout = c.next().unwrap();
            let children_bounds = children_layout.bounds();

            let draw_menu_background = |r: &mut Renderer|{
                let menu_quad = renderer::Quad{
                    bounds: pad_rectangle(children_bounds, styling.background_expand.into()),
                    border_radius: styling.border_radius.into(),
                    border_width: styling.border_width,
                    border_color: styling.border_color,
                };
                let menu_color = styling.background;
                r.fill_quad(menu_quad, menu_color);
            };

            let draw_path_highlight = |r: &mut Renderer, active:usize|{
                let active_bounds = children_layout.children()
                    .skip(active).next().unwrap().bounds();
                let path_quad = renderer::Quad{
                    bounds: active_bounds,
                    border_radius: styling.border_radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                };
                let path_color = styling.path;
                r.fill_quad(path_quad, path_color);
            };

            let draw_items = |r: &mut Renderer|{
                menu_root.children.iter()
                    .zip(children_layout.clone().children())
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

            let draw_path = match &self.path_highlight{
                Some(ph) => match ph{
                    PathHighlight::Full => true,
                    PathHighlight::OmitActive => {
                        indices.len() > 0 && e < indices.len() - 1
                    },
                    PathHighlight::MenuActive => {
                        e < state.indices.len() - 1
                    }
                },
                None => false,
            };

            let draw_menu = |r: &mut Renderer|{
                draw_menu_background(r);

                if let (true, Some(active)) = (draw_path, i){
                    draw_path_highlight(r, *active);
                }

                draw_items(r);
            };

            renderer.with_layer(layout.bounds(), draw_menu);

            // only the last menu can have a None active index
            if let Some(active) = i {
                &menu_root.children[*active]
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

fn process_root_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    menu_roots: &mut Vec<MenuTree<'a, Message, Renderer>>,
    position: Point,
    tree: &mut Tree,
    event: event::Event,
    layout: layout::Layout<'_>,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) -> event::Status{
    use event::Status;
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
        }).fold(Status::Ignored, Status::merge)
}

fn init_root_menu<T, B>(
    state: &mut MenuBarState,
    root_bounds_list: T,
    position: Point,
) -> Option<usize>
where
    T: Iterator<Item = B>,
    B: std::borrow::Borrow<Rectangle>,
{
    for (i, rect) in root_bounds_list.enumerate(){
        if rect.borrow().contains(position){
            state.active_root = Some(i);
            state.indices.push(None);
            return Some(i);
        }
    }
    return None;
}

fn process_menu_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    tree: &'b mut Tree,
    menu_roots: &'b mut Vec<MenuTree<'a, Message, Renderer>>,
    event: event::Event,
    layout: layout::Layout<'_>,
    cursor_position: Point,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
) -> event::Status {
    use event::Status;

    let state = tree.state.downcast_mut::<MenuBarState>();
    let Some(active_root) = state.active_root else { return Status::Ignored; };
    
    let indices = state.get_flat_indices().collect::<Vec<_>>();

    if indices.is_empty() { return Status::Ignored; }
    
    // get active item
    let mt = indices.iter()
        .fold(&mut menu_roots[active_root], |mt, &i|{
            &mut mt.children[i]
        });
    
    // get layout
    let last_index = indices.last().unwrap();
    let menu_layout = layout.children().skip(indices.len()-1).next().unwrap();
    let children_layout = menu_layout.children().next().unwrap();
    let child_layout = children_layout.children().skip(*last_index).next().unwrap();
    
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

    // state.indices.iter()
    //     .zip(layout.children())
    //     .fold(root, |menu_root, (i, mlo)|{
    //     menu_root.children.iter_mut()
    //         .zip(mlo.children().skip(1))
    //         .for_each(|(mt, clo)|{
    //         mt.item.as_widget_mut().on_event(
    //             &mut tree[mt.index], 
    //             event.clone(), 
    //             clo, 
    //             cursor_position, 
    //             renderer, 
    //             clipboard, 
    //             shell
    //         );
    //     });
    // 
    //     if let Some(active) = i {
    //         &mut menu_root.children[*active]
    //     }else {
    //         menu_root
    //     }
    // });
}

fn process_overlay_events<'a, 'b, Message, Renderer: renderer::Renderer>(
    menu: &mut Menu<'a, 'b, Message, Renderer>,
    layout: layout::Layout<'_>,
    position: Point,
) -> event::Status
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    use event::Status::*;
    /* 
    if no active root:
        init root menu
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

    // init
    let Some(active_root) = state.active_root else{
        init_root_menu(state, menu.root_bounds_list.iter(), position);
        return Ignored;
    };

    if state.pressed { return Ignored; }
    
    // get menu_bounds
    let mut menu_bounds = layout.children() // menu_nodes
        .map(|lo|{
            let mut c = lo.children();
            let children_bounds = c.next().unwrap().bounds();
            let parent_bounds = c.next().unwrap().bounds();
            let check_bounds = c.next().unwrap().bounds();
            [children_bounds, parent_bounds, check_bounds]
        }).collect::<Vec<_>>();
    
    if state.indices.len() != menu_bounds.len() { return Ignored; }
    // assert_eq!(state.indices.len(), menu_bounds.len());

    // remove invalid menu
    for i in (0..state.indices.len()).rev(){
        let [_, pb, ckb] = menu_bounds[i];

        if pb.contains(position) || ckb.contains(position){
            break;
        }
        state.indices.pop();
        menu_bounds.pop();
    }


    // update active item
    let Some(last_index) = state.indices.last_mut() else{
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

    let [last_cb, last_pb, _] = menu_bounds.last().unwrap();

    if last_pb.contains(position){
        // cursor is in the parent part
        *last_index = None;
        return Captured;
    }
    // cursor is in the children part

    // calc new index
    let menu_pos = last_cb.position();

    let height_diff = (position.y - menu_pos.y)
        .clamp(0.0, last_cb.height - 0.001);
    let new_index = (height_diff / menu.item_size.height).floor() as usize;
    *last_index = Some(new_index);
    

    // get new active item
    let active_menu_root = &menu.menu_roots[active_root];
    let item = state.get_flat_indices()
        .fold(active_menu_root, |mt, i|{
            &mt.children[i]
        });

    // check if new item is a menu
    if !item.children.is_empty(){
        state.indices.push(None);
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


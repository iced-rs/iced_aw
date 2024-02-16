//! [`MenuBar`]

use iced_widget::core::{
    alignment, event,
    layout::{Limits, Node},
    mouse, overlay, renderer,
    widget::{tree, Tree},
    Alignment, Clipboard, Color, Element, Event, Layout, Length, Overlay, Padding, Point,
    Rectangle, Shell, Size, Widget,
};

use super::{flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*, types::*};

pub(super) struct MenuBarState {
    pub(super) active_root: Index,
    pub(super) open: bool,
    pub(super) is_pressed: bool,
}
impl Default for MenuBarState {
    fn default() -> Self {
        Self {
            active_root: None,
            open: false,
            is_pressed: false,
        }
    }
}

/// menu bar
pub struct MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    roots: Vec<Item<'a, Message, Theme, Renderer>>,
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
}
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a [`MenuBar`] with the given root items.
    pub fn new(mut roots: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        roots.iter_mut().for_each(|i|{
            if let Some(m) = i.menu.as_mut(){
                m.axis = Axis::Vertical;
            }
        });
        Self {
            roots,
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    /// Sets the width of the [`MenuBar`].
    pub fn width(mut self, width: impl Into<Length>) -> Self{
        self.width = width.into();
        self
    }

    /// Sets the height of the [`MenuBar`].
    pub fn height(mut self, height: impl Into<Length>) -> Self{
        self.height = height.into();
        self
    }

    /// Sets the spacing of the [`MenuBar`].
    pub fn spacing(mut self, spacing: f32) -> Self{
        self.spacing = spacing;
        self
    }

    /// Sets the padding of the [`MenuBar`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self{
        self.padding = padding.into();
        self
    }

}
impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuBarState>()
    }

    fn state(&self) -> tree::State {
        tree::State::Some(Box::new(MenuBarState::default()))
    }

    /// \[Tree{stateless, \[widget_state, menu_state]}...]
    fn children(&self) -> Vec<Tree> {
        // println!("bar children");
        self.roots
            .iter()
            .map(|item| item.tree())
            .collect::<Vec<_>>()
    }

    /// tree: Tree{bar_state, \[item_tree...]}
    fn diff(&self, tree: &mut Tree) {
        // println!("bar diff");
        tree.diff_children_custom(
            &self.roots,
            |tree, item| item.diff(tree),
            |item| item.tree(),
        )
    }

    /// tree: Tree{bar_state, \[item_tree...]}
    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node {
        // println!("bar layout");
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            alignment::Alignment::Center,
            &self.roots.iter().map(|item| &item.item).collect::<Vec<_>>(),
            &mut tree
                .children
                .iter_mut()
                .map(|tree| &mut tree.children[0])
                .collect::<Vec<_>>(),
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // println!("bar event");
        use event::Status::*;

        let status = self
            .roots
            .iter_mut() // [Item...]
            .zip(tree.children.iter_mut()) // [item_tree...]
            .zip(layout.children()) // [widget_node...]
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
            .fold(Ignored, |acc, x| acc.merge(x));

        let bar = tree.state.downcast_mut::<MenuBarState>();
        let bar_bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(bar_bounds) {
                    bar.is_pressed = true;
                    Captured
                } else {
                    Ignored
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if cursor.is_over(bar_bounds) && bar.is_pressed {
                    bar.open = true;
                    bar.is_pressed = false;
                    for (i, l) in layout.children().enumerate() {
                        if cursor.is_over(l.bounds()) {
                            bar.active_root = Some(i);
                            break;
                        }
                    }
                    Captured
                } else {
                    Ignored
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if bar.open{
                    if cursor.is_over(bar_bounds) {
                        for (i, l) in layout.children().enumerate() {
                            if cursor.is_over(l.bounds()) {
                                bar.active_root = Some(i);
                                break;
                            }
                        }
                    }else{
                        bar.open = false
                    }
                    Captured
                }else{
                    Ignored
                }
            }
            _ => Ignored,
        }
        .merge(status)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        // println!("bar draw");
        self.roots
            .iter() // [Item...]
            .zip(tree.children.iter()) // [item_tree...]
            .zip(layout.children()) // [widget_node...]
            .for_each(|((item, tree), layout)| {
                item.draw(tree, renderer, theme, style, layout, cursor, viewport)
            });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        // println!("bar overlay");
        let state = tree.state.downcast_mut::<MenuBarState>();

        let init_bar_bounds = layout.bounds();
        // let init_parent_bounds = state.active_root
        //     .map(|i| layout.children().nth(i).unwrap().bounds());
        let init_root_bounds = layout.children().map(|l| l.bounds() ).collect();

        if state.open {
            // println!("bar open");
            Some(
                MenuBarOverlay {
                    tree,
                    roots: &mut self.roots,
                    init_bar_bounds,
                    // init_parent_bounds,
                    init_root_bounds,
                }
                .overlay_element(),
            )
        } else {
            // println!("None");
            None
        }
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
    // Renderer::Theme: StyleSheet,
{
    fn from(value: MenuBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

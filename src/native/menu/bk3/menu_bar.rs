//! menu bar

use iced_widget::core::{
    alignment, event, layout::{self, Node, Limits}, mouse, overlay, renderer, touch, widget::{tree, Tree}, 
    Event, 
    Alignment, Clipboard, Color, Element, Layout, Length, Overlay, Padding, Rectangle, Shell, Size, Widget
};

use super::{
    flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*
};

pub(super) struct MenuBarState{
    pub(super) active_root: usize,
    pub(super) open: bool,
    pub(super) viewport: Rectangle,
    pub(super) indices: Vec<usize>,
}
impl Default for MenuBarState{
    fn default() -> Self {
        Self {
            active_root: 0,
            open: false,
            viewport: Rectangle::default(),
            indices: Vec::new(),
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
#[allow(missing_docs)]
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(roots: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            roots,
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
    
}
impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for MenuBar<'a, Message, Theme, Renderer>
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

    /// \[Tree{item_state, \[widget_state, menu_state]}...]
    fn children(&self) -> Vec<Tree> {
        println!("bar children");
        self.roots.iter().map(|item| item.tree()).collect::<Vec<_>>()
    }

    /// tree: Tree{bar_state, \[item_tree...]}
    fn diff(&self, tree: &mut Tree) {
        println!("bar diff");
        tree.diff_children_custom(
            &self.roots, 
            |tree, item| item.diff(tree), 
            |item| item.tree()
        )
    }
    
    /// tree: Tree{bar_state, \[item_tree...]}
    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        println!("bar layout");
        flex::resolve(
            flex::Axis::Horizontal,
            renderer, 
            limits, 
            self.width, 
            self.height, 
            self.padding, 
            self.spacing, 
            alignment::Alignment::Center,
            &self.roots.iter().map(|item| &item.item ).collect::<Vec<_>>(),
            &mut tree.children.iter_mut().map(|tree| &mut tree.children[0]).collect::<Vec<_>>()
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
        println!("bar event");
        use event::Status::*;

        let status = self.roots // [Item...]
            .iter_mut()
            .zip(tree.children.iter_mut()) // [item_tree...]
            .zip(layout.children()) // [widget_layout...]
            .map(|((item, tree), layout)|{
                item.item.as_widget_mut().on_event(
                    &mut tree.children[0], 
                    event.clone(), 
                    layout,
                    cursor, 
                    renderer, 
                    clipboard, 
                    shell, 
                    viewport
                )
            })
            .fold(event::Status::Ignored, |acc, x| acc.merge(x) );
    
    // make sure there's only one item that is open
    let mut use_open = false;
    for (i, t) in tree.children.iter_mut().enumerate(){
        let item_state = t.state.downcast_mut::<ItemState>();
        if use_open == true{
            item_state.open = false;
        }else if item_state.open{
            use_open = true;
            let bar = tree.state.downcast_mut::<MenuBarState>();
            // store the active item index
            bar.active_root = i;
        }
    }

    status

        // let bar_bounds = layout.bounds();

        // match event {
        //     Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        //     | Event::Touch(touch::Event::FingerPressed { .. }) => {

        //         Ignored
        //     }
        //     Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        //     | Event::Touch(touch::Event::FingerLifted { .. })
        //     | Event::Touch(touch::Event::FingerLost { .. }) => {

        //         Ignored
        //     }   
        //     Event::Mouse(mouse::Event::CursorMoved { position:_ }) => {
        //         Ignored
        //     }
        //     _ => Ignored
        // }.merge(status)
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
        println!("bar draw");
        if let Some(viewport) = layout.bounds().intersection(viewport) {
            for ((root, tree), layout) in self
                .roots
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
            {
                root.item.as_widget().draw(
                    &tree.children[0], renderer, theme, style, layout, cursor, &viewport,
                );
            }
        }
    }
    
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        println!("bar overlay");
        let state = tree.state.downcast_mut::<MenuBarState>();
        let bar_bounds = layout.bounds();
        let parent_bounds = layout.children().nth(state.active_root).unwrap().bounds();

        if state.open{
            Some(MenuBarOverlay{
                tree,
                roots: &mut self.roots,
                parent_bounds,
                bar_bounds,
            }.overlay_element())
        }else {
            None
        }
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
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

use iced_widget::core::{
    alignment, event, layout, mouse, overlay, renderer, touch, widget::{tree, Tree}, Alignment, Clipboard, Color, Element, Layout, Length, Overlay, Padding, Rectangle, Shell, Size, Widget
};

use super::{
    flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*
};

pub(super) struct MenuBarState{
    pub(super) active_root: usize,
    pub(super) open: bool,
}
impl Default for MenuBarState{
    fn default() -> Self {
        Self {
            active_root: 0,
            open: false,
        }
    }
}

pub struct MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    roots: Vec<MenuTree<'a, Message, Theme, Renderer>>,
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
}
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(roots: Vec<MenuTree<'a, Message, Theme, Renderer>>) -> Self {
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

    fn children(&self) -> Vec<Tree> {
        self.roots.iter().map(|mt| mt.tree()).collect::<Vec<_>>()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(
            &self.roots, 
            |tree, mt| diff(tree, mt), 
            |mt| mt.tree()
        )
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            alignment::Alignment::Center,
            &self.roots.iter().map(|mt| &mt.parent).collect::<Vec<_>>(),
            &mut tree.children.iter().map(|tree| &mut tree.children[0]).collect::<Vec<_>>(),
        )
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        
        event::Status::Ignored
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
        if let Some(viewport) = layout.bounds().intersection(viewport) {
            for ((root, state), layout) in self
                .roots
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
            {
                root.parent.as_widget().draw(
                    state, renderer, theme, style, layout, cursor, &viewport,
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
        let state = tree.state.downcast_mut::<MenuBarState>();
        let parent_bounds = layout.children().nth(state.active_root).unwrap().bounds();

        if state.open{
            Some(MenuBarOverlay{
                tree,
                active_tree: &mut tree.children[state.active_root],
                active_root: &mut self.roots[state.active_root],
                parent_bounds,
            }.overlay_element())
        }else {
            None
        }
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    // Renderer::Theme: StyleSheet,
{
    fn from(value: MenuBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

//! doc
//!
use iced_widget::core::{
    event, layout::{self, Node}, mouse, overlay, renderer, touch, widget::tree::{self, Tree}, 
    Alignment, Border, Clipboard, Color, Element, Event, Length, 
    Padding, Point, Vector, Rectangle, Shell, Size, Widget,
};
use super::types::*;
use super::menu_tree_overlay::MenuTreeOverlay;

pub(super) struct MenuTreeState{
    pub(super) open: bool,
    pub(super) scroll_offset: f32,
}
impl Default for MenuTreeState{
    fn default() -> Self {
        Self { open: false, scroll_offset: 0.0 }
    }
}

/// doc
#[allow(clippy::missing_docs_in_private_parents)]
pub struct MenuTree<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) parent: Element<'a, Message, Theme, Renderer>,
    pub(super) children: Vec<MenuTree<'a, Message, Theme, Renderer>>,
    pub(super) spacing: f32,
    pub(super) padding: Padding,
    pub(super) max_width: f32,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) axis: Axis,
    pub(super) offset: f32,
    pub(super) open_condition: OpenCondition,
}
impl<'a, Message, Theme, Renderer> MenuTree<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    #[allow(missing_docs)]
    pub fn new(
        parent: impl Into<Element<'a, Message, Theme, Renderer>>
    ) -> Self{
        Self{
            parent: parent.into(),
            children: Vec::new(),
            spacing: 4.0,
            padding: [0.0;4].into(),
            max_width: f32::MAX,
            width: Length::Shrink,
            height: Length::Shrink,
            axis: Axis::Vertical,
            offset: 0.0,
            open_condition: OpenCondition::Hover,
        }
    }

    #[allow(missing_docs)]
    pub fn with_children(
        parent: impl Into<Element<'a, Message, Theme, Renderer>>, 
        children: Vec<impl Into<MenuTree<'a, Message, Theme, Renderer>>>
    ) -> Self{
        Self{
            parent: parent.into(),
            children: children.into_iter().map(Into::into).collect(),
            spacing: 4.0,
            padding: [0.0;4].into(),
            max_width: f32::MAX,
            width: Length::Shrink,
            height: Length::Shrink,
            axis: Axis::Vertical,
            offset: 0.0,
            open_condition: OpenCondition::Hover,
        }
    }

    pub fn tree(&self) -> Tree{
        Tree{
            tag: self.tag(),
            state: self.state(),
            children: self.children()
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

    /// Sets the [`Padding`] of the [`MenuTree`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`MenuTree`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`MenuTree`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the axis of the [`MenuTree`].
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis.into();
        self
    }

    /// Sets the open condition of the [`MenuTree`].
    pub fn open_condition(mut self, open_condition: OpenCondition) -> Self {
        self.open_condition = open_condition;
        self
    }
}
impl <'a, Message, Theme, Renderer> 
    // Widget<Message, Theme, Renderer> for 
    MenuTree<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(super) fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    pub(super) fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuTreeState>()
    }
    
    pub(super) fn state(&self) -> tree::State {
        tree::State::new(MenuTreeState::default())
    }

    pub(super) fn children(&self) -> Vec<Tree> {
        [
            Tree::new(&self.parent),
            Tree{
                children: self.children.iter().map(|mt| mt.tree()).collect::<Vec<_>>(),
                ..Tree::empty()
            }
        ].into()
    }

    /*
    Tree{
        tag: MTS
        state: MTS
        children: [
            Tree{parent},
            Tree{tag:none, state:none, children:[
                Tree{tag:MTS, state:MTS, children:[..]},
                Tree{tag:MTS, state:MTS, children:[..]},
                Tree{tag:MTS, state:MTS, children:[..]},
                ...
            ]}
        ]
    }
    */
    pub(super) fn diff(&self, tree: &mut Tree) {
        let parent = &mut tree.children[0];
        let children = &mut tree.children[1];

        parent.diff(&self.parent);
        children.diff_children_custom(
            &self.children, 
            |tree, mt| diff(tree, mt), 
            |mt| mt.tree()
        )
    }

    pub(super) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // println!("mt layout");
        self.parent.as_widget().layout(&mut tree.children[0], renderer, limits)
    }

    pub(super) fn on_event(
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
        // println!("mt event");
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

        let state = tree.state.downcast_mut::<MenuTreeState>();
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
        // println!("mt draw");
        self.parent.as_widget().draw(&tree.children[0], renderer, theme, style, layout, cursor, viewport)
    }

    // pub(super) fn overlay<'b>(
    //     &'b mut self,
    //     extra_input:f32,
    //     tree: &'b mut Tree,
    //     layout: layout::Layout<'_>,
    //     renderer: &Renderer,
    // ) -> Option<MenuOverlayElement<'a, 'b, Message, Theme, Renderer>> {
    // // ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
    // // ) -> Option<MenuTreeOverlay<'a, 'b, Message, Theme, Renderer>> {
    //     // println!("mt overlay");
    //     let mut og = overlay::Group::new();
    //     let Tree { tag, state, children } = tree;
        
    //     // let [parent_tree, children_tree] = children.as_mut_slice() else { panic!("Tree Error") };
    //     let parent_tree = &mut children[0];
    //     let children_tree = &mut children[1];
        
    //     if let Some(c) = self.parent
    //         .as_widget_mut()
    //         .overlay(parent_tree, layout, renderer)
    //     {
    //         og = og.push(c);
    //     }

    //     let ms = state.downcast_mut::<MenuTreeState>();
    //     if !ms.open {
    //         Some(og.overlay().into())
    //     }else{
    //         // Some(og.push(
    //         //     MenuTreeOverlay{
    //         //         state: ms,
    //         //         tree: children_tree,
    //         //         children: &mut self.children,
    //         //         parent_bounds: layout.bounds(),
    //         //         max_width: 1000.0,
    //         //         spacing: self.spacing,
    //         //         padding: self.padding,
    //         //         width: self.width,
    //         //         height: self.height,
    //         //         axis: self.axis,
    //         //         offset: self.offset,
    //         //     }.overlay()
    //         // ).overlay())
    //         None
    //     }
    // }
}

pub fn diff<'a, Message, Theme, Renderer>(
    tree: &mut Tree,
    new: &MenuTree<'a, Message, Theme, Renderer>
) where
    Renderer: renderer::Renderer,
{
    if tree.tag == new.tag() {
        new.diff(tree);
    } else {
        *tree = new.tree()
    }
}
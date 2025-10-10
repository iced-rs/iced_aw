//! A container widget that allows you to specify the layout of its children.

use iced_core::{Length, Rectangle, Widget};

#[allow(unused_imports)]
pub use iced_core::{
    layout::{Limits, Node},
    widget::Tree,
    Renderer,
};

type LayoutFn<'a, Message, Theme, Renderer> = Box<
    dyn Fn(
        &mut Vec<iced_core::Element<'a, Message, Theme, Renderer>>,
        &mut Vec<Tree>,
        &Renderer,
        &Limits,
    ) -> Node,
>;

/// A container widget that allows you to specify the layout of its children.
pub struct CustomLayout<'a, Message, Theme, Renderer> {
    elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>>,
    width: Length,
    height: Length,
    layout: LayoutFn<'a, Message, Theme, Renderer>,
}

impl<'b, Message, Theme, Renderer: iced_core::Renderer> CustomLayout<'b, Message, Theme, Renderer> {
    /// Creates a new [`CustomLayout`]
    pub fn new(
        elements: Vec<iced_core::Element<'b, Message, Theme, Renderer>>,
        layout: impl Fn(
                &mut Vec<iced_core::Element<'b, Message, Theme, Renderer>>,
                &mut Vec<Tree>,
                &Renderer,
                &Limits,
            ) -> Node
            + 'static,
    ) -> Self {
        Self {
            elements,
            width: Length::Shrink,
            height: Length::Shrink,
            layout: Box::new(layout),
        }
    }

    /// Sets the width of the [`CustomLayout`]
    #[must_use]
    pub fn width(mut self, length: impl Into<Length>) -> Self {
        self.width = length.into();
        self
    }

    /// Sets the height of the [`CustomLayout`]
    #[must_use]
    pub fn height(mut self, length: impl Into<Length>) -> Self {
        self.height = length.into();
        self
    }
}

impl<Message, Theme, Renderer: iced_core::Renderer> Widget<Message, Theme, Renderer>
    for CustomLayout<'_, Message, Theme, Renderer>
{
    fn size(&self) -> iced_core::Size<Length> {
        iced_core::Size::new(self.width, self.height)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        (self.layout)(&mut self.elements, &mut tree.children, renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        tree.children
            .iter()
            .zip(layout.children())
            .zip(self.elements.iter())
            .for_each(|((state, layout), element)| {
                element
                    .as_widget()
                    .draw(state, renderer, theme, style, layout, cursor, viewport);
            });
    }

    fn children(&self) -> Vec<Tree> {
        self.elements.iter().map(|x| Tree::new(x)).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements);
    }

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation,
    ) {
        state
            .children
            .iter_mut()
            .zip(layout.children())
            .zip(self.elements.iter_mut())
            .for_each(|((state, layout), element)| {
                operation.container(None, layout.bounds());
                operation.traverse(&mut |operation| {
                    element.as_widget_mut().operate(state, layout, renderer, operation);
                });
            });
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) {
        for ((state, layout), element) in state
            .children
            .iter_mut()
            .zip(layout.children())
            .zip(self.elements.iter_mut())
        {
            element.as_widget_mut().update(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }
    }

    fn size_hint(&self) -> iced_core::Size<Length> {
        self.size()
    }

    fn overlay<'a>(
        &'a mut self,
        state: &'a mut Tree,
        layout: iced_core::Layout<'a>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced_core::Vector,
    ) -> Option<iced_core::overlay::Element<'a, Message, Theme, Renderer>> {
        iced_core::overlay::from_children(
            &mut self.elements,
            state,
            layout,
            renderer,
            viewport,
            translation,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.elements
            .iter()
            .zip(&state.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }
}

impl<'b, Message: 'b, Theme: 'b, Renderer: iced_core::Renderer + 'b>
    From<CustomLayout<'b, Message, Theme, Renderer>>
    for iced_core::Element<'b, Message, Theme, Renderer>
{
    fn from(value: CustomLayout<'b, Message, Theme, Renderer>) -> Self {
        iced_core::Element::new(value)
    }
}

//! A container widget that allows you to specify the layout of its children.

use iced::{advanced::Widget, Rectangle};

#[allow(unused_imports)]
pub use iced::advanced::{
    layout::{Limits, Node},
    widget::Tree,
    Renderer,
};

type LayoutFn<'a, Message, Theme, Renderer> = Box<
    dyn Fn(
        &mut Vec<iced::Element<'a, Message, Theme, Renderer>>,
        &mut Vec<Tree>,
        &Renderer,
        &Limits,
    ) -> Node,
>;

/// A container widget that allows you to specify the layout of its children.
pub struct CustomLayout<'a, Message, Theme, Renderer> {
    elements: Vec<iced::Element<'a, Message, Theme, Renderer>>,
    width: iced::Length,
    height: iced::Length,
    layout: LayoutFn<'a, Message, Theme, Renderer>,
}

impl<'b, Message, Theme, Renderer: iced::advanced::Renderer>
    CustomLayout<'b, Message, Theme, Renderer>
{
    /// Creates a new [`CustomLayout`]
    pub fn new(
        elements: Vec<iced::Element<'b, Message, Theme, Renderer>>,
        layout: impl Fn(
                &mut Vec<iced::Element<'b, Message, Theme, Renderer>>,
                &mut Vec<Tree>,
                &Renderer,
                &Limits,
            ) -> Node
            + 'static,
    ) -> Self {
        Self {
            elements,
            width: iced::Shrink,
            height: iced::Shrink,
            layout: Box::new(layout),
        }
    }

    /// Sets the width of the [`CustomLayout`]
    #[must_use]
    pub fn width(mut self, length: impl Into<iced::Length>) -> Self {
        self.width = length.into();
        self
    }

    /// Sets the height of the [`CustomLayout`]
    #[must_use]
    pub fn height(mut self, length: impl Into<iced::Length>) -> Self {
        self.height = length.into();
        self
    }
}

impl<Message, Theme, Renderer: iced::advanced::Renderer> Widget<Message, Theme, Renderer>
    for CustomLayout<'_, Message, Theme, Renderer>
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size::new(self.width, self.height)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        (self.layout)(&mut self.elements, &mut tree.children, renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
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

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.elements);
    }

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        state
            .children
            .iter_mut()
            .zip(layout.children())
            .zip(self.elements.iter_mut())
            .for_each(|((state, layout), element)| {
                operation.container(None, layout.bounds(), &mut |operation| {
                    element
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
                });
            });
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
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

    fn size_hint(&self) -> iced::Size<iced::Length> {
        self.size()
    }

    fn overlay<'a>(
        &'a mut self,
        state: &'a mut Tree,
        layout: iced::advanced::Layout<'a>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'a, Message, Theme, Renderer>> {
        iced::advanced::overlay::from_children(
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
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
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

impl<'b, Message: 'b, Theme: 'b, Renderer: iced::advanced::Renderer + 'b>
    From<CustomLayout<'b, Message, Theme, Renderer>>
    for iced::Element<'b, Message, Theme, Renderer>
{
    fn from(value: CustomLayout<'b, Message, Theme, Renderer>) -> Self {
        iced::Element::new(value)
    }
}

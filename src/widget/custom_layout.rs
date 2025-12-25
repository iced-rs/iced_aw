//! A container widget that allows you to specify the layout of its children.

use iced_core::{Length, Rectangle, Widget};

#[allow(unused_imports)]
pub use iced_core::{
    Renderer,
    layout::{Limits, Node},
    widget::Tree,
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
                    element
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Size, layout::Node};

    #[derive(Clone)]
    enum TestMessage {}

    type TestCustomLayout<'a> =
        CustomLayout<'a, TestMessage, iced_widget::Theme, iced_widget::Renderer>;

    fn simple_layout_fn(
        _elements: &mut Vec<
            iced_core::Element<'_, TestMessage, iced_widget::Theme, iced_widget::Renderer>,
        >,
        _trees: &mut Vec<Tree>,
        _renderer: &iced_widget::Renderer,
        limits: &Limits,
    ) -> Node {
        Node::new(limits.resolve(Length::Shrink, Length::Shrink, Size::ZERO))
    }

    #[test]
    fn custom_layout_new_has_default_values() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn);

        assert_eq!(layout.width, Length::Shrink);
        assert_eq!(layout.height, Length::Shrink);
        assert_eq!(layout.elements.len(), 0);
    }

    #[test]
    fn custom_layout_width_sets_value() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn).width(200);
        assert_eq!(layout.width, Length::Fixed(200.0));
    }

    #[test]
    fn custom_layout_width_fill_sets_value() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn).width(Length::Fill);
        assert_eq!(layout.width, Length::Fill);
    }

    #[test]
    fn custom_layout_height_sets_value() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn).height(100);
        assert_eq!(layout.height, Length::Fixed(100.0));
    }

    #[test]
    fn custom_layout_height_shrink_sets_value() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn).height(Length::Shrink);
        assert_eq!(layout.height, Length::Shrink);
    }

    #[test]
    fn custom_layout_size_method_returns_correct_size() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn)
            .width(150)
            .height(75);

        let size = Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&layout);
        assert_eq!(size.width, Length::Fixed(150.0));
        assert_eq!(size.height, Length::Fixed(75.0));
    }

    #[test]
    fn custom_layout_chaining_methods() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn)
            .width(200)
            .height(100);

        assert_eq!(layout.width, Length::Fixed(200.0));
        assert_eq!(layout.height, Length::Fixed(100.0));
    }

    #[test]
    fn custom_layout_with_elements() {
        let elements = vec![
            iced_widget::text::Text::new("Element 1").into(),
            iced_widget::text::Text::new("Element 2").into(),
        ];
        let layout = TestCustomLayout::new(elements, simple_layout_fn);

        assert_eq!(layout.elements.len(), 2);
    }

    #[test]
    fn custom_layout_children_count_matches_elements() {
        let elements = vec![
            iced_widget::text::Text::new("Element 1").into(),
            iced_widget::text::Text::new("Element 2").into(),
            iced_widget::text::Text::new("Element 3").into(),
        ];
        let layout = TestCustomLayout::new(elements, simple_layout_fn);

        let children =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::children(&layout);
        assert_eq!(children.len(), 3);
    }

    #[test]
    fn custom_layout_empty_elements() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn);

        let children =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::children(&layout);
        assert_eq!(children.len(), 0);
    }

    #[test]
    fn custom_layout_length_fillportion() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn)
            .width(Length::FillPortion(2))
            .height(Length::FillPortion(3));

        assert_eq!(layout.width, Length::FillPortion(2));
        assert_eq!(layout.height, Length::FillPortion(3));
    }

    #[test]
    fn custom_layout_size_hint_equals_size() {
        let layout = TestCustomLayout::new(vec![], simple_layout_fn)
            .width(100)
            .height(50);

        let size = Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&layout);
        let size_hint =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size_hint(&layout);

        assert_eq!(size.width, size_hint.width);
        assert_eq!(size.height, size_hint.height);
    }
}

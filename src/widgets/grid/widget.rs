use iced::{
    advanced::{
        layout::{Limits, Node},
        overlay::Group,
        renderer,
        renderer::Style,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event, mouse, overlay, Element, Event, Length, Rectangle, Size, Vector,
};

use super::{layout::layout, types::Grid};

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Grid<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        if self.element_count() == 0 {
            return Node::new(Size::ZERO);
        }

        assert!(
            !self.column_widths.is_empty(),
            "At least one column width is required"
        );
        assert!(
            !self.row_heights.is_empty(),
            "At least one row height is required"
        );

        layout(
            tree,
            renderer,
            limits,
            self.column_count(),
            self.row_count(),
            self.element_count(),
            &self.rows,
            self.column_spacing,
            self.row_spacing,
            self.padding,
            self.horizontal_alignment,
            self.vertical_alignment,
            self.width,
            self.height,
            &self.column_widths,
            &self.row_heights,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((element, state), layout) in self
            .elements_iter()
            .zip(&state.children)
            .zip(layout.children())
        {
            element
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.elements_iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements_iter().collect::<Vec<_>>());
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        for ((element, state), layout) in self
            .elements_iter()
            .zip(&mut state.children)
            .zip(layout.children())
        {
            element
                .as_widget()
                .operate(state, layout, renderer, operation);
        }
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let children_status = self
            .elements_iter_mut()
            .zip(&mut state.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            });

        children_status.fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.elements_iter()
            .zip(&state.children)
            .zip(layout.children())
            .map(|((e, state), layout)| {
                e.as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .fold(mouse::Interaction::default(), |interaction, next| {
                interaction.max(next)
            })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let children = self
            .elements_iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .filter_map(|((child, state), layout)| {
                child
                    .as_widget_mut()
                    .overlay(state, layout, renderer, translation)
            })
            .collect::<Vec<_>>();

        (!children.is_empty()).then(|| Group::with_children(children).overlay())
    }
}

impl<'a, Message, Theme, Renderer> From<Grid<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Message: 'static,
    Theme: 'a,
{
    fn from(grid: Grid<'a, Message, Theme, Renderer>) -> Self {
        Element::new(grid)
    }
}

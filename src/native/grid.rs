//! Use a grid as an input element for creating grids.
//!
//! *This API requires the following crate features to be activated: `grid`*
use iced_native::{
    event, layout::Node, mouse, Clipboard, Event, Layout, Length, Point, Rectangle, Shell, Size,
};
use iced_native::{overlay, widget::Tree, Element, Widget};

/// A container that distributes its contents in a grid.
///
/// # Example
///
/// ```
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::Text;
/// #
/// # pub type Grid<'a, Message> = iced_aw::Grid<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
/// }
///
/// let grid = Grid::<Message>::with_columns(2)
///     .push(Text::new("First row, first column"))
///     .push(Text::new("First row, second column"))
///     .push(Text::new("Second row, first column"))
///     .push(Text::new("Second row, second column"));
///
/// ```
#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message, Renderer> {
    /// The distribution [`Strategy`](Strategy) of the [`Grid`](Grid).
    strategy: Strategy,
    /// The elements in the [`Grid`](Grid).
    elements: Vec<Element<'a, Message, Renderer>>,
}

/// The [`Strategy`](Strategy) of how to distribute the columns of the [`Grid`](Grid).
enum Strategy {
    /// Use `n` columns.
    Columns(usize),
    /// Try to fit as much columns that have a fixed width.
    ColumnWidth(u16),
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Columns(1)
    }
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    /// Creates a new empty [`Grid`](Grid).
    /// Elements will be laid out in a specific amount of columns.
    #[must_use]
    pub fn with_columns(columns: usize) -> Self {
        Self {
            strategy: Strategy::Columns(columns),
            elements: Vec::new(),
        }
    }

    /// Creates a new empty [`Grid`](Grid).
    /// Columns will be generated to fill the given space.
    #[must_use]
    pub fn with_column_width(column_width: u16) -> Self {
        Self {
            strategy: Strategy::ColumnWidth(column_width),
            elements: Vec::new(),
        }
    }

    /// Adds an [`Element`](Element) to the [`Grid`](Grid).
    #[must_use]
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }

    /// Inserts an [`Element`](Element) into the [`Grid`](Grid).
    pub fn insert<E>(&mut self, element: E)
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Grid<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.elements.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements);
    }

    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        if self.elements.is_empty() {
            return Node::new(Size::ZERO);
        }

        match self.strategy {
            // find out how wide a column is by finding the widest cell in it
            Strategy::Columns(columns) => {
                if columns == 0 {
                    return Node::new(Size::ZERO);
                }

                let mut layouts = Vec::with_capacity(self.elements.len());
                let mut column_widths = Vec::<f32>::with_capacity(columns);

                for (column, element) in (0..columns).cycle().zip(&self.elements) {
                    let layout = element.as_widget().layout(renderer, limits);
                    #[allow(clippy::option_if_let_else)]
                    match column_widths.get_mut(column) {
                        Some(column_width) => *column_width = column_width.max(layout.size().width),
                        None => column_widths.insert(column, layout.size().width),
                    }

                    layouts.push(layout);
                }

                let column_aligns =
                    std::iter::once(&0.)
                        .chain(column_widths.iter())
                        .scan(0., |state, width| {
                            *state += width;
                            Some(*state)
                        });
                let grid_width = column_widths.iter().sum();

                build_grid(columns, column_aligns, layouts.into_iter(), grid_width)
            }
            // find number of columns by checking how many can fit
            Strategy::ColumnWidth(column_width) => {
                let column_limits = limits.width(Length::Units(column_width));
                let column_width: f32 = column_width.into();
                let max_width = limits.max().width;
                let columns = (max_width / column_width).floor() as usize;

                let layouts = self
                    .elements
                    .iter()
                    .map(|element| element.as_widget().layout(renderer, &column_limits));
                let column_aligns =
                    std::iter::successors(Some(0.), |width| Some(width + column_width));
                #[allow(clippy::cast_precision_loss)] // TODO: possible precision loss
                let grid_width = (columns as f32) * column_width;

                build_grid(columns, column_aligns, layouts, grid_width)
            }
        }
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let children_status = self
            .elements
            .iter_mut()
            .zip(&mut state.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                )
            });

        children_status.fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.elements
            .iter()
            .zip(&state.children)
            .zip(layout.children())
            .map(|((e, state), layout)| {
                e.as_widget()
                    .mouse_interaction(state, layout, cursor_position, viewport, renderer)
            })
            .fold(mouse::Interaction::default(), |interaction, next| {
                interaction.max(next)
            })
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        for ((element, state), layout) in self
            .elements
            .iter()
            .zip(&state.children)
            .zip(layout.children())
        {
            element.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        overlay::from_children(&self.elements, tree, layout, renderer)
    }
}

/// Builds the layout of the [`Grid`](grid).
fn build_grid(
    columns: usize,
    column_aligns: impl Iterator<Item = f32> + Clone,
    layouts: impl Iterator<Item = Node> + ExactSizeIterator,
    grid_width: f32,
) -> Node {
    let mut nodes = Vec::with_capacity(layouts.len());
    let mut grid_height = 0.;
    let mut row_height = 0.;

    for ((column, column_align), mut node) in (0..columns).zip(column_aligns).cycle().zip(layouts) {
        if column == 0 {
            grid_height += row_height;
            row_height = 0.;
        }

        node.move_to(Point::new(column_align, grid_height));
        row_height = row_height.max(node.size().height);
        nodes.push(node);
    }

    grid_height += row_height;

    Node::with_children(Size::new(grid_width, grid_height), nodes)
}

impl<'a, Message, Renderer> From<Grid<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + 'a,
    Message: 'static,
{
    fn from(grid: Grid<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(grid)
    }
}

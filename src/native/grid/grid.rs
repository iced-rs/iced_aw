//! A container to layout widgets in a grid.

use iced_widget::core::{
    alignment::{Horizontal, Vertical},
    event,
    layout::{Limits, Node},
    mouse, overlay,
    overlay::Group,
    renderer::Style,
    widget::{Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Pixels, Point, Rectangle, Shell, Size, Widget,
};

use crate::grid::row::GridRow;

/// A container that distributes its contents in a grid of rows and columns.
///
/// The number of columns is determined by the row with the most elements.
#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message, Renderer = crate::Renderer> {
    rows: Vec<GridRow<'a, Message, Renderer>>,
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    row_height_strategy: Strategy,
    columng_width_stratgey: Strategy,
    row_spacing: Pixels,
    column_spacing: Pixels,
}

impl<'a, Message, Renderer> Default for Grid<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            row_height_strategy: Strategy::Minimum,
            columng_width_stratgey: Strategy::Minimum,
            row_spacing: 1.0.into(),
            column_spacing: 1.0.into(),
        }
    }
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    /// Creates a new [`Grid`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`Grid`] with the given [`GridRow`]s.
    pub fn with_rows(rows: Vec<GridRow<'a, Message, Renderer>>) -> Self {
        Self {
            rows,
            ..Default::default()
        }
    }

    /// Adds a [`GridRow`] to the [`Grid`].
    pub fn push(mut self, row: GridRow<'a, Message, Renderer>) -> Self {
        self.rows.push(row);
        self
    }

    /// Sets the horizontal alignment of the widgets within their cells. Default:
    /// [`Horizontal::Left`]
    pub fn horizontal_alignment(mut self, align: Horizontal) -> Self {
        self.horizontal_alignment = align;
        self
    }

    /// Sets the vertical alignment of the widgets within their cells. Default:
    /// [`Vertical::Center`]
    pub fn vertical_alignment(mut self, align: Vertical) -> Self {
        self.vertical_alignment = align;
        self
    }

    /// Sets the [`Strategy`] used to determine the height of the rows.
    pub fn row_height_strategy(mut self, strategy: Strategy) -> Self {
        self.row_height_strategy = strategy;
        self
    }

    /// Sets the [`Strategy`] used to determine the width of the columns.
    pub fn column_width_strategy(mut self, strategy: Strategy) -> Self {
        self.columng_width_stratgey = strategy;
        self
    }

    /// Sets the spacing between the rows and columns.
    // pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
    pub fn spacing(mut self, spacing: f32) -> Self {
        let spacing: Pixels = spacing.into();
        self.row_spacing = spacing;
        self.column_spacing = spacing;
        self
    }

    /// Sets the spacing between the rows.
    pub fn row_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.row_spacing = spacing.into();
        self
    }

    /// Sets the spacing between the columns.
    pub fn column_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.column_spacing = spacing.into();
        self
    }

    fn elements_iter(&self) -> impl Iterator<Item = &Element<'a, Message, Renderer>> {
        self.rows.iter().flat_map(|row| row.elements.iter())
    }

    fn elements_iter_mut(&mut self) -> impl Iterator<Item = &mut Element<'a, Message, Renderer>> {
        self.rows.iter_mut().flat_map(|row| row.elements.iter_mut())
    }

    fn column_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.elements.len())
            .max()
            .unwrap_or(0)
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn element_count(&self) -> usize {
        self.rows.iter().map(|row| row.elements.len()).sum()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Grid<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        if self.element_count() == 0 {
            return Node::new(Size::ZERO);
        }

        // Calculate the column widths and row heights to fit the contents
        let mut min_columns_widths = Vec::<f32>::with_capacity(self.column_count());
        let mut min_row_heights = Vec::<f32>::with_capacity(self.row_count());
        let mut max_row_height = 0.0f32;
        let mut max_column_width = 0.0f32;
        for row in self.rows.iter() {
            let mut row_height = 0.0f32;

            for (col_idx, element) in row.elements.iter().enumerate() {
                let layout = element.as_widget().layout(renderer, &limits);
                let Size { width, height } = layout.size();

                if let Some(column_width) = min_columns_widths.get_mut(col_idx) {
                    *column_width = column_width.max(width);
                } else {
                    min_columns_widths.insert(col_idx, width);
                }

                row_height = row_height.max(height);
                max_column_width = max_column_width.max(width);
            }
            min_row_heights.push(row_height);
            max_row_height = max_row_height.max(row_height);
        }

        // Create the grid layout
        let mut x = 0.0;
        let mut y = 0.0;
        let mut nodes = Vec::with_capacity(self.element_count());
        for (row_idx, row) in self.rows.iter().enumerate() {
            x = 0.0;
            let row_height = match self.row_height_strategy {
                Strategy::Minimum => min_row_heights[row_idx],
                Strategy::Equal => max_row_height,
            };
            for (col_idx, element) in row.elements.iter().enumerate() {
                let col_width = match self.columng_width_stratgey {
                    Strategy::Minimum => min_columns_widths[col_idx],
                    Strategy::Equal => max_column_width,
                };
                let cell_size = Size::new(col_width, row_height);

                let mut node = element.as_widget().layout(renderer, &limits);
                node.move_to(Point::new(x, y));
                node.align(
                    self.horizontal_alignment.into(),
                    self.vertical_alignment.into(),
                    cell_size,
                );
                nodes.push(node);
                x += col_width;
                if col_idx < row.elements.len() - 1 {
                    x += self.column_spacing.0;
                }
            }
            y += row_height;
            if row_idx < self.rows.len() - 1 {
                y += self.row_spacing.0;
            }
        }

        let grid_size = Size::new(x, y);

        let grid = Node::with_children(grid_size, nodes);
        grid
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_widget::core::Renderer>::Theme,
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
        tree.diff_children(&self.elements_iter().collect::<Vec<_>>())
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
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
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let children = self
            .elements_iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .filter_map(|((child, state), layout)| {
                child.as_widget_mut().overlay(state, layout, renderer)
            })
            .collect::<Vec<_>>();

        (!children.is_empty()).then(|| Group::with_children(children).overlay())
    }
}

impl<'a, Message, Renderer> From<Grid<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer + 'a,
    Message: 'static,
{
    fn from(grid: Grid<'a, Message, Renderer>) -> Self {
        Element::new(grid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Strategy used for determining the widths and height of columns and rows.
pub enum Strategy {
    /// Each row (column) has the height (width) needed to fit its contents.
    Minimum,

    /// All rows (columns) have the same height (width). The height (width) is determined by the
    /// row (column) with the talest (widest) contents.
    Equal,
}

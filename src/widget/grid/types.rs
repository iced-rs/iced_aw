use iced::{
    Element, Length, Padding, Pixels,
    advanced::renderer,
    alignment::{Horizontal, Vertical},
};

/// A container that distributes its contents in a grid of rows and columns.
///
/// The number of columns is determined by the row with the most elements.
#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    pub(super) rows: Vec<GridRow<'a, Message, Theme, Renderer>>,
    pub(super) horizontal_alignment: Horizontal,
    pub(super) vertical_alignment: Vertical,
    pub(super) column_spacing: Pixels,
    pub(super) row_spacing: Pixels,
    pub(super) padding: Padding,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) column_widths: Vec<Length>,
    pub(super) row_heights: Vec<Length>,
}

impl<Message, Theme, Renderer> Default for Grid<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            column_spacing: 1.0.into(),
            row_spacing: 1.0.into(),
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            column_widths: vec![Length::Fill],
            row_heights: vec![Length::Fill],
        }
    }
}

impl<'a, Message, Theme, Renderer> Grid<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a new [`Grid`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`Grid`] with the given [`GridRow`]s.
    #[must_use]
    pub fn with_rows(rows: Vec<GridRow<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            rows,
            ..Default::default()
        }
    }

    /// Adds a [`GridRow`] to the [`Grid`].
    #[must_use]
    pub fn push(mut self, row: GridRow<'a, Message, Theme, Renderer>) -> Self {
        self.rows.push(row);
        self
    }

    /// Sets the horizontal alignment of the widget within their cells. Default:
    /// [`Horizontal::Left`]
    #[must_use]
    pub fn horizontal_alignment(mut self, align: Horizontal) -> Self {
        self.horizontal_alignment = align;
        self
    }

    /// Sets the vertical alignment of the widget within their cells. Default:
    /// [`Vertical::Center`]
    #[must_use]
    pub fn vertical_alignment(mut self, align: Vertical) -> Self {
        self.vertical_alignment = align;
        self
    }

    /// Sets the spacing between rows and columns. To set row and column spacing separately, use
    /// [`Self::column_spacing()`] and [`Self::row_spacing()`].
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        let spacing: Pixels = spacing.into();
        self.row_spacing = spacing;
        self.column_spacing = spacing;
        self
    }

    /// Sets the spacing between columns.
    #[must_use]
    pub fn column_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.column_spacing = spacing.into();
        self
    }

    /// Sets the spacing between rows.
    #[must_use]
    pub fn row_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.row_spacing = spacing.into();
        self
    }

    /// Sets the padding around the grid.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the grid width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the grid height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the column width.
    ///
    /// The same setting will be used for all columns. To set separate values for each column, use
    /// [`Self::column_widths()`]. Columns are never smaller than the space needed to fit their
    /// contents.
    #[must_use]
    pub fn column_width(mut self, width: impl Into<Length>) -> Self {
        self.column_widths = vec![width.into()];
        self
    }

    /// Sets the row height.
    ///
    /// The same setting will be used for all rows. To set separate values for each row, use
    /// [`Self::row_heights()`]. Rows are never smaller than the space needed to fit their
    /// contents.
    #[must_use]
    pub fn row_height(mut self, height: impl Into<Length>) -> Self {
        self.row_heights = vec![height.into()];
        self
    }

    /// Sets a separate width for each column.
    ///
    /// Columns are never smaller than the space needed to fit their contents. When supplying fewer
    /// values than the number of columns, values are are repeated using
    /// [`std::iter::Iterator::cycle()`].
    #[must_use]
    pub fn column_widths(mut self, widths: &[Length]) -> Self {
        self.column_widths = widths.into();
        self
    }

    /// Sets a separate height for each row.
    ///
    /// Rows are never smaller than the space needed to fit their contents. When supplying fewer
    /// values than the number of rows, values are are repeated using
    /// [`std::iter::Iterator::cycle()`].
    #[must_use]
    pub fn row_heights(mut self, heights: &[Length]) -> Self {
        self.row_heights = heights.into();
        self
    }

    pub(super) fn elements_iter(
        &self,
    ) -> impl Iterator<Item = &Element<'a, Message, Theme, Renderer>> {
        self.rows.iter().flat_map(|row| row.elements.iter())
    }

    pub(super) fn elements_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Element<'a, Message, Theme, Renderer>> {
        self.rows.iter_mut().flat_map(|row| row.elements.iter_mut())
    }

    pub(super) fn column_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.elements.len())
            .max()
            .unwrap_or(0)
    }

    pub(super) fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub(super) fn element_count(&self) -> usize {
        self.rows.iter().map(|row| row.elements.len()).sum()
    }
}

/// A container that distributes its contents in a row of a [`crate::Grid`].
#[allow(missing_debug_implementations)]
pub struct GridRow<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    pub(crate) elements: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<Message, Theme, Renderer> Default for GridRow<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl<'a, Message, Theme, Renderer> GridRow<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a new [`GridRow`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`GridRow`] with the given widget.
    #[must_use]
    pub fn with_elements(children: Vec<impl Into<Element<'a, Message, Theme, Renderer>>>) -> Self {
        Self {
            elements: children.into_iter().map(std::convert::Into::into).collect(),
        }
    }

    /// Adds a widget to the [`GridRow`].
    #[must_use]
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }

    /// Applies a transformation to the produced message of all the row's [`Element`].
    pub fn map<B>(self, f: impl Fn(Message) -> B + 'a + Clone) -> GridRow<'a, B, Theme, Renderer>
    where
        Message: 'a,
        Theme: 'a,
        Renderer: renderer::Renderer + 'a,
        B: 'a,
    {
        GridRow {
            elements: self
                .elements
                .into_iter()
                .map(|element| {
                    let f = f.clone();
                    element.map(f)
                })
                .collect(),
        }
    }
}

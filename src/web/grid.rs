//! Use a grid as an input element for creating grids.
//!
//! *This API requires the following crate features to be activated: `grid`*
use dodrio::bumpalo;
use iced_web::{Bus, Css, Element, Widget};

/// A container that distributes its contents in a grid.
///
/// # Example
///
/// ```
/// # use iced_aw::Grid;
/// # use iced_web::Text;
/// let grid = Grid::with_columns(2)
///     .push(Text::new("First row, first column"))
///     .push(Text::new("First row, second column"))
///     .push(Text::new("Second row, first column"))
///     .push(Text::new("Second row, second column"));
///
/// ```
#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message> {
    strategy: Strategy,
    elements: Vec<Element<'a, Message>>,
}

enum Strategy {
    Columns(usize),
    ColumnWidth(u16),
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Columns(1)
    }
}

impl<'a, Message> Grid<'a, Message> {
    /// Creates a new empty [`Grid`](Grid).
    /// Elements will be laid out in a specific amount of columns.
    pub fn with_columns(columns: usize) -> Self {
        Self {
            strategy: Strategy::Columns(columns),
            elements: Vec::new(),
        }
    }

    /// Creates a new empty [`Grid`](Grid).
    /// Columns will be generated to fill the given space.
    pub fn with_column_width(column_width: u16) -> Self {
        Self {
            strategy: Strategy::ColumnWidth(column_width),
            elements: Vec::new(),
        }
    }

    /// Adds an [`Element`](Element) to the [`Grid`](Grid).
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message>>,
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

impl<'a, Message> Widget<Message> for Grid<'a, Message> {
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        publish: &Bus<Message>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::div;

        let mut grid = div(bump);

        match self.strategy {
            Strategy::Columns(columns) => {
                let (class, body) = ("g", "{ display: grid; }");

                grid = grid.attr(
                    "class",
                    bumpalo::format!(in bump, ".{} {}", class, body).into_bump_str(),
                );

                if columns > 0 {
                    let mut style = String::from("grid-template-columns:");

                    for _ in 0..columns {
                        style.push_str(" auto");
                    }

                    grid = grid.attr(
                        "style",
                        bumpalo::format!(in bump, "{}", style).into_bump_str(),
                    );
                }
            }
            Strategy::ColumnWidth(column_width) => {
                let class = "f";

                grid = grid.attr(
                    "class",
                    bumpalo::format!(in bump,
                    ".{0} {{ display: flex; flex-wrap: wrap; }} \
                     .{0} > * {{ width: {1}px!important }}", class, column_width)
                    .into_bump_str(),
                );
            }
        }

        for element in &self.elements {
            grid = grid.child(element.node(bump, publish, style_sheet));
        }

        grid.finish()
    }
}

impl<'a, Message> From<Grid<'a, Message>> for Element<'a, Message>
where
    Message: 'static,
{
    fn from(grid: Grid<'a, Message>) -> Element<'a, Message> {
        Element::new(grid)
    }
}

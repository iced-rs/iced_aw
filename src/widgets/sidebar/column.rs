//! Distribute content rows vertically while setting the row width to widest row.
//! For alignment [`Alignment::Start`] the last element of the row is flushed to the end.
//! For alignment [`Alignment::End`] the first element of the row is flushed to the start.
//!
//! Future: Idea to implement leaders before/after the flushed element for `Start`/`End`
//! alignments.

use iced::{
    advanced::{
        layout::{self, Node},
        mouse, overlay, renderer,
        widget::{tree::Tree, Operation},
        Clipboard, Layout, Shell, Widget,
    },
    alignment,
    event::{self, Event},
    widget::Row,
    Alignment, Element, Length, Padding, Pixels, Point, Rectangle, Size, Vector,
};

/// A container that distributes its contents vertically.
#[allow(missing_debug_implementations)]
pub struct FlushColumn<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    align: Alignment,
    clip: bool,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    flush: bool,
}

impl<'a, Message: 'a, Theme: 'a, Renderer> FlushColumn<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + 'a,
{
    /// Creates an empty [`Column`].
    #[must_use]
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Column`] with the given capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Column`] with the given elements.
    #[must_use]
    pub fn with_children(
        children: impl IntoIterator<Item = Row<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let iterator = children.into_iter();
        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Creates a [`Column`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Column`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Column::width`] or [`Column::height`] accordingly.
    #[must_use]
    pub fn from_vec(children: Vec<Row<'a, Message, Theme, Renderer>>) -> Self {
        let children = children
            .into_iter()
            .map(|x| Element::into(x.into()))
            .collect();
        Self {
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            align: Alignment::Start,
            clip: false,
            children,
            flush: true,
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    #[must_use]
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`Column`].
    #[must_use]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Column`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Column`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Column`].
    #[must_use]
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    #[must_use]
    pub fn align_x(mut self, align: impl Into<alignment::Vertical>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }

    /// Sets whether the contents of the [`Column`] should be clipped on
    /// overflow.
    #[must_use]
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets whether the end row element is flushed to the end when the alignment is set to Start,
    /// or the start row element is flushed to the start when the alignment is set to End.
    /// No effect for alignment set to Center.
    #[must_use]
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }

    /// Adds an element to the [`Column`].
    #[must_use]
    pub fn push(mut self, child: impl Into<Row<'a, Message, Theme, Renderer>>) -> Self {
        let child = child.into();
        let child_size = child.size_hint();
        self.width = self.width.enclose(child_size.width);
        self.height = self.height.enclose(child_size.height);
        self.children.push(child.into());
        self
    }

    /// Adds an element to the [`Column`], if `Some`.
    #[must_use]
    pub fn push_maybe(self, child: Option<impl Into<Row<'a, Message, Theme, Renderer>>>) -> Self {
        if let Some(child) = child {
            self.push(child)
        } else {
            self
        }
    }

    /// Extends the [`Column`] with the given children.
    #[must_use]
    pub fn extend(
        self,
        children: impl IntoIterator<Item = Row<'a, Message, Theme, Renderer>>,
    ) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

#[allow(clippy::mismatching_type_param_order)]
impl<'a, Message: 'a, Renderer> Default for FlushColumn<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer + 'a,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer: iced::advanced::Renderer + 'a>
    FromIterator<Row<'a, Message, Theme, Renderer>> for FlushColumn<'a, Message, Theme, Renderer>
{
    fn from_iter<T: IntoIterator<Item = Row<'a, Message, Theme, Renderer>>>(iter: T) -> Self {
        Self::with_children(iter)
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer> Widget<Message, Theme, Renderer>
    for FlushColumn<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.max_width(self.max_width);
        let node = layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            self.align,
            &self.children,
            &mut tree.children,
        );
        let mut container_x = f32::MAX;
        let mut container_width = 0.0f32;
        for row in node.children() {
            if row.size().width > container_width {
                container_width = row.size().width;
            }
            if row.bounds().x < container_x {
                container_x = row.bounds().x;
            }
        }
        let mut children = Vec::<Node>::new();
        for row in node.children() {
            let mut row_children = Vec::<Node>::new();
            let bounds = row.bounds();
            let width_diff = container_width - bounds.width;
            if !row.children().is_empty() {
                for element in row.children() {
                    let bounds = element.bounds();
                    let x = bounds.x
                        + match self.align {
                            Alignment::Start => 0.0,
                            Alignment::Center => width_diff / 2.0,
                            Alignment::End => width_diff,
                        };
                    let mut element_node =
                        Node::with_children(element.size(), element.children().to_owned());
                    element_node.move_to_mut(Point::new(x, bounds.y));
                    row_children.push(element_node);
                }
                if row_children.len() > 1 {
                    match self.align {
                        Alignment::Start => {
                            let element = row_children.last().expect("Always exists.");
                            let bounds = element.bounds();
                            let mut position = bounds.position();
                            let mut element_node =
                                Node::with_children(bounds.size(), element.children().to_owned());
                            position.x += width_diff;
                            element_node.move_to_mut(position);
                            let node = row_children.last_mut().expect("Always exists.");
                            *node = element_node;
                        }
                        Alignment::Center => {}
                        Alignment::End => {
                            let element = row_children.first().expect("Always exists.");
                            let bounds = element.bounds();
                            let mut position = bounds.position();
                            let mut element_node =
                                Node::with_children(bounds.size(), element.children().to_owned());
                            position.x -= width_diff;
                            element_node.move_to_mut(position);
                            let node = row_children.first_mut().expect("Always exists.");
                            *node = element_node;
                        }
                    }
                }
            }
            let mut row_node =
                Node::with_children(Size::new(container_width, row.size().height), row_children);
            row_node.move_to_mut(Point::new(container_x, bounds.y));
            children.push(row_node);
        }
        Node::with_children(node.size(), children)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
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
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
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
        if let Some(clipped_viewport) = layout.bounds().intersection(viewport) {
            for ((child, state), layout) in self
                .children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
            {
                child.as_widget().draw(
                    state,
                    renderer,
                    theme,
                    style,
                    layout,
                    cursor,
                    if self.clip {
                        &clipped_viewport
                    } else {
                        viewport
                    },
                );
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<FlushColumn<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(column: FlushColumn<'a, Message, Theme, Renderer>) -> Self {
        Self::new(column)
    }
}

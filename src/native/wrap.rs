//! A widget that displays its children in multiple horizontal or vertical runs.
//!
//! *This API requires the following crate features to be activated: `wrap`*

use iced_native::{
    event,
    layout::{Limits, Node},
    mouse, Alignment, Clipboard, Event, Layout, Length, Padding, Point, Rectangle, Shell, Size,
};
use iced_native::{widget::tree::Tree, Element, Widget};

use std::marker::PhantomData;

/// A container that distributes its contents horizontally.
#[allow(missing_debug_implementations)]
pub struct Wrap<'a, Message, Renderer, Direction> {
    /// The elements to distribute.
    pub elements: Vec<Element<'a, Message, Renderer>>,
    /// The alignment of the [`Wrap`](Wrap).
    pub alignment: Alignment,
    /// The width of the [`Wrap`](Wrap).
    pub width: Length,
    /// The height of the [`Wrap`](Wrap).
    pub height: Length,
    /// The maximum width of the [`Wrap`](Wrap).
    pub max_width: u32,
    /// The maximum height of the [`Wrap`](Wrap)
    pub max_height: u32,
    /// The padding of each element of the [`Wrap`](Wrap).
    pub padding: u16,
    /// The spacing between each element of the [`Wrap`](Wrap).
    pub spacing: u16,
    /// The spacing between each line of the [`Wrap`](Wrap).
    pub line_spacing: u16,
    /// The minimal length of each line of the [`Wrap`](Wrap).
    pub line_minimal_length: u32,
    #[allow(clippy::missing_docs_in_private_items)]
    _direction: PhantomData<Direction>,
}

impl<'a, Message, Renderer> Wrap<'a, Message, Renderer, direction::Horizontal> {
    /// Creates an empty horizontal [`Wrap`](Wrap).
    #[must_use]
    pub fn new() -> Self {
        Self::with_elements(Vec::new())
    }

    /// Creates a [`Wrap`](Wrap) with the given elements.
    ///
    /// It expects:
    ///     * the vector containing the [`Element`](iced_native::Element)s for this [`Wrap`](Wrap).
    #[must_use]
    pub fn with_elements(elements: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            elements,
            ..Wrap::default()
        }
    }
}

impl<'a, Message, Renderer> Wrap<'a, Message, Renderer, direction::Vertical> {
    /// Creates an empty vertical [`Wrap`](Wrap).
    #[must_use]
    pub fn new_vertical() -> Self {
        Self::with_elements_vertical(Vec::new())
    }

    /// Creates a [`Wrap`](Wrap) with the given elements.
    ///
    /// It expects:
    ///     * the vector containing the [`Element`](iced_native::Element)s for this [`Wrap`](Wrap).
    #[must_use]
    pub fn with_elements_vertical(elements: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            elements,
            ..Wrap::default()
        }
    }
}

impl<'a, Message, Renderer, Direction> Wrap<'a, Message, Renderer, Direction> {
    /// Sets the spacing of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the spacing of the lines of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn line_spacing(mut self, units: u16) -> Self {
        self.line_spacing = units;
        self
    }

    /// Sets the minimal length of the lines of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn line_minimal_length(mut self, units: u32) -> Self {
        self.line_minimal_length = units;
        self
    }

    /// Sets the padding of the elements in the [`Wrap`](Wrap).
    #[must_use]
    pub const fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn width_items(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn height_items(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the alignment of the [`Wrap`](Wrap).
    #[must_use]
    pub const fn align_items(mut self, align: Alignment) -> Self {
        self.alignment = align;
        self
    }

    /// Pushes an [`Element`](iced_native::Element) to the [`Wrap`](Wrap).
    #[must_use]
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }
}

impl<'a, Message, Renderer, Direction> Widget<Message, Renderer>
    for Wrap<'a, Message, Renderer, Direction>
where
    Self: WrapLayout<Renderer>,
    Renderer: iced_native::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.elements.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements);
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.inner_layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        self.elements
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
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn overlay<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        self.elements
            .iter()
            .zip(&mut state.children)
            .zip(layout.children())
            .find_map(|((child, state), layout)| child.as_widget().overlay(state, layout, renderer))
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
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state,
                    layout,
                    cursor_position,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .elements
            .iter()
            .zip(&state.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
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
}

impl<'a, Message, Renderer> From<Wrap<'a, Message, Renderer, direction::Vertical>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer,
    Message: 'a,
{
    fn from(
        wrap: Wrap<'a, Message, Renderer, direction::Vertical>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(wrap)
    }
}

impl<'a, Message, Renderer> From<Wrap<'a, Message, Renderer, direction::Horizontal>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer,
    Message: 'a,
{
    fn from(
        wrap: Wrap<'a, Message, Renderer, direction::Horizontal>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(wrap)
    }
}

impl<'a, Message, Renderer, Direction> Default for Wrap<'a, Message, Renderer, Direction> {
    fn default() -> Self {
        Self {
            elements: vec![],
            alignment: Alignment::Start,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            padding: 0,
            spacing: 0,
            line_spacing: 0,
            line_minimal_length: 10,
            _direction: PhantomData::default(),
        }
    }
}
/// A inner layout of the [`Wrap`](Wrap).
pub trait WrapLayout<Renderer>
where
    Renderer: iced_native::Renderer,
{
    /// A inner layout of the [`Wrap`](Wrap).
    fn inner_layout(&self, renderer: &Renderer, limits: &Limits) -> Node;
}

impl<'a, Message, Renderer> WrapLayout<Renderer>
    for Wrap<'a, Message, Renderer, direction::Horizontal>
where
    Renderer: iced_native::Renderer + 'a,
{
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn inner_layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = Padding::from(self.padding);
        let spacing = f32::from(self.spacing);
        let line_spacing = f32::from(self.line_spacing);
        #[allow(clippy::cast_precision_loss)] // TODO: possible precision loss
        let line_minimal_length = self.line_minimal_length as f32;
        let limits = limits
            .pad(padding)
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .max_height(self.max_height);
        let max_width = limits.max().width;

        let mut curse = f32::from(padding.left);
        let mut deep_curse = f32::from(padding.left);
        let mut current_line_height = line_minimal_length;
        let mut max_main = curse;
        let mut align = vec![];
        let mut start = 0;
        let mut end = 0;
        let mut nodes: Vec<Node> = self
            .elements
            .iter()
            .map(|elem| {
                let node_limit = Limits::new(
                    Size::new(limits.min().width, line_minimal_length),
                    limits.max(),
                );
                let mut node = elem.as_widget().layout(renderer, &node_limit);

                let size = node.size();

                let offset_init = size.width + spacing;
                let offset = curse + offset_init;

                if offset > max_width {
                    deep_curse += current_line_height + line_spacing;
                    align.push((start..end, current_line_height));
                    start = end;
                    end += 1;
                    current_line_height = line_minimal_length;
                    node.move_to(Point::new(f32::from(padding.left), deep_curse));
                    curse = offset_init + f32::from(padding.left);
                } else {
                    node.move_to(Point::new(curse, deep_curse));
                    curse = offset;
                    end += 1;
                }
                current_line_height = current_line_height.max(size.height);
                max_main = max_main.max(curse);

                node
            })
            .collect();
        if end != start {
            align.push((start..end, current_line_height));
        }
        for (range, max_length) in align {
            nodes[range].iter_mut().for_each(|node| {
                let size = node.size();
                let space = Size::new(size.width, max_length);
                node.align(Alignment::Start, self.alignment, space);
            });
        }
        let (width, height) = (
            max_main - f32::from(padding.left),
            deep_curse - f32::from(padding.left) + current_line_height,
        );
        let size = limits.resolve(Size::new(width, height));

        Node::with_children(size.pad(padding), nodes)
    }
}

impl<'a, Message, Renderer> WrapLayout<Renderer>
    for Wrap<'a, Message, Renderer, direction::Vertical>
where
    Renderer: iced_native::Renderer + 'a,
{
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn inner_layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = Padding::from(self.padding);
        let spacing = f32::from(self.spacing);
        let line_spacing = f32::from(self.line_spacing);
        #[allow(clippy::cast_precision_loss)] // TODO: possible precision loss
        let line_minimal_length = self.line_minimal_length as f32;
        let limits = limits
            .pad(padding)
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .max_height(self.max_height);
        let max_height = limits.max().height;

        let mut curse = f32::from(padding.left);
        let mut wide_curse = f32::from(padding.left);
        let mut current_line_width = line_minimal_length;
        let mut max_main = curse;
        let mut align = vec![];
        let mut start = 0;
        let mut end = 0;
        let mut nodes: Vec<Node> = self
            .elements
            .iter()
            .map(|elem| {
                let node_limit = Limits::new(
                    Size::new(line_minimal_length, limits.min().height),
                    limits.max(),
                );
                let mut node = elem.as_widget().layout(renderer, &node_limit);

                let size = node.size();

                let offset_init = size.height + spacing;
                let offset = curse + offset_init;

                if offset > max_height {
                    wide_curse += current_line_width + line_spacing;
                    align.push((start..end, current_line_width));
                    start = end;
                    end += 1;
                    current_line_width = line_minimal_length;
                    node.move_to(Point::new(wide_curse, f32::from(padding.left)));
                    curse = offset_init + f32::from(padding.left);
                } else {
                    node.move_to(Point::new(wide_curse, curse));
                    end += 1;
                    curse = offset;
                }
                current_line_width = current_line_width.max(size.width);
                max_main = max_main.max(curse);

                node
            })
            .collect();
        if end != start {
            align.push((start..end, current_line_width));
        }

        for (range, max_length) in align {
            nodes[range].iter_mut().for_each(|node| {
                let size = node.size();
                let space = Size::new(max_length, size.height);
                node.align(self.alignment, Alignment::Start, space);
            });
        }

        let (width, height) = (
            wide_curse - f32::from(padding.left) + current_line_width,
            max_main - f32::from(padding.left),
        );
        let size = limits.resolve(Size::new(width, height));

        Node::with_children(size.pad(padding), nodes)
    }
}

/// An optional directional attribute of the [`Wrap`](crate::Wrap).
pub mod direction {
    /// An vertical direction of the [`Wrap`](crate::Wrap).
    #[derive(Debug)]
    pub struct Vertical;
    /// An horizontal direction of the [`Wrap`](crate::Wrap).
    #[derive(Debug)]
    pub struct Horizontal;
}

//! A widget that displays its children in multiple horizontal or vertical runs.
//!
//! *This API requires the following crate features to be activated: `wrap`*

use iced_native::{
    event,
    layout::{self, Limits, Node},
    overlay, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Size, Widget,
};

/// A container that distributes its contents horizontally.
#[allow(missing_debug_implementations)]
pub struct Wrap<'a, Message, Renderer> {
    /// The elements to distribute.
    pub elements: Vec<Element<'a, Message, Renderer>>,
    // pub horizontal_alignment: Align,
    // pub vertical_alignment: Align,
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
    /// The height of each line of the [`Wrap`](Wrap).
    pub line_height: u32,
}

impl<'a, Message, Renderer> Wrap<'a, Message, Renderer> {
    /// Creates an empty [`Wrap`](Wrap).
    pub fn new() -> Self {
        Self::with_elements(Vec::new())
    }
    /// Creates a [`Wrap`](Wrap) with the given elements.
    ///
    /// It expects:
    ///     * the vector containing the [`Element`](iced_native::Element)s for this [`Wrap`](Wrap).
    pub fn with_elements(elements: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            elements,
            ..Default::default()
        }
    }
    /// Sets the spacing of the [`Wrap`](Wrap).
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }
    /// Sets the spacing of the lines of the [`Wrap`](Wrap).
    pub fn line_spacing(mut self, units: u16) -> Self {
        self.line_spacing = units;
        self
    }
    /// Sets the padding of the elements in the [`Wrap`](Wrap).
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }
    /// Sets the width of the [`Wrap`](Wrap).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
    /// Sets the height of the [`Wrap`](Wrap).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
    /// Sets the maximum width of the [`Wrap`](Wrap).
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }
    /// Sets the maximum height of the [`Wrap`](Wrap).
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    // pub fn vertical_alignment(mut self, align: Align) -> Self {
    //     self.vertical_alignment = align;
    //     self
    // }

    // pub fn horizontal_alignment(mut self, align: Align) -> Self {
    //     self.horizontal_alignment = align;
    //     self
    // }
    /// Pushes an [`Element`](iced_native::Element) to the [`Wrap`](Wrap).
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Wrap<'a, Message, Renderer>
where
    Renderer: iced_native::row::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let padding = self.padding as f32;
        let spacing = self.spacing as f32;
        let line_spacing = self.line_spacing as f32;
        let line_height = self.line_height as f32;
        let limits = limits
            .pad(padding)
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .max_height(self.max_height);
        let max_width = limits.max().width;

        let mut curse = padding;
        let mut deep_curse = padding;
        let mut current_line_height = line_height;
        let mut max_main = curse;

        let nodes: Vec<Node> = self
            .elements
            .iter()
            .map(|elem| {
                let node_limit =
                    Limits::new(Size::new(limits.min().width, line_height), limits.max());
                let mut node = elem.layout(renderer, &node_limit);

                let size = node.size();

                let offset_init = size.width + spacing;
                let offset = curse + offset_init;

                if offset > max_width {
                    deep_curse += current_line_height + line_spacing;
                    current_line_height = line_height;
                    node.move_to(Point::new(padding, deep_curse));
                    curse = offset_init + padding;
                } else {
                    node.move_to(Point::new(curse, deep_curse));
                    curse = offset;
                }
                current_line_height = current_line_height.max(size.height);
                max_main = max_main.max(curse);

                // node.align(self.horizontal_alignment, self.vertical_alignment, size);

                node
            })
            .collect();

        let (width, height) = (
            max_main - padding,
            deep_curse - padding + current_line_height,
        );
        // let size = limits.resolve(Size::new(width, height));

        Node::with_children(Size::new(width, height).pad(padding), nodes)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        // self.vertical_alignment.hash(state);
        // self.horizontal_alignment.hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);
        self.line_spacing.hash(state);
        self.padding.hash(state);
        self.spacing.hash(state);

        for elem in &self.elements {
            elem.hash_layout(state)
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        self.elements
            .iter_mut()
            .zip(layout.children())
            .map(|(child, layout)| {
                child.on_event(
                    event.clone(),
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    messages,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }
    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.elements
            .iter_mut()
            .zip(layout.children())
            .filter_map(|(child, layout)| child.overlay(layout))
            .next()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(defaults, &self.elements, layout, cursor_position, viewport)
    }
}

impl<'a, Message, Renderer> From<Wrap<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::row::Renderer,
    Message: 'a,
{
    fn from(wrap: Wrap<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(wrap)
    }
}

impl<'a, Message, Renderer> Default for Wrap<'a, Message, Renderer> {
    fn default() -> Self {
        Self {
            elements: vec![],
            // horizontal_alignment: Align::Center,
            // vertical_alignment: Align::Center,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            padding: 0,
            spacing: 0,
            line_spacing: 0,
            line_height: 10,
        }
    }
}

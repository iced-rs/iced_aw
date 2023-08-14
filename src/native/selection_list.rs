//! Display a dropdown list of selectable values.
pub mod list;
use crate::style::selection_list::StyleSheet;

use iced_widget::{
    container,
    core::{
        self, event,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        renderer,
        widget::Tree,
        Clipboard, Element, Event, Layout, Length, Rectangle, Shell, Size, Widget,
    },
    runtime::Font,
    scrollable, text,
    text::LineHeight,
    Container, Scrollable,
};

pub use list::List;
use std::hash::Hash;
use std::{fmt::Display, marker::PhantomData};

/// A widget for selecting a single value from a dynamic scrollable list of options.
#[allow(missing_debug_implementations)]
#[allow(clippy::type_repetition_in_bounds)]
pub struct SelectionList<'a, T, Message, Renderer = crate::Renderer>
where
    T: Clone + ToString + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
    Renderer: core::Renderer + core::text::Renderer<Font = core::Font>,
    Renderer::Theme: StyleSheet + container::StyleSheet,
{
    /// Container for Rendering List.
    container: Container<'a, Message, Renderer>,
    /// List of Elements to Render.
    options: &'a [T],
    /// Label Font
    font: Renderer::Font,
    /// The Containers Width
    width: Length,
    /// The Containers height
    height: Length,
    /// The padding Width
    padding: f32,
    /// The Text Size
    text_size: f32,
    /// Style for Looks
    style: <Renderer::Theme as StyleSheet>::Style,
}

#[allow(clippy::type_repetition_in_bounds)]
impl<'a, T, Message, Renderer> SelectionList<'a, T, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer + core::text::Renderer<Font = core::Font>,
    Renderer::Theme: StyleSheet + container::StyleSheet + scrollable::StyleSheet,
    T: Clone + Display + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Creates a new [`SelectionList`] with the given list of `options`,
    /// the current selected value, and the `message` to produce when an option is
    /// selected. This will default the `style`, `text_size` and `padding`. use `new_with`
    /// to set those.
    pub fn new(options: &'a [T], on_selected: impl Fn(usize, T) -> Message + 'static) -> Self {
        let container = Container::new(Scrollable::new(List {
            options,
            font: Font::default(),
            text_size: 12.0,
            padding: 5.0,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            on_selected: Box::new(on_selected),
            selected: None,
            phantomdata: PhantomData,
        }))
        .padding(1);

        Self {
            options,
            font: Font::default(),
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            container,
            width: Length::Fill,
            height: Length::Fill,
            padding: 5.0,
            text_size: 12.0,
        }
    }

    /// Creates a new [`SelectionList`] with the given list of `options`,
    /// the current selected value, the message to produce when an option is
    /// selected, the `style`, `text_size`, `padding` and `font`.
    pub fn new_with(
        options: &'a [T],
        on_selected: impl Fn(usize, T) -> Message + 'static,
        text_size: f32,
        padding: f32,
        style: <Renderer::Theme as StyleSheet>::Style,
        selected: Option<usize>,
        font: Font,
    ) -> Self {
        let container = Container::new(Scrollable::new(List {
            options,
            font,
            text_size,
            padding,
            style: style.clone(),
            selected,
            on_selected: Box::new(on_selected),
            phantomdata: PhantomData,
        }))
        .padding(1);

        Self {
            options,
            font,
            style,
            container,
            width: Length::Fill,
            height: Length::Fill,
            padding,
            text_size,
        }
    }

    /// Sets the width of the [`SelectionList`](SelectionList).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`SelectionList`](SelectionList).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`SelectionList`](SelectionList).
    #[must_use]
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for SelectionList<'a, T, Message, Renderer>
where
    T: 'a + Clone + ToString + Eq + Hash,
    Message: 'static,
    Renderer: core::Renderer + core::text::Renderer<Font = core::Font> + 'a,
    Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.container as &dyn Widget<_, _>)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.container as &dyn Widget<_, _>]);
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        use std::f32;

        let limits = limits.width(self.width).height(self.height);

        let max_width = match self.width {
            Length::Shrink => {
                let labels = self.options.iter().map(ToString::to_string);

                labels
                    .map(|label| {
                        let size = renderer.measure(
                            &label,
                            self.text_size,
                            LineHeight::default(),
                            self.font,
                            Size::new(f32::INFINITY, f32::INFINITY),
                            text::Shaping::Advanced,
                        );

                        size.width.round() as u32 + self.padding as u32 * 2
                    })
                    .max()
                    .unwrap_or(100)
            }
            _ => limits.max().width as u32,
        };

        let limits = limits.max_width(max_width as f32 + self.padding * 2.0);

        let content = self.container.layout(renderer, &limits);
        let size = limits.resolve(content.size());
        Node::with_children(size, vec![content])
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.container.on_event(
            &mut state.children[0],
            event,
            layout
                .children()
                .next()
                .expect("Scrollable Child Missing in Selection List"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.container
            .mouse_interaction(&state.children[0], layout, cursor, viewport, renderer)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_color: theme.style(&self.style).border_color,
                border_width: theme.style(&self.style).border_width,
                border_radius: (0.0).into(),
            },
            theme.style(&self.style).background,
        );

        self.container.draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout
                .children()
                .next()
                .expect("Scrollable Child Missing in Selection List"),
            cursor,
            &layout.bounds(),
        );
    }
}

impl<'a, T, Message, Renderer> From<SelectionList<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: Clone + ToString + Eq + Hash,
    Message: 'static,
    Renderer: 'a + core::Renderer + core::text::Renderer<Font = core::Font>,
    Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn from(selection_list: SelectionList<'a, T, Message, Renderer>) -> Self {
        Element::new(selection_list)
    }
}

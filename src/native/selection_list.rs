//! Display a dropdown list of selectable values.
pub mod list;
use crate::selection_list::StyleSheet;

use iced_native::{
    event,
    layout::{Limits, Node},
    mouse, renderer, Clipboard, Event, Layout, Length, Point, Rectangle, Shell, Size,
};

use iced_native::widget::tree::Tree;
use iced_native::widget::{Container, Scrollable};
use iced_native::{Element, Widget};

pub use list::List;
use std::borrow::Cow;
use std::marker::PhantomData;

/// A widget for selecting a single value from a dynamic scrollable list of options.
#[allow(missing_debug_implementations)]
#[allow(clippy::type_repetition_in_bounds)]
pub struct SelectionList<'a, T, Message, Renderer>
where
    T: Clone + ToString,
    [T]: ToOwned<Owned = Vec<T>>,
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: StyleSheet + iced_style::container::StyleSheet,
{
    /// Container for Rendering List.
    container: Container<'a, Message, Renderer>,
    /// List of Elements to Render.
    options: Cow<'a, [T]>,
    /// Label Font
    font: Renderer::Font,
    /// The Containers Width
    width: Length,
    /// The Containers height
    height: Length,
    /// The padding Width
    padding: u16,
    /// The Text Size
    text_size: u16,
    /// Style for Looks
    style: <Renderer::Theme as StyleSheet>::Style,
}

#[allow(clippy::type_repetition_in_bounds)]
impl<'a, T, Message, Renderer> SelectionList<'a, T, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme:
        StyleSheet + iced_style::container::StyleSheet + iced_style::scrollable::StyleSheet,
    T: Clone + ToString + Eq,
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Creates a new [`SelectionList`] with the given list of `options`,
    /// the current selected value, and the `message` to produce when an option is
    /// selected. This will default the `style`, `text_size` and `padding`. use `new_with`
    /// to set those.
    pub fn new(
        options: impl Into<Cow<'a, [T]>>,
        on_selected: impl Fn(T) -> Message + 'static,
    ) -> Self {
        let options = options.into();
        let container = Container::new(Scrollable::new(List {
            options: options.clone(),
            font: iced_graphics::Font::default(),
            text_size: 12,
            padding: 5,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            on_selected: Box::new(on_selected),
            phantomdata: PhantomData::default(),
        }))
        .padding(1);

        Self {
            options,
            font: iced_graphics::Font::default(),
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            container,
            width: Length::Fill,
            height: Length::Fill,
            padding: 5,
            text_size: 12,
        }
    }

    /// Creates a new [`SelectionList`] with the given list of `options`,
    /// the current selected value, the message to produce when an option is
    /// selected, the `style`, `text_size` and `padding`.
    pub fn new_with(
        options: impl Into<Cow<'a, [T]>>,
        on_selected: impl Fn(T) -> Message + 'static,
        text_size: u16,
        padding: u16,
        style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self {
        let options = options.into();
        let container = Container::new(Scrollable::new(List {
            options: options.clone(),
            font: iced_graphics::Font::default(),
            text_size,
            padding,
            style,
            on_selected: Box::new(on_selected),
            phantomdata: PhantomData::default(),
        }))
        .padding(1);

        Self {
            options,
            font: iced_graphics::Font::default(),
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
    T: 'a + Clone + ToString + Eq,
    Message: 'static,
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font> + 'a,
    Renderer::Theme: StyleSheet + iced_style::container::StyleSheet,
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
                        let (width, _) = renderer.measure(
                            &label,
                            self.text_size,
                            self.font,
                            Size::new(f32::INFINITY, f32::INFINITY),
                        );

                        width.round() as u32 + u32::from(self.padding * 2)
                    })
                    .max()
                    .unwrap_or(100)
            }
            _ => limits.max().width as u32,
        };

        let limits = limits.max_width(max_width + u32::from(self.padding * 2));

        let content = self.container.layout(renderer, &limits);
        let size = limits.resolve(content.size());
        Node::with_children(size, vec![content])
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
        self.container.on_event(
            &mut state.children[0],
            event,
            layout
                .children()
                .next()
                .expect("Scrollable Child Missing in Selection List"),
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.container.mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_color: theme.style(self.style).border_color,
                border_width: theme.style(self.style).border_width,
                border_radius: 0.0,
            },
            theme.style(self.style).background,
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
            cursor_position,
            &layout.bounds(),
        );
    }
}

impl<'a, T, Message, Renderer> From<SelectionList<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: Clone + ToString + Eq,
    Message: 'static,
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: StyleSheet + iced_style::container::StyleSheet,
{
    fn from(selection_list: SelectionList<'a, T, Message, Renderer>) -> Self {
        Element::new(selection_list)
    }
}

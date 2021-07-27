//! Display a dropdown list of selectable values.
pub mod list;
use crate::selection_list;
use iced_native::{
    container,
    event::{self, Event},
    layout, scrollable, text, Clipboard, Container, Element, Hasher, Layout, Length, Point,
    Rectangle, Scrollable, Size, Widget,
};
pub use list::List;
use std::marker::PhantomData;

/// A widget for selecting a single value from a dynamic scrollable list of options.
#[allow(missing_debug_implementations)]
pub struct SelectionList<'a, T, Message, Renderer: self::Renderer>
where
    T: Clone + ToString,
{
    /// Container for Rendering List.
    container: Container<'a, Message, Renderer>,
    /// List of Elements to Render.
    options: &'a [T],
    /// Label Font
    font: Renderer::Font,
    /// Style for Looks
    style: selection_list::Style,
}

/// The local state of a [`SelectionList`].
#[derive(Debug, Clone)]
pub struct State<T> {
    /// Statehood of Scrollbar
    scrollable: scrollable::State,
    /// Statehood of hovered_option
    hovered_option: Option<usize>,
    /// Statehood of last_selection
    last_selection: Option<T>,
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self {
            scrollable: scrollable::State::default(),
            hovered_option: Option::default(),
            last_selection: Option::default(),
        }
    }
}

impl<'a, T, Message, Renderer: self::Renderer> SelectionList<'a, T, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a,
    T: Clone + ToString + Eq,
{
    /// Creates a new [`SelectionList`] with the given [`State`], a list of options,
    /// the current selected value, and the message to produce when an option is
    /// selected.
    pub fn new(
        state: &'a mut State<T>,
        options: &'a [T],
        selected: &Option<T>,
        on_selected: impl Fn(T) -> Message + 'static,
        style: selection_list::Style,
    ) -> Self {
        let State {
            hovered_option,
            last_selection,
            ..
        } = state;

        *hovered_option = options
            .iter()
            .position(|option| Some(option) == selected.as_ref());

        let container = Container::new(Scrollable::new(&mut state.scrollable).push(List {
            options,
            hovered_option,
            last_selection,
            font: Default::default(),
            style,
            on_selected: Box::new(on_selected),
            phantomdata: PhantomData::default(),
        }))
        .padding(1);

        Self {
            options,
            font: Default::default(),
            style,
            container,
        }
    }
}

impl<'a, T: 'a, Message, Renderer> Widget<Message, Renderer>
    for SelectionList<'a, T, Message, Renderer>
where
    T: Clone + ToString + Eq,
    Message: 'static,
    Renderer: self::Renderer + scrollable::Renderer + 'a,
{
    fn width(&self) -> Length {
        self.style.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use std::f32;

        let limits = limits.width(self.style.width).height(self.style.height);

        let max_width = match self.style.width {
            Length::Shrink => {
                let labels = self.options.iter().map(ToString::to_string);

                labels
                    .map(|label| {
                        let (width, _) = renderer.measure(
                            &label,
                            self.style.text_size,
                            self.font,
                            Size::new(f32::INFINITY, f32::INFINITY),
                        );

                        width.round() as u32 + u32::from(self.style.padding * 2)
                    })
                    .max()
                    .unwrap_or(100)
            }
            _ => limits.max().width as u32,
        };

        let limits = limits.max_width(max_width + u32::from(self.style.padding * 2));

        let content = self.container.layout(renderer, &limits);
        let size = limits.resolve(content.size());
        layout::Node::with_children(size, vec![content])
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash as _;
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        match self.style.width {
            Length::Shrink => {
                self.options
                    .iter()
                    .map(ToString::to_string)
                    .for_each(|label| label.hash(state));
            }
            _ => {
                self.style.width.hash(state);
            }
        }

        self.container.hash_layout(state);
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
        self.container.on_event(
            event,
            layout
                .children()
                .next()
                .expect("Scrollable Child Missing in Selection List"),
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        let primitives = self.container.draw(
            renderer,
            defaults,
            layout
                .children()
                .next()
                .expect("Scrollable Child Missing in Selection List"),
            cursor_position,
            &layout.bounds(),
        );

        renderer.decorate(layout.bounds(), cursor_position, &self.style, primitives)
    }
}

/// The renderer of a [`SelectionList`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`SelectionList`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer:
    text::Renderer + list::Renderer + container::Renderer + scrollable::Renderer
{
    /// Decorates a the list of options of a [`SelectionList`].
    ///
    /// This method can be used to draw a background for the [`SelectionList`].
    fn decorate(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        style: &selection_list::Style,
        primitive: Self::Output,
    ) -> Self::Output;

    /// Draws the list of options of a [`SelectionList`].
    fn draw(&mut self) -> Self::Output;
}

impl<'a, T, Message, Renderer> From<SelectionList<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: Clone + ToString + Eq,
    Message: 'static,
    Renderer: self::Renderer + 'a,
{
    fn from(selection_list: SelectionList<'a, T, Message, Renderer>) -> Self {
        Element::new(selection_list)
    }
}

//! Build and show dropdown `ListMenus`.
use iced_native::{
    container::{self, Container},
    event::{self, Event},
    layout, mouse, overlay, scrollable, text, touch, Clipboard, Element, Hasher, Layout, Length,
    Point, Rectangle, Scrollable, Size, Vector, Widget,
};

/// A list of selectable options.
#[allow(missing_debug_implementations)]
pub struct ListMenu<'a, T, Renderer: self::Renderer> {
    /// The state of the [`ListMenu']
    state: &'a mut State,
    /// Selection Options to render
    options: &'a [T],
    /// Mouse Hovered Item for Rendering.
    hovered_option: &'a mut Option<usize>,
    /// Mouse button Down selection.
    last_selection: &'a mut Option<T>,
    /// Width of the List box within the Layout
    width: u16,
    /// surrounding Padding of Element. [`Padding`] was not exportable?
    padding: u16,
    /// Size of the Text to Render
    text_size: Option<u16>,
    /// Font to Render Glyphs from
    font: Renderer::Font,
    /// Style for Looks
    style: <Renderer as self::Renderer>::Style,
}

impl<'a, T, Renderer> ListMenu<'a, T, Renderer>
where
    T: ToString + Clone,
    Renderer: self::Renderer + 'a,
{
    /// Creates a new [`ListMenu`] with the given [`State`], a list of options, and
    /// the message to produced when an option is selected.
    pub fn new(
        state: &'a mut State,
        options: &'a [T],
        hovered_option: &'a mut Option<usize>,
        last_selection: &'a mut Option<T>,
    ) -> Self {
        ListMenu {
            state,
            options,
            hovered_option,
            last_selection,
            width: 0,
            padding: 0,
            text_size: None,
            font: Default::default(),
            style: Default::default(),
        }
    }

    /// Sets the width of the [`ListMenu`].
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Sets the [`Padding`] of the [`ListMenu`].
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the text size of the [`ListMenu`].
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the font of the [`ListMenu`].
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`ListMenu`].
    pub fn style(mut self, style: impl Into<<Renderer as self::Renderer>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Turns the [`ListMenu`] into an overlay [`Element`] at the given target
    /// position.
    ///
    /// The `target_height` will be used to display the `ListMenu` either on top
    /// of the target or under it, depending on the screen position and the
    /// dimensions of the [`ListMenu`].
    pub fn overlay<Message: 'a>(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(ListOverlay::new(self)))
    }
}

/// The local state of a [`ListMenu`].
#[derive(Debug, Clone, Default)]
pub struct State {
    /// The state of the [`Scrollable']
    scrollable: scrollable::State,
}

impl State {
    /// Creates a new [`State`] for a [`ListMenu`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// The Overlay for[`ListMenu`].
struct ListOverlay<'a, Message, Renderer: self::Renderer> {
    /// Element to contain List
    container: Container<'a, Message, Renderer>,
    /// width of the Element
    width: u16,
    /// Style of the ListMenu overlay
    style: <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer: self::Renderer> ListOverlay<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a,
{
    /// Creates a new [`ListOverlay`] for a [`ListMenu`].
    pub fn new<T>(listmenu: ListMenu<'a, T, Renderer>) -> Self
    where
        T: Clone + ToString,
    {
        let ListMenu {
            state,
            options,
            hovered_option,
            last_selection,
            width,
            padding,
            font,
            text_size,
            style,
        } = listmenu;

        let container = Container::new(Scrollable::new(&mut state.scrollable).push(List {
            options,
            hovered_option,
            last_selection,
            font,
            text_size,
            padding,
            style: style.clone(),
        }))
        .padding(1);

        Self {
            container,
            width,
            style,
        }
    }
}

impl<'a, Message, Renderer> overlay::Overlay<Message, Renderer>
    for ListOverlay<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        let limits =
            layout::Limits::new(Size::ZERO, Size::new(bounds.width - position.x, position.y))
                .width(Length::Units(self.width));

        let mut node = self.container.layout(renderer, &limits);

        node.move_to(position - Vector::new(0.0, node.size().height));
        node
    }

    fn hash_layout(&self, state: &mut Hasher, position: Point) {
        use std::hash::Hash;
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
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
            layout,
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
    ) -> Renderer::Output {
        let primitives = self.container.draw(
            renderer,
            defaults,
            layout,
            cursor_position,
            &layout.bounds(),
        );

        renderer.decorate(layout.bounds(), cursor_position, &self.style, primitives)
    }
}

/// The Private [`List`] Handles the Actual list rendering.
struct List<'a, T, Renderer: self::Renderer> {
    /// Options pointer to hold all rendered strings
    options: &'a [T],
    /// Hovered Item Pointer
    hovered_option: &'a mut Option<usize>,
    /// Last choosen Item Clicked for Processing
    last_selection: &'a mut Option<T>,
    /// padding within the list for string offsets
    padding: u16,
    /// Text size use for rendering the font
    text_size: Option<u16>,
    /// Font needed for rendering the String
    font: Renderer::Font,
    /// Style for Font colors and Box hover colors.
    style: <Renderer as self::Renderer>::Style,
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for List<'a, T, Renderer>
where
    T: Clone + ToString,
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use std::f32;

        let limits = limits.width(Length::Fill).height(Length::Shrink);
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());

        let size = {
            #[allow(clippy::cast_precision_loss)]
            let intrinsic = Size::new(
                0.0,
                f32::from(text_size + (self.padding * 2)) * self.options.len() as f32,
            );

            limits.resolve(intrinsic)
        };

        layout::Node::new(size)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash as _;
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.options.len().hash(state);
        self.text_size.hash(state);
        self.padding.hash(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let bounds = layout.bounds();

                if bounds.contains(cursor_position) {
                    if let Some(index) = *self.hovered_option {
                        if let Some(option) = self.options.get(index) {
                            *self.last_selection = Some(option.clone());
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let bounds = layout.bounds();

                if bounds.contains(cursor_position) {
                    let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());

                    *self.hovered_option = Some(
                        ((cursor_position.y - bounds.y) / f32::from(text_size + (self.padding * 2)))
                            as usize,
                    );
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();

                if bounds.contains(cursor_position) {
                    let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());

                    *self.hovered_option = Some(
                        ((cursor_position.y - bounds.y) / f32::from(text_size + (self.padding * 2)))
                            as usize,
                    );

                    if let Some(index) = *self.hovered_option {
                        if let Some(option) = self.options.get(index) {
                            *self.last_selection = Some(option.clone());
                        }
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        self::Renderer::draw(
            renderer,
            layout.bounds(),
            cursor_position,
            viewport,
            self.options,
            *self.hovered_option,
            self.padding,
            self.text_size.unwrap_or_else(|| renderer.default_size()),
            self.font,
            &self.style,
        )
    }
}

/// The renderer of a [`ListMenu`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`ListMenu`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer: scrollable::Renderer + container::Renderer + text::Renderer {
    /// The [`ListMenu`] style supported by this renderer.
    type Style: Default + Clone;

    /// Decorates a the list of options of a [`ListMenu`].
    ///
    /// This method can be used to draw a background for the [`ListMenu`].
    fn decorate(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        style: &<Self as Renderer>::Style,
        primitive: Self::Output,
    ) -> Self::Output;

    /// Draws the list of options of a [`ListMenu`].
    #[allow(clippy::too_many_arguments)]
    fn draw<T: ToString>(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        viewport: &Rectangle,
        options: &[T],
        hovered_option: Option<usize>,
        padding: u16,
        text_size: u16,
        font: Self::Font,
        style: &<Self as Renderer>::Style,
    ) -> Self::Output;
}

impl<'a, T, Message, Renderer> From<List<'a, T, Renderer>> for Element<'a, Message, Renderer>
where
    T: ToString + Clone,
    Message: 'a,
    Renderer: 'a + self::Renderer,
{
    fn from(list: List<'a, T, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(list)
    }
}

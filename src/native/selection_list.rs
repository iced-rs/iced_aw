//! Display a dropdown list of selectable values.
use super::overlay::list_menu::{self, ListMenu};
use iced_native::{
    event::{self, Event},
    layout, mouse, overlay, scrollable, text, touch, Clipboard, Element, Hasher, Layout, Length,
    Padding, Point, Rectangle, Size, Widget,
};

/// A widget for selecting a single value from a dynamic scrollable list of options.
#[allow(missing_debug_implementations)]
pub struct SelectionList<'a, T, Message, Renderer: self::Renderer>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// The state of the [`ListMenu`](T).
    list_menu: &'a mut list_menu::State,
    /// Mouse Hovered Item for Rendering.
    hovered_option: &'a mut Option<usize>,
    /// Mouse button Down selection.
    last_selection: &'a mut Option<T>,
    /// Function Pointer On Select to call on Mouse button press.
    on_selected: Box<dyn Fn(T) -> Message>,
    /// current selected It for use with hover override on creation.
    selected: Option<T>,
    /// List of Elements to Render.
    options: &'a [T],
    /// Width of the List box within the Layout
    width: Length,
    /// surrounding Padding of Element. [`Padding`] was not exportable?
    padding: u16,
    /// Size of the Text to Render
    text_size: Option<u16>,
    /// Font to Render Glyphs from
    font: Renderer::Font,
    /// Style for Looks
    style: <Renderer as self::Renderer>::Style,
}

/// The local state of a [`SelectionList`].
#[derive(Debug, Clone)]
pub struct State<T> {
    /// Statehood of ListMenu
    list_menu: list_menu::State,
    /// Statehood of hovered_option
    hovered_option: Option<usize>,
    /// Statehood of last_selection
    last_selection: Option<T>,
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self {
            list_menu: list_menu::State::default(),
            hovered_option: Option::default(),
            last_selection: Option::default(),
        }
    }
}

impl<'a, T: 'a, Message, Renderer: self::Renderer> SelectionList<'a, T, Message, Renderer>
where
    T: ToString + Eq,
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Creates a new [`SelectionList`] with the given [`State`], a list of options,
    /// the current selected value, and the message to produce when an option is
    /// selected.
    pub fn new(
        state: &'a mut State<T>,
        options: &'a [T],
        selected: Option<T>,
        on_selected: impl Fn(T) -> Message + 'static,
    ) -> Self {
        let State {
            list_menu,
            hovered_option,
            last_selection,
        } = state;

        Self {
            list_menu,
            hovered_option,
            last_selection,
            on_selected: Box::new(on_selected),
            options,
            selected,
            width: Length::Shrink,
            text_size: None,
            padding: <Renderer as self::Renderer>::DEFAULT_PADDING,
            font: Default::default(),
            style: Default::default(),
        }
    }

    /// Sets the width of the [`SelectionList`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the [`Padding`] of the [`SelectionList`].
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the text size of the [`SelectionList`].
    pub fn text_size(mut self, size: u16) -> Self {
        self.text_size = Some(size);
        self
    }

    /// Sets the font of the [`SelectionList`].
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Private function to update the Hovered option
    fn update_hovered_option(&mut self) {
        let selected = self.selected.as_ref();

        *self.hovered_option = self
            .options
            .iter()
            .position(|option| Some(option) == selected);
    }

    /// Sets the style of the [`SelectionList`].
    pub fn style(mut self, style: impl Into<<Renderer as self::Renderer>::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, T: 'a, Message, Renderer> Widget<Message, Renderer>
    for SelectionList<'a, T, Message, Renderer>
where
    T: Clone + ToString + Eq,
    [T]: ToOwned<Owned = Vec<T>>,
    Message: 'static,
    Renderer: self::Renderer + scrollable::Renderer + 'a,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use std::f32;

        let limits = limits
            .width(self.width)
            .height(Length::Shrink)
            .pad(Padding::from(self.padding));

        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());

        let max_width = match self.width {
            Length::Shrink => {
                let labels = self.options.iter().map(ToString::to_string);

                labels
                    .map(|label| {
                        let (width, _) = renderer.measure(
                            &label,
                            text_size,
                            self.font,
                            Size::new(f32::INFINITY, f32::INFINITY),
                        );

                        width.round() as u32
                    })
                    .max()
                    .unwrap_or(100)
            }
            _ => 0,
        };

        let size = {
            let intrinsic = Size::new(
                f64::from(max_width) as f32 + f32::from(text_size) + f32::from(self.padding),
                f32::from(text_size),
            );

            limits.resolve(intrinsic).pad(Padding::from(self.padding))
        };

        layout::Node::new(size)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash as _;

        match self.width {
            Length::Shrink => {
                self.options
                    .iter()
                    .map(ToString::to_string)
                    .for_each(|label| label.hash(state));
            }
            _ => {
                self.width.hash(state);
            }
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let event_status = if layout.bounds().contains(cursor_position) {
                    self.update_hovered_option();

                    event::Status::Captured
                } else {
                    event::Status::Ignored
                };

                self.last_selection
                    .take()
                    .map_or(event_status, |last_selection| {
                        self.selected = Some(last_selection.clone());
                        messages.push((self.on_selected)(last_selection));

                        event::Status::Captured
                    })
            }
            _ => event::Status::Ignored,
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        self::Renderer::draw(renderer)
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        let bounds = layout.bounds();

        let mut list_menu = ListMenu::new(
            &mut self.list_menu,
            self.options,
            &mut self.hovered_option,
            &mut self.last_selection,
        )
        .width(bounds.width.round() as u16)
        .padding(self.padding)
        .font(self.font)
        .style(Renderer::menu_style(&self.style));

        if let Some(text_size) = self.text_size {
            list_menu = list_menu.text_size(text_size);
        }

        Some(list_menu.overlay(layout.position()))
    }
}

/// The renderer of a [`SelectionList`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`SelectionList`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer: text::Renderer + list_menu::Renderer {
    /// The default padding of a [`SelectionList`].
    const DEFAULT_PADDING: u16;

    /// The [`SelectionList`] style supported by this renderer.
    type Style: Default;

    /// Returns the style of the [`ListMenu`] of the [`SelectionList`].
    fn menu_style(style: &<Self as Renderer>::Style) -> <Self as list_menu::Renderer>::Style;

    /// Draws a [`SelectionList`].
    fn draw(&mut self) -> Self::Output;
}

impl<'a, T, Message, Renderer> From<SelectionList<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: Clone + ToString + Eq,
    [T]: ToOwned<Owned = Vec<T>>,
    Message: 'static,
    Renderer: self::Renderer + 'a,
{
    fn from(selection_list: SelectionList<'a, T, Message, Renderer>) -> Self {
        Element::new(selection_list)
    }
}

//! Display a dropdown list of selectable values.
pub mod list;
use crate::style::{
    selection_list::{Catalog, Style},
    Status, StyleFn,
};

use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        text::{paragraph, Paragraph, Text},
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    alignment::{Horizontal, Vertical},
    mouse::{self, Cursor},
    widget::{
        container, scrollable,
        text::{self, LineHeight, Wrapping},
        Container, Scrollable,
    },
    Border, Element, Event, Font, Length, Padding, Pixels, Rectangle, Size,
};
use std::{fmt::Display, hash::Hash, marker::PhantomData};

pub use list::List;

/// A widget for selecting a single value from a dynamic scrollable list of options.
#[allow(missing_debug_implementations)]
#[allow(clippy::type_repetition_in_bounds)]
pub struct SelectionList<'a, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: Clone + ToString + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog + container::Catalog,
{
    /// Container for Rendering List.
    container: Container<'a, Message, Theme, Renderer>,
    /// List of Elements to Render.
    options: &'a [T],
    /// Label Font
    font: Renderer::Font,
    /// The Containers Width
    width: Length,
    /// The Containers height
    height: Length,
    /// The padding Width
    padding: Padding,
    /// The Text Size
    text_size: f32,
    /// Style for Looks
    class: <Theme as Catalog>::Class<'a>,
}

#[allow(clippy::type_repetition_in_bounds)]
impl<'a, T, Message, Theme, Renderer> SelectionList<'a, T, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a + Catalog + container::Catalog + scrollable::Catalog,
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
            padding: 5.0.into(),
            class: <Theme as Catalog>::default(),
            on_selected: Box::new(on_selected),
            selected: None,
            phantomdata: PhantomData,
        }))
        .padding(1);

        Self {
            options,
            font: Font::default(),
            class: <Theme as Catalog>::default(),
            container,
            width: Length::Fill,
            height: Length::Fill,
            padding: 5.0.into(),
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
        padding: impl Into<Padding>,
        style: impl Fn(&Theme, Status) -> Style + 'a + Clone,
        selected: Option<usize>,
        font: Font,
    ) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        let class: <Theme as Catalog>::Class<'a> =
            (Box::new(style.clone()) as StyleFn<'a, Theme, Style>).into();
        let class2: <Theme as Catalog>::Class<'a> =
            (Box::new(style) as StyleFn<'a, Theme, Style>).into();

        let padding = padding.into();

        let container = Container::new(Scrollable::new(List {
            options,
            font,
            text_size,
            padding,
            class: class2,
            selected,
            on_selected: Box::new(on_selected),
            phantomdata: PhantomData,
        }))
        .padding(1);

        Self {
            options,
            font,
            class,
            container,
            width: Length::Fill,
            height: Length::Fill,
            padding,
            text_size,
        }
    }

    /// Sets the width of the [`SelectionList`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`SelectionList`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the style of the [`SelectionList`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`SelectionList`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for SelectionList<'a, T, Message, Theme, Renderer>
where
    T: 'a + Clone + ToString + Eq + Hash + Display,
    Message: 'static,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font> + 'a,
    Theme: Catalog + container::Catalog,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.container as &dyn Widget<_, _, _>)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.container as &dyn Widget<_, _, _>]);
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        state.values = self
            .options
            .iter()
            .map(|_| paragraph::Plain::<Renderer::Paragraph>::default())
            .collect();
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer::Paragraph>::new(self.options))
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        use std::f32;

        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        let limits = limits.width(self.width).height(self.height);

        let max_width = match self.width {
            Length::Shrink => self
                .options
                .iter()
                .enumerate()
                .map(|(id, val)| {
                    let s: &str = &val.to_string();
                    let text = Text {
                        content: s,
                        size: Pixels(self.text_size),
                        line_height: LineHeight::default(),
                        bounds: Size::INFINITY,
                        font: self.font,
                        align_x: Horizontal::Left.into(),
                        align_y: Vertical::Top,
                        shaping: text::Shaping::Advanced,
                        wrapping: Wrapping::default(),
                    };

                    let _ = state.values[id].update(text);
                    (state.values[id].min_bounds().width + self.padding.horizontal()).round() as u32
                })
                .max()
                .unwrap_or(100),
            _ => limits.max().width as u32,
        };

        let limits = limits.max_width(max_width as f32 + self.padding.horizontal());

        let content = self
            .container
            .layout(&mut tree.children[0], renderer, &limits);
        let size = limits.resolve(self.width, self.height, content.size());
        Node::with_children(size, vec![content])
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        viewport: &Rectangle,
    ) {
        self.container.update(
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
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style_sheet = <Theme as Catalog>::style(theme, &self.class, Status::Active);

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (0.0).into(),
                        width: style_sheet.border_width,
                        color: style_sheet.border_color,
                    },
                    ..Default::default()
                },
                style_sheet.background,
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
                &clipped_viewport,
            );
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<SelectionList<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Clone + ToString + Eq + Hash + Display,
    Message: 'static,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a + Catalog + container::Catalog,
{
    fn from(selection_list: SelectionList<'a, T, Message, Theme, Renderer>) -> Self {
        Element::new(selection_list)
    }
}

/// A Paragraph cache to enhance speed of layouting.
#[derive(Default, Clone)]
pub struct State<P: Paragraph> {
    values: Vec<paragraph::Plain<P>>,
}

impl<P: Paragraph> State<P> {
    /// Creates a new [`State`], representing an unfocused [`TextInput`](iced::widget::TextInput).
    pub fn new<T>(options: &[T]) -> Self
    where
        T: Clone + Display + Eq + Hash,
        [T]: ToOwned<Owned = Vec<T>>,
    {
        Self {
            values: options
                .iter()
                .map(|_| paragraph::Plain::<P>::default())
                .collect(),
        }
    }
}

//! Display a dropdown list of selectable values.
pub mod list;
use crate::style::{
    Status, StyleFn,
    selection_list::{Catalog, Style},
};

use iced_core::{
    Border, Clipboard, Element, Event, Font, Layout, Length, Padding, Pixels, Rectangle, Shell,
    Size, Widget,
    alignment::Vertical,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    renderer,
    text::{Paragraph, Text, paragraph},
    widget::{Tree, tree},
};
use iced_widget::{
    Container, Scrollable, container, scrollable,
    text::{self, LineHeight, Wrapping},
};
use std::{fmt::Display, hash::Hash, marker::PhantomData};

pub use list::List;

/// A widget for selecting a single value from a dynamic scrollable list of options.
#[allow(missing_debug_implementations)]
#[allow(clippy::type_repetition_in_bounds)]
pub struct SelectionList<
    'a,
    T,
    Message,
    Theme = iced_widget::Theme,
    Renderer = iced_widget::Renderer,
> where
    T: Clone + ToString + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
    Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
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
    Renderer: 'a + renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: 'a + Catalog + container::Catalog + scrollable::Catalog + iced_widget::text::Catalog,
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
    Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font> + 'a,
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

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
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
                        bounds: Size::INFINITE,
                        font: self.font,
                        align_x: text::Alignment::Left,
                        align_y: Vertical::Top,
                        shaping: text::Shaping::Advanced,
                        wrapping: Wrapping::default(),
                    };

                    let _ = state.values[id].update(text);
                    (state.values[id].min_bounds().width + self.padding.x()).round() as u32
                })
                .max()
                .unwrap_or(100),
            _ => limits.max().width as u32,
        };

        let limits = limits.max_width(max_width as f32 + self.padding.x());

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
        );
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
                    ..renderer::Quad::default()
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

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<()>,
    ) {
        Widget::<Message, Theme, Renderer>::operate(
            &mut self.container,
            &mut state.children[0],
            layout
                .children()
                .next()
                .expect("Scrollable Child Missing in Selection List"),
            renderer,
            operation,
        );
    }
}

impl<'a, T, Message, Theme, Renderer> From<SelectionList<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Clone + ToString + Eq + Hash + Display,
    Message: 'static,
    Renderer: 'a + renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
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
    /// Creates a new [`State`], representing an unfocused [`TextInput`](iced_widget::TextInput).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum TestMessage {
        Selected(usize, String),
    }

    type TestSelectionList<'a> = SelectionList<'a, String, TestMessage, iced_widget::Theme>;

    #[test]
    fn selection_list_new_creates_instance() {
        let options = vec!["Option 1".to_owned(), "Option 2".to_owned()];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected);

        assert_eq!(selection_list.options.len(), 2);
        assert_eq!(selection_list.width, Length::Fill);
        assert_eq!(selection_list.height, Length::Fill);
    }

    #[test]
    fn selection_list_new_with_creates_instance() {
        let options = vec!["Option 1".to_owned(), "Option 2".to_owned()];

        let selection_list = TestSelectionList::new_with(
            &options,
            TestMessage::Selected,
            14.0,
            10.0,
            crate::style::selection_list::primary,
            Some(0),
            Font::default(),
        );

        assert_eq!(selection_list.options.len(), 2);
        assert_eq!(selection_list.text_size, 14.0);
    }

    #[test]
    fn selection_list_width_sets_value() {
        let options = vec!["Option 1".to_owned()];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected).width(200);

        assert_eq!(selection_list.width, Length::Fixed(200.0));
    }

    #[test]
    fn selection_list_height_sets_value() {
        let options = vec!["Option 1".to_owned()];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected).height(300);

        assert_eq!(selection_list.height, Length::Fixed(300.0));
    }

    #[test]
    fn selection_list_tag_returns_state_tag() {
        let options = vec!["Option 1".to_owned()];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected);

        let tag =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::tag(&selection_list);
        assert_eq!(
            tag,
            tree::Tag::of::<State<<iced_widget::Renderer as iced_core::text::Renderer>::Paragraph>>(
            )
        );
    }

    #[test]
    fn selection_list_has_one_child() {
        let options = vec!["Option 1".to_owned()];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected);

        let children = Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::children(
            &selection_list,
        );
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn selection_list_size_defaults() {
        let options = vec!["Option 1".to_owned()];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected);

        let size =
            Widget::<TestMessage, iced_widget::Theme, iced_widget::Renderer>::size(&selection_list);
        assert_eq!(size.width, Length::Fill);
        assert_eq!(size.height, Length::Shrink);
    }

    #[test]
    fn selection_list_multiple_options() {
        let options = vec![
            "Option 1".to_owned(),
            "Option 2".to_owned(),
            "Option 3".to_owned(),
        ];

        let selection_list = TestSelectionList::new(&options, TestMessage::Selected);

        assert_eq!(selection_list.options.len(), 3);
        assert_eq!(selection_list.options[0], "Option 1");
        assert_eq!(selection_list.options[1], "Option 2");
        assert_eq!(selection_list.options[2], "Option 3");
    }

    #[test]
    fn state_new_creates_empty_values() {
        type TestState = State<<iced_widget::Renderer as iced_core::text::Renderer>::Paragraph>;

        let options = vec!["A".to_owned(), "B".to_owned()];
        let state = TestState::new(&options);

        assert_eq!(state.values.len(), 2);
    }
}

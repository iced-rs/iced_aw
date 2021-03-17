//! A menu bar.
//!
//! *This API requires the following crate features to be activated: `menu`*
use std::hash::Hash;

use iced_graphics::{Point, Size};
use iced_native::{
    event, layout, mouse, overlay, row, touch, Clipboard, Element, Event, Layout, Length, Row,
    Widget,
};

use crate::core::renderer::DrawEnvironment;

use super::overlay::MenuOverlay;

/// A menu bar.
///
/// # Example
/// ```
/// # use iced_aw::menu::{State, Section, Entry};
/// # use iced_native::{Text, renderer::Null};
/// #
/// # pub type Menu<'a, Message> = iced_aw::native::Menu<'a, Message, Null>;
/// #[derive(Clone, Debug)]
/// enum Message {
///     Entry1,
///     Entry2,
///     Entry3,    
/// }
///
/// let mut menu_state = State::new();
///
/// let menu = Menu::new(&mut menu_state)
///     .push(Section::new(
///         Text::new("Section 1"),
///         vec![
///             Entry::Item(Text::new("Entry 1").into(), Some(Message::Entry1)),
///             Entry::Item(Text::new("Entry 2").into(), Some(Message::Entry2)),
///         ]
///     ))
///     .push(Section::new(
///         Text::new("Section2"),
///         vec![
///             Entry::Item(Text::new("Entry 3").into(), Some(Message::Entry3)),
///         ]
///     ));
/// ```
#[allow(missing_debug_implementations)]
pub struct Menu<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// The state of the [`Menu`](Menu).
    state: &'a mut State,
    /// A vector containing the [`Section`](Section)s of the [`Menu`](Menu).
    sections: Vec<Section<'a, Message, Renderer>>,
    /// The width of the [`Menu`](Menu).
    width: Length,
    /// The height of the [`Menu`](Menu).
    height: Length,
    /// The space between the [`Section`](Section)s of the [`Menu`](Menu).
    space: f32,
    /// The padding around the [`Section`](Section)s.
    padding: f32,
    /// The [`Style`](crate::style::menu::Style) of the [`Menu`](Menu).
    style: Renderer::Style,
}

impl<'a, Message, Renderer> Menu<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    /// Creates a new [`Menu`](Menu) with an empty list of sections.
    ///
    /// It expects:
    ///     * a mutable reference to the [`Menu`](Menu)'s [`State`](State).
    pub fn new(state: &'a mut State) -> Self {
        Menu::with_sections(state, Vec::new())
    }

    /// Creates a new [`Menu`](Menu) with the given list of sections.
    ///
    /// It expects:
    ///     * a mutable reference to the [`Menu`](Menu)'s [`State`](State).
    ///     * a vector containing the sections.
    pub fn with_sections(
        state: &'a mut State,
        sections: Vec<Section<'a, Message, Renderer>>,
    ) -> Self {
        Self {
            state,
            sections,
            width: Length::Fill,
            height: Length::Shrink,
            space: 10.0,
            padding: 5.0,
            style: Renderer::Style::default(),
        }
    }

    /// Pushes a [`Section`](Section) to the [`Menu`](Menu).
    pub fn push<S>(mut self, section: S) -> Self
    where
        S: Into<Section<'a, Message, Renderer>>,
    {
        self.sections.push(section.into());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Menu<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer + row::Renderer + crate::native::overlay::menu::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &iced_native::layout::Limits) -> layout::Node {
        let limits = limits.clone().width(self.width()).height(self.height());

        let width = Row::<Message, Renderer>::new()
            .width(Length::Fill)
            .layout(renderer, &limits)
            .bounds()
            .width;

        let section_limits = limits.clone().pad(self.padding).width(Length::Shrink);

        let mut entries: Vec<layout::Node> = self
            .sections
            .iter()
            .map(|section| section.label.layout(renderer, &section_limits))
            .collect();

        let mut offset = self.padding;

        entries.iter_mut().for_each(|entry| {
            entry.move_to(Point::new(offset, entry.bounds().y + self.padding));
            offset += entry.bounds().width + self.space
        });

        let max_height = entries
            .iter()
            .map(layout::Node::bounds)
            .map(|bounds| bounds.height)
            //.max()
            // https://stackoverflow.com/a/28446718
            .fold(0.0, |a: f32, b: f32| a.max(b));

        layout::Node::with_children(Size::new(width, max_height + 2.0 * self.padding), entries)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let bounds = layout.bounds();
        let children = layout.children();

        if self.state.stack.is_empty() {
            let (status, index) = self
                .sections
                .iter_mut()
                .zip(children)
                .enumerate()
                .map(|(i, (section, layout))| {
                    let mut status = (event::Status::Ignored, 0);
                    match event {
                        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                        | Event::Touch(touch::Event::FingerPressed { .. }) => {
                            if layout.bounds().contains(cursor_position)
                                && !section.entries.is_empty()
                            {
                                status = (event::Status::Captured, i);
                            }
                        }
                        _ => {}
                    }
                    status
                })
                .find(|(status, _)| *status == event::Status::Captured)
                .unwrap_or((event::Status::Ignored, 0));

            if status == event::Status::Captured {
                self.state.stack.push(index);
            }

            status
        } else {
            if bounds.contains(cursor_position) {
                let element = self.sections.iter().zip(children).enumerate().find(
                    |(_, (section, layout))| {
                        layout.bounds().contains(cursor_position) && !section.entries.is_empty()
                    },
                );

                if let Some((i, _)) = element {
                    self.state.stack.clear();
                    self.state.stack.push(i);
                }
            }

            event::Status::Ignored
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) -> Renderer::Output {
        <Renderer as self::Renderer>::draw(
            renderer,
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &self.style,
                viewport: Some(viewport),
                focus: (),
            },
            &self.sections,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.sections
            .iter()
            .for_each(|section| section.label.hash_layout(state));
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        if self.state.stack.is_empty() {
            return None;
        }

        let index = *self.state.stack.first()?;

        let bounds = layout.bounds();
        let entry_bounds = layout.children().nth(index)?.bounds();

        let position = Point::new(entry_bounds.x, bounds.y + bounds.height);

        Some(MenuOverlay::new(&mut self.state, self.sections.get(index)?, position).overlay())
    }
}

/// The renderer of  a [`Menu`](Menu).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Menu`](Menu) in your user interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Menu`](Menu).
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        section: &[Section<'_, Message, Self>],
    ) -> Self::Output;
}

impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        _section: &[Section<'_, Message, Self>],
    ) -> Self::Output {
    }
}

/// The state of the [`Menu`](Menu).
#[derive(Debug, Default)]
pub struct State {
    /// The stack containing the indices that build a path to the opened [`Entry`](Entry).
    pub(crate) stack: Vec<usize>,
}

impl State {
    /// Creates a new [`State`](State).
    #[must_use]
    pub const fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

/// A section of the menu containing a list of entries.
#[allow(missing_debug_implementations)]
pub struct Section<'a, Message, Renderer> {
    /// The label of the [`Section`](Section).
    pub(crate) label: Element<'a, Message, Renderer>,
    /// A vector containing the [entries](Entry) of the [`Section`](Section).
    /// If this vector is empty the section will be disabled.
    pub(crate) entries: Vec<Entry<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Section<'a, Message, Renderer> {
    /// Creates a new [`Section`](Section).
    ///
    /// It expects:
    ///     * A label that is displayed on the [`Menu`](Menu) bar.
    ///     * A vector containing the group of entries this [`Section`](Section) has.
    pub fn new<E>(label: E, entries: Vec<Entry<'a, Message, Renderer>>) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        Self {
            label: label.into(),
            entries,
        }
    }

    /// Applies a transformation to the produced message of the [`Section`](Section).
    ///
    /// Take a look into the [`Element`](iced_native::Element) documentation for
    /// more information.
    pub fn map<F, B>(self, f: F) -> Section<'a, B, Renderer>
    where
        Message: 'static,
        Renderer: 'a + iced_native::Renderer,
        B: 'static,
        F: 'static + Copy + Fn(Message) -> B,
    {
        Section::new(
            self.label.map(f),
            self.entries.into_iter().map(|e| e.map(f)).collect(),
        )
    }
}

/// An [`Entry`](Entry) of a [`Section`](Section) or `[Entry](Entry)::Group`.
#[allow(missing_debug_implementations)]
pub enum Entry<'a, Message, Renderer> {
    /// An [`Entry`] item holding an [`Element`](iced_native::Element) for it's label
    /// and a message that is send when the item is pressed.
    /// If the message is none the item will be disabled.
    Item(Element<'a, Message, Renderer>, Option<Message>),
    /// An [`Entry`] item that can be toggled.
    Toggle(
        Element<'a, Message, Renderer>,
        bool,
        Option<Box<dyn Fn(bool) -> Message + 'static>>,
    ),
    /// A group of [`Entry`](Entry)s holding an [`Element`](iced_native::Element) for
    /// it's label.
    /// If the vector is empty the group will be disabled.
    Group(
        Element<'a, Message, Renderer>,
        Vec<Entry<'a, Message, Renderer>>,
    ),
    /// A separator.
    Separator,
}

impl<'a, Message, Renderer: iced_native::Renderer> Entry<'a, Message, Renderer> {
    /// Applies a transformation to the produced message of the [`Element`](Element).
    ///
    /// Take a look into the [`Element`](iced_native::Element) documentation for
    /// more information.
    pub fn map<F, B>(self, f: F) -> Entry<'a, B, Renderer>
    where
        Message: 'static,
        Renderer: 'a,
        B: 'static,
        F: 'static + Copy + Fn(Message) -> B,
    {
        match self {
            Entry::Item(label, message) => Entry::Item(label.map(f), message.map(f)),
            Entry::Toggle(label, toggled, message) => Entry::Toggle(
                label.map(f),
                toggled,
                message.map(|m| {
                    // TODO: I can't believe that this actually works...
                    Box::new(move |b: bool| f(m(b))) as Box<dyn Fn(bool) -> B>
                }),
            ),
            Entry::Group(label, entries) => Entry::Group(
                label.map(f),
                entries.into_iter().map(|entry| entry.map(f)).collect(),
            ),
            Entry::Separator => Entry::Separator,
        }
    }
}

impl<'a, Message, Renderer> From<Menu<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + row::Renderer + crate::native::overlay::menu::Renderer,
{
    fn from(menu: Menu<'a, Message, Renderer>) -> Self {
        Element::new(menu)
    }
}

//! A menu bar.
//!
//! *This API requires the following crate features to be activated: `menu`*
use std::hash::Hash;

use iced_graphics::{Point, Size};
use iced_native::{
    event,
    layout::{self, Limits},
    mouse, overlay, touch, Clipboard, Event, Layout, Length,
};

use crate::{
    core::{
        menu::{get_entry, stack_to_path_list},
        renderer::DrawEnvironment,
    },
    menu::{Entry, State},
    native::menu::Section,
};

/// The size of the little arrow marking an `Entry::Group`.
pub const GROUP_ICON_SIZE: f32 = 16.0;
/// The size of the little checkmark icon for `Entry::Touble`.
pub const TOGGLE_ICON_SIZE: f32 = 16.0;

/// The overlay of the [`Menu`](crate::native::Menu).
#[allow(missing_debug_implementations)]
pub struct MenuOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    /// The state of the [`Menu`](crate::native::Menu).
    state: &'a mut State,
    /// The [`Section`](crate::native::menu::Section) that is displayed on this overlay.
    section: &'a Section<'a, Message, Renderer>,
    /// The padding around each entry.
    padding: f32,
    /// The position of the [`Section`](crate::native::menu::Section).
    position: Point,
    /// The style of the [`Menu`](crate::native::Menu)/[`MenuOverlay`](MenuOverlay).
    style: Renderer::Style,
}

impl<'a, Message, Renderer> MenuOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    /// Creates a new [`MenuOverlay`](MenuOverlay) on the given position, displaying
    /// the specified section.
    pub fn new(
        state: &'a mut State,
        section: &'a Section<'a, Message, Renderer>,
        position: Point,
    ) -> Self {
        Self {
            state,
            section,
            padding: 3.0,
            position,
            style: Renderer::Style::default(),
        }
    }

    /// Turn this [`MenuOverlay`](MenuOverlay) into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for MenuOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> layout::Node {
        let mut nodes = Vec::new();
        let offset_y = position.y;

        layout_entries(
            renderer,
            &self.section.entries,
            Size::new(bounds.width, bounds.height - position.y),
            self.padding,
            Positioning {
                position: Point::new(position.x, 0.0),
                fallback_offset: 0.0,
                use_fallback: false,
            },
            &self.state.stack[1..],
            &mut nodes,
        );

        nodes.reverse();

        let mut node =
            layout::Node::with_children(Size::new(bounds.width, bounds.height - position.y), nodes);

        node.move_to(Point::new(0.0, offset_y));

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if !layout
                    .children()
                    .into_iter()
                    .any(|layout| layout.bounds().contains(cursor_position))
                {
                    self.state.stack.clear();
                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        let children = layout.children();

        let path_list = stack_to_path_list(&self.state.stack);

        let mut new_path = None;

        children.zip(path_list).for_each(|(layout, path)| {
            layout.children().enumerate().for_each(|(index, layout)| {
                if layout.bounds().contains(cursor_position) {
                    // TODO: clean up
                    let mut path = path.to_vec();

                    let mut entry_path = path.to_vec();
                    entry_path.push(index);
                    let entry = get_entry(self.section, &entry_path);

                    match entry {
                        Entry::Item(_, message) => match event {
                            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                                if let Some(message) = message {
                                    messages.push(message.to_owned());
                                    path = Vec::new();
                                }
                            }
                            _ => {}
                        },
                        Entry::Toggle(_, b, message) => match event {
                            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                                if let Some(message) = message {
                                    messages.push((message)(!*b));
                                    path = Vec::new();
                                }
                            }
                            _ => {}
                        },
                        Entry::Group(_, entries) => {
                            if !entries.is_empty() {
                                path = entry_path;
                            }
                        }
                        Entry::Separator => {}
                    }
                    new_path = Some(path);
                }
            })
        });

        if let Some(new_path) = new_path {
            self.state.stack = new_path;
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        <Renderer as self::Renderer>::draw(
            renderer,
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &self.style,
                viewport: None,
                focus: (),
            },
            self.state,
            self.section,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.state.stack.hash(state);
        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
    }
}

/// The renderer of a [`MenuOverlay`](MenuOverlay).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`MenuOverlay`](MenuOverlay) in your user interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`MenuOverlay`](MenuOverlay).
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        state: &State,
        section: &Section<'_, Message, Self>,
    ) -> Self::Output;
}

impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        _state: &State,
        _section: &Section<'_, Message, Self>,
    ) -> Self::Output {
    }
}

/// Defines the layout of the entries.
fn layout_entries<'a, Message, Renderer: iced_native::Renderer>(
    renderer: &Renderer,
    entries: &[Entry<'a, Message, Renderer>],
    bounds: Size,
    padding: f32,
    positioning: Positioning,
    path: &[usize],
    nodes: &mut Vec<layout::Node>,
) {
    let entry_limits = Limits::new(Size::ZERO, bounds)
        .pad(padding)
        .width(Length::Shrink)
        .height(Length::Shrink);

    let mut entry_nodes: Vec<layout::Node> = entries
        .iter()
        .map(|entry| match entry {
            Entry::Item(element, _) | Entry::Toggle(element, _, _) | Entry::Group(element, _) => {
                element.layout(renderer, &entry_limits)
            }
            Entry::Separator => {
                // TODO
                layout::Node::default()
            }
        })
        .collect();

    entry_nodes.iter_mut().for_each(|entry| {
        entry.move_to(Point::new(
            entry.bounds().x + padding + TOGGLE_ICON_SIZE,
            entry.bounds().y + padding,
        ))
    });

    let max_width = entry_nodes
        .iter()
        .map(|entry| entry.bounds().width)
        //.max()
        // https://stackoverflow.com/a/28446718
        .fold(0.0, |a: f32, b: f32| a.max(b))
        + 2.0 * padding
        + TOGGLE_ICON_SIZE
        + GROUP_ICON_SIZE;
    let max_height = entry_nodes
        .iter()
        .map(|entry| entry.bounds().height)
        //.max()
        // https://stackoverflow.com/a/28446718
        .fold(0.0, |a: f32, b: f32| a.max(b))
        + 2.0 * padding;

    let mut entry_nodes: Vec<layout::Node> = entry_nodes
        .into_iter()
        .map(|entry| layout::Node::with_children(Size::new(max_width, max_height), vec![entry]))
        .collect();

    let mut offset = 0.0;

    entry_nodes.iter_mut().for_each(|entry| {
        entry.move_to(Point::new(entry.bounds().x, offset));
        offset += entry.bounds().height;
    });

    let mut node = layout::Node::with_children(
        Size::new(
            max_width,
            entry_nodes.iter().map(|entry| entry.bounds().height).sum(),
        ),
        entry_nodes,
    );

    node.move_to(positioning.position);
    vertical_bounce(&mut node, bounds);
    let use_fallback = horizontal_bounce(&mut node, bounds, positioning);

    if !path.is_empty() {
        if let Entry::Group(_, entries) = &entries[path[0]] {
            layout_entries(
                renderer,
                entries,
                bounds,
                padding,
                Positioning {
                    position: Point::new(
                        node.bounds().x + node.bounds().width,
                        positioning.position.y + node.children()[path[0]].bounds().y,
                    ),
                    fallback_offset: node.bounds().width,
                    use_fallback,
                },
                &path[1..],
                nodes,
            );
        }
    }

    nodes.push(node);
}

/// Lets the node vertically bounce at the given bounds.
fn vertical_bounce(node: &mut layout::Node, bounds: Size) {
    if node.bounds().y + node.bounds().height > bounds.height {
        node.move_to(Point::new(
            node.bounds().x,
            (node.bounds().y + (bounds.height - (node.bounds().y + node.bounds().height))).max(0.0),
        ))
    }
}

/// Lets the node horizontally bounce at the given bounds. The node will be positioned at the other
/// side of the parent based on the positioning.
fn horizontal_bounce(node: &mut layout::Node, bounds: Size, positioning: Positioning) -> bool {
    if node.bounds().x + node.bounds().width > bounds.width || positioning.use_fallback {
        node.move_to(Point::new(
            (node.bounds().x - node.bounds().width - positioning.fallback_offset).max(0.0),
            node.bounds().y,
        ));

        true
    } else {
        false
    }
}

/// Positioning information for a group of entries.
#[derive(Clone, Copy)]
struct Positioning {
    /// The position of the group.
    position: Point,
    /// The fallback offset if the group is bouncing horizontally.
    fallback_offset: f32,
    /// If the fallback should be used.
    use_fallback: bool,
}

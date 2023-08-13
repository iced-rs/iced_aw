//! Build and show dropdown `ListMenus`.
use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::selection_list::StyleSheet;

use iced_widget::{
    core::{
        self,
        alignment::{Horizontal, Vertical},
        event, layout,
        mouse::{self, Cursor},
        renderer, touch,
        widget::{
            tree::{State, Tag},
            Tree,
        },
        Clipboard, Color, Element, Event, Layout, Length, Rectangle, Shell, Size, Widget,
    },
    text::LineHeight,
};

/// The Private [`List`] Handles the Actual list rendering.
#[allow(missing_debug_implementations)]
pub struct List<'a, T: 'a, Message, Renderer>
where
    T: Clone + ToString + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
    Renderer: core::Renderer + core::text::Renderer<Font = core::Font>,
    Renderer::Theme: StyleSheet,
{
    /// Options pointer to hold all rendered strings
    pub options: Cow<'a, [T]>,
    /// Hovered Item Pointer
    /// Label Font
    pub font: Renderer::Font,
    /// Style for Font colors and Box hover colors.
    pub style: <Renderer::Theme as StyleSheet>::Style,
    /// Function Pointer On Select to call on Mouse button press.
    pub on_selected: Box<dyn Fn(usize, T) -> Message>,
    /// The padding Width
    pub padding: f32,
    /// The Text Size
    pub text_size: f32,
    /// Set the Selected ID manually.
    pub selected: Option<usize>,
    /// Shadow Type holder for Renderer.
    pub phantomdata: PhantomData<Renderer>,
}

/// The Private [`ListState`] Handles the State of the inner list.
#[derive(Debug, Clone, Default)]
pub struct ListState {
    /// Statehood of hovered_option
    pub hovered_option: Option<usize>,
    /// The index in the list of options of the last chosen Item Clicked for Processing
    pub last_selected_index: Option<(usize, u64)>,
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for List<'a, T, Message, Renderer>
where
    T: Clone + ToString + Eq + Hash,
    Renderer: core::Renderer + core::text::Renderer<Font = core::Font>,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> Tag {
        Tag::of::<ListState>()
    }

    fn state(&self) -> State {
        State::new(ListState::default())
    }

    fn diff(&self, state: &mut Tree) {
        let list_state = state.state.downcast_mut::<ListState>();

        if let Some(id) = self.selected {
            if let Some(option) = self.options.get(id) {
                let mut hasher = DefaultHasher::new();
                option.hash(&mut hasher);

                list_state.last_selected_index = Some((id, hasher.finish()));
            } else {
                list_state.last_selected_index = None;
            }
        } else if let Some((id, hash)) = list_state.last_selected_index {
            if let Some(option) = self.options.get(id) {
                let mut hasher = DefaultHasher::new();
                option.hash(&mut hasher);

                if hash != hasher.finish() {
                    list_state.last_selected_index = None;
                }
            } else {
                list_state.last_selected_index = None;
            }
        }
    }

    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use std::f32;
        let limits = limits.height(Length::Fill).width(Length::Fill);

        #[allow(clippy::cast_precision_loss)]
        let intrinsic = Size::new(
            limits.fill().width,
            (self.text_size + self.padding * 2.0) * self.options.len() as f32,
        );

        layout::Node::new(intrinsic)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let bounds = layout.bounds();
        let mut status = event::Status::Ignored;
        let list_state = state.state.downcast_mut::<ListState>();
        let cursor = cursor.position().unwrap_or_default();

        if bounds.contains(cursor) {
            match event {
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    list_state.hovered_option = Some(
                        ((cursor.y - bounds.y) / (self.text_size + (self.padding * 2.0))) as usize,
                    );
                }
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    list_state.hovered_option = Some(
                        ((cursor.y - bounds.y) / (self.text_size + (self.padding * 2.0))) as usize,
                    );

                    if let Some(index) = list_state.hovered_option {
                        if let Some(option) = self.options.get(index) {
                            let mut hasher = DefaultHasher::new();
                            option.hash(&mut hasher);
                            list_state.last_selected_index = Some((index, hasher.finish()));
                        }
                    }

                    status =
                        list_state
                            .last_selected_index
                            .map_or(event::Status::Ignored, |last| {
                                if let Some(option) = self.options.get(last.0) {
                                    shell.publish((self.on_selected)(last.0, option.clone()));
                                    event::Status::Captured
                                } else {
                                    event::Status::Ignored
                                }
                            });
                }
                _ => {}
            }
        }

        status
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();

        if bounds.contains(cursor.position().unwrap_or_default()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        use std::f32;

        let bounds = layout.bounds();
        let option_height = self.text_size + (self.padding * 2.0);
        let offset = viewport.y - bounds.y;
        let start = (offset / option_height) as usize;
        let end = ((offset + viewport.height) / option_height).ceil() as usize;

        let visible_options = &self.options[start..end.min(self.options.len())];
        let list_state = state.state.downcast_ref::<ListState>();

        for (i, option) in visible_options.iter().enumerate() {
            let i = start + i;
            let is_selected = list_state
                .last_selected_index
                .map(|u| u.0 == i)
                .unwrap_or_default();
            let is_hovered = list_state.hovered_option == Some(i);

            let bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + option_height * i as f32,
                width: bounds.width,
                height: self.text_size + (self.padding * 2.0),
            };

            if is_selected || is_hovered {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border_radius: (0.0).into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    if is_selected {
                        theme.style(&self.style).selected_background
                    } else {
                        theme.style(&self.style).hovered_background
                    },
                );
            }

            let text_color = if is_selected {
                theme.style(&self.style).selected_text_color
            } else if is_hovered {
                theme.style(&self.style).hovered_text_color
            } else {
                theme.style(&self.style).text_color
            };

            renderer.fill_text(core::text::Text {
                content: &option.to_string(),
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.center_y(),
                    width: f32::INFINITY,
                    ..bounds
                },
                size: self.text_size,
                color: text_color,
                font: self.font,
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::default(),
                shaping: iced_widget::text::Shaping::Advanced,
            });
        }
    }
}

impl<'a, T, Message, Renderer> From<List<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: Clone + ToString + Eq + Hash,
    Message: 'a,
    Renderer: 'a + core::Renderer + core::text::Renderer<Font = core::Font>,
    Renderer::Theme: StyleSheet,
{
    fn from(list: List<'a, T, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(list)
    }
}

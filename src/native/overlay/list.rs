//! Build and show dropdown `ListMenus`.
use crate::selection_list;
use iced_native::{
    container,
    event::{self, Event},
    layout, mouse, scrollable, text, touch, Clipboard, Element, Hasher, Layout, Length, Point,
    Rectangle, Size, Widget,
};
use std::marker::PhantomData;

/// The Private [`List`] Handles the Actual list rendering.
#[allow(missing_debug_implementations)]
pub struct List<'a, T, Message, Renderer: self::Renderer> {
    /// Options pointer to hold all rendered strings
    pub options: &'a [T],
    /// Hovered Item Pointer
    pub hovered_option: &'a mut Option<usize>,
    /// Last choosen Item Clicked for Processing
    pub last_selection: &'a mut Option<T>,
    /// Label Font
    pub font: Renderer::Font,
    /// Style for Font colors and Box hover colors.
    pub style: selection_list::Style,
    /// Function Pointer On Select to call on Mouse button press.
    pub on_selected: Box<dyn Fn(T) -> Message>,
    /// Shadow Type holder for Renderer.
    pub phantomdata: PhantomData<Renderer>,
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for List<'a, T, Message, Renderer>
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

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use std::f32;
        let limits = limits.height(Length::Fill).width(Length::Fill);

        #[allow(clippy::cast_precision_loss)]
        let intrinsic = Size::new(
            limits.fill().width,
            f32::from(self.style.text_size + (self.style.padding * 2)) * self.options.len() as f32,
        );

        layout::Node::new(intrinsic)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.options.len().hash(state);
        self.style.text_size.hash(state);
        self.style.padding.hash(state);
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
        let bounds = layout.bounds();
        let mut status = event::Status::Ignored;

        if bounds.contains(cursor_position) {
            match event {
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    *self.hovered_option = Some(
                        ((cursor_position.y - bounds.y)
                            / f32::from(self.style.text_size + (self.style.padding * 2)))
                            as usize,
                    );
                }
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    *self.hovered_option = Some(
                        ((cursor_position.y - bounds.y)
                            / f32::from(self.style.text_size + (self.style.padding * 2)))
                            as usize,
                    );

                    if let Some(index) = self.hovered_option {
                        if let Some(option) = self.options.get(*index) {
                            *self.last_selection = Some(option.clone());
                        }
                    }

                    status = self
                        .last_selection
                        .take()
                        .map_or(event::Status::Ignored, |last| {
                            messages.push((self.on_selected)(last.clone()));
                            event::Status::Captured
                        });
                }
                _ => {}
            }
        }

        status
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
            self.font,
            &self.style,
        )
    }
}

/// The renderer of a [`List`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`List`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer: scrollable::Renderer + container::Renderer + text::Renderer {
    /// Draws the list of options of a [`List`].
    #[allow(clippy::too_many_arguments)]
    fn draw<T: ToString>(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        viewport: &Rectangle,
        options: &[T],
        hovered_option: Option<usize>,
        font: Self::Font,
        style: &selection_list::Style,
    ) -> Self::Output;
}

impl<'a, T, Message, Renderer> From<List<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: ToString + Clone,
    Message: 'a,
    Renderer: 'a + self::Renderer,
{
    fn from(list: List<'a, T, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(list)
    }
}

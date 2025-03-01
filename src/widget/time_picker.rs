//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*

use super::overlay::time_picker::{self, TimePickerOverlay, TimePickerOverlayButtons};

use chrono::Local;
use iced::{
    Element,
    Event,
    Length,
    Point,
    Rectangle,
    Renderer, // the actual type
    Size,
    Vector,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        layout::{Limits, Node},
        overlay, renderer,
        widget::tree::{self, Tag, Tree},
    },
    event,
    mouse::{self, Cursor},
    widget::{button, container, text},
};

pub use crate::{
    core::time::{Period, Time},
    style::{
        Status, StyleFn,
        time_picker::{Catalog, Style},
    },
};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking times.
///
/// # Example
/// ```ignore
/// # use iced_aw::{TimePicker, time_picker};
/// # use iced_widget::{button, Button, Text};
/// #
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(time_picker::Time),
/// }
///
/// let time_picker = TimePicker::new(
///     true,
///     time_picker::Time::now_hms(true),
///     Button::new(Text::new("Pick time"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct TimePicker<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + button::Catalog,
{
    /// Show the picker.
    show_picker: bool,
    /// The time to show.
    time: Time,
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The message that is send if the cancel button of the [`TimePickerOverlay`] is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`] is pressed.
    on_submit: Box<dyn Fn(Time) -> Message>,
    /// The style of the [`TimePickerOverlay`].
    class: <Theme as Catalog>::Class<'a>,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Theme, Renderer>,
    /// Toggle the use of the 24h clock of the [`TimePickerOverlay`].
    use_24h: bool,
    /// Toggle the use of the seconds of the [`TimePickerOverlay`].
    show_seconds: bool,
}

impl<'a, Message, Theme> TimePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog,
{
    /// Creates a new [`TimePicker`] wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the time picker is visible.
    ///     * the initial time to show.
    ///     * the underlay [`Element`] on which this [`TimePicker`]
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`TimePicker`] is pressed.
    ///     * a function that will be called when the submit button of the [`TimePicker`]
    ///         is pressed, which takes the picked [`Time`] value.
    pub fn new<U, F>(
        show_picker: bool,
        time: impl Into<Time>,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            show_picker,
            time: time.into(),
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            class: <Theme as Catalog>::default(),
            overlay_state: TimePickerOverlayButtons::default().into(),
            use_24h: false,
            show_seconds: false,
        }
    }

    /// Enables the picker to also pick seconds.
    #[must_use]
    pub fn show_seconds(mut self) -> Self {
        self.show_seconds = true;
        self
    }

    /// Sets the style of the [`TimePicker`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Use 24 hour format instead of AM/PM.
    #[must_use]
    pub fn use_24h(mut self) -> Self {
        self.use_24h = true;
        self
    }

    /// Sets the class of the input of the [`TimePicker`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// The state of the [`TimePicker`] / [`TimePickerOverlay`].
#[derive(Debug)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: time_picker::State,
}

impl State {
    /// Creates a new [`State`] with the current time.
    #[must_use]
    pub fn now() -> Self {
        Self {
            overlay_state: time_picker::State::default(),
        }
    }

    /// Creates a new [`State`] with the given time.
    #[must_use]
    pub fn new(time: Time, use_24h: bool, show_seconds: bool) -> Self {
        Self {
            overlay_state: time_picker::State::new(time, use_24h, show_seconds),
        }
    }

    /// Resets the time of the state to the current time.
    pub fn reset(&mut self) {
        self.overlay_state.clock_cache.clear();
        self.overlay_state.time = Local::now().naive_local().time();
    }
}

impl<Message, Theme> Widget<Message, Theme, Renderer> for TimePicker<'_, Message, Theme>
where
    Message: 'static + Clone,
    Theme: Catalog + button::Catalog + text::Catalog + container::Catalog,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.time, self.use_24h, self.show_seconds))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay_state)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.underlay.as_widget_mut().update(
            &mut state.children[0],
            event,
            layout,
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
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
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
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let picker_state: &mut State = state.state.downcast_mut();

        if !self.show_picker {
            return self.underlay.as_widget_mut().overlay(
                &mut state.children[0],
                layout,
                renderer,
                translation,
            );
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            TimePickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.class,
                &mut state.children[1],
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Theme> From<TimePicker<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'static + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog + container::Catalog,
{
    fn from(time_picker: TimePicker<'a, Message, Theme>) -> Self {
        Element::new(time_picker)
    }
}

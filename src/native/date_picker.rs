//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use chrono::Local;
use iced_widget::{
    button, container,
    core::{
        self, event,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        renderer,
        widget::{
            self,
            tree::{Tag, Tree},
        },
        Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
    },
    renderer::Renderer,
    text,
};

pub use crate::core::date::Date;

pub use crate::style::date_picker::{Appearance, StyleSheet};

use super::overlay::date_picker::{self, DatePickerOverlay, DatePickerOverlayButtons};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking dates.
///
/// # Example
/// ```ignore
/// # use iced_aw::DatePicker;
/// # use iced::widget::{button, Button, Text};
/// #
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(date_picker::Date),
/// }
///
/// let date_picker = DatePicker::new(
///     true,
///     date_picker::Date::today(),
///     Button::new(Text::new("Pick date"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct DatePicker<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet,
{
    /// Show the picker.
    show_picker: bool,
    /// The date to show.
    date: Date,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer<Theme>>,
    /// The message that is send if the cancel button of the [`DatePickerOverlay`](DatePickerOverlay) is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`DatePickerOverlay`](DatePickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Date) -> Message>,
    /// The style of the [`DatePickerOverlay`](DatePickerOverlay).
    style: <Theme as StyleSheet>::Style,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Renderer<Theme>>,
    //button_style: <Renderer as button::Renderer>::Style, // clone not satisfied
}

impl<'a, Message, Theme> DatePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    /// Creates a new [`DatePicker`](DatePicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the date picker is visible.
    ///     * the initial date to show.
    ///     * the underlay [`Element`] on which this [`DatePicker`](DatePicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`DatePicker`](DatePicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`DatePicker`](DatePicker)
    ///         is pressed, which takes the picked [`Date`](crate::date_picker::Date) value.
    pub fn new<U, F>(
        show_picker: bool,
        date: impl Into<Date>,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer<Theme>>>,
        F: 'static + Fn(Date) -> Message,
    {
        Self {
            show_picker,
            date: date.into(),
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style: <Theme as StyleSheet>::Style::default(),
            overlay_state: DatePickerOverlayButtons::default().into(),
            //button_style: <Renderer as button::Renderer>::Style::default(),
        }
    }

    /// Sets the style of the [`DatePicker`](DatePicker).
    #[must_use]
    pub fn style(mut self, style: <Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        //self.button_style = style.into();
        self
    }
}

/// The state of the [`DatePicker`](DatePicker) / [`DatePickerOverlay`](DatePickerOverlay).
#[derive(Debug)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: date_picker::State,
}

impl State {
    /// Creates a new [`State`](State) with the current date.
    #[must_use]
    pub fn now() -> Self {
        Self {
            overlay_state: date_picker::State::default(),
        }
    }

    /// Creates a new [`State`](State) with the given date.
    #[must_use]
    pub fn new(date: Date) -> Self {
        Self {
            overlay_state: date_picker::State::new(date.into()),
        }
    }

    /// Resets the date of the state to the current date.
    pub fn reset(&mut self) {
        self.overlay_state.date = Local::now().naive_local().date();
    }
}

impl<'a, Message, Theme> Widget<Message, Renderer<Theme>> for DatePicker<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::new(self.date))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay_state)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn layout(&self, renderer: &Renderer<Theme>, limits: &Limits) -> Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer<Theme>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
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
        renderer: &Renderer<Theme>,
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
        renderer: &mut Renderer<Theme>,
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
        renderer: &Renderer<Theme>,
    ) -> Option<core::overlay::Element<'b, Message, Renderer<Theme>>> {
        let picker_state: &mut State = state.state.downcast_mut();

        if !self.show_picker {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            DatePickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                self.style.clone(),
                &mut state.children[1],
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Theme> From<DatePicker<'a, Message, Theme>>
    for Element<'a, Message, Renderer<Theme>>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    fn from(date_picker: DatePicker<'a, Message, Theme>) -> Self {
        Element::new(date_picker)
    }
}

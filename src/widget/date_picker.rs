//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use super::overlay::date_picker::{self, DatePickerOverlay, DatePickerOverlayButtons};

use chrono::Local;
use iced_core::{
    Clipboard, Element, Event, Layout, Length, Pixels, Point, Rectangle, Shell, Size, Vector,
    Widget,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    renderer,
    text::Renderer as _,
    widget::{
        self,
        tree::{Tag, Tree},
    },
};
use iced_widget::Renderer;

pub use crate::{
    core::date::Date,
    style::{Status, StyleFn, date_picker::Style},
};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking dates.
///
/// # Example
/// ```ignore
/// # use iced_aw::DatePicker;
/// # use iced_widget::{button, Button, Text};
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
    Theme: crate::style::date_picker::Catalog + iced_widget::button::Catalog,
{
    /// Show the picker.
    show_picker: bool,
    /// The date to show.
    date: Date,
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The message that is send if the cancel button of the [`DatePickerOverlay`] is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`DatePickerOverlay`] is pressed.
    on_submit: Box<dyn Fn(Date) -> Message>,
    /// The style of the [`DatePickerOverlay`].
    class: <Theme as crate::style::date_picker::Catalog>::Class<'a>,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Theme, Renderer>,
    //button_style: <Renderer as button::Renderer>::Style, // clone not satisfied
    /// The font and icon size of the [`DatePickerOverlay`] or `None` for the default
    font_size: Option<Pixels>,
}

impl<'a, Message, Theme> DatePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
{
    /// Creates a new [`DatePicker`] wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the date picker is visible.
    ///     * the initial date to show.
    ///     * the underlay [`Element`] on which this [`DatePicker`]
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`DatePicker`]
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`DatePicker`]
    ///         is pressed, which takes the picked [`Date`] value.
    pub fn new<U, F>(
        show_picker: bool,
        date: impl Into<Date>,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        F: 'static + Fn(Date) -> Message,
    {
        Self {
            show_picker,
            date: date.into(),
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            class: <Theme as crate::style::date_picker::Catalog>::default(),
            overlay_state: DatePickerOverlayButtons::default().into(),
            //button_style: <Renderer as button::Renderer>::Style::default(),
            font_size: None,
        }
    }

    /// Sets the style of the [`DatePicker`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as crate::style::date_picker::Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the font and icon size of the [`DatePicker`].
    #[must_use]
    pub fn font_size<P: Into<Pixels>>(mut self, size: P) -> Self {
        self.font_size = Some(size.into());
        self
    }

    /// Sets the class of the input of the [`DatePicker`].
    #[must_use]
    pub fn class(
        mut self,
        class: impl Into<<Theme as crate::style::date_picker::Catalog>::Class<'a>>,
    ) -> Self {
        self.class = class.into();
        self
    }
}

/// The state of the [`DatePicker`] / [`DatePickerOverlay`].
#[derive(Debug)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: date_picker::State,
}

impl State {
    /// Creates a new [`State`] with the current date.
    #[must_use]
    pub fn now() -> Self {
        Self {
            overlay_state: date_picker::State::default(),
        }
    }

    /// Creates a new [`State`] with the given date.
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

impl<Message, Theme> Widget<Message, Theme, Renderer> for DatePicker<'_, Message, Theme>
where
    Message: 'static + Clone,
    Theme: crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
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

    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
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
        self.underlay.as_widget_mut().update(
            &mut state.children[0],
            event,
            layout,
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
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<()>,
    ) {
        self.underlay.as_widget_mut().operate(
            &mut state.children[0],
            layout,
            renderer,
            operation,
        );
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
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        let picker_state: &mut State = tree.state.downcast_mut();

        if !self.show_picker {
            return self.underlay.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            DatePickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                &self.class,
                &mut tree.children[1],
                self.font_size.unwrap_or_else(|| renderer.default_size()),
                *viewport,
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Theme> From<DatePicker<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'static + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
{
    fn from(date_picker: DatePicker<'a, Message, Theme>) -> Self {
        Element::new(date_picker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[derive(Clone, Debug)]
    enum TestMessage {
        Cancel,
        Submit(Date),
    }

    type TestDatePicker<'a> = DatePicker<'a, TestMessage, iced_widget::Theme>;

    #[test]
    fn state_new_creates_state_with_date() {
        let date = Date::from_ymd(2024, 6, 15);
        let state = State::new(date);

        assert_eq!(state.overlay_state.date.year(), 2024);
        assert_eq!(state.overlay_state.date.month(), 6);
        assert_eq!(state.overlay_state.date.day(), 15);
    }

    #[test]
    fn state_now_creates_state() {
        let state = State::now();
        // Just verify it was created successfully
        assert!(state.overlay_state.date.year() > 2020);
    }

    #[test]
    fn state_reset_updates_to_current_date() {
        let old_date = Date::from_ymd(2020, 1, 1);
        let mut state = State::new(old_date);

        state.reset();

        // After reset, the date should be current (greater than 2020)
        assert!(state.overlay_state.date.year() >= 2024);
    }

    #[test]
    fn date_picker_new_creates_instance() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 6, 15);

        let date_picker = TestDatePicker::new(
            false,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(!date_picker.show_picker);
        assert_eq!(date_picker.date.year, 2024);
        assert_eq!(date_picker.date.month, 6);
        assert_eq!(date_picker.date.day, 15);
        assert!(date_picker.font_size.is_none());
    }

    #[test]
    fn date_picker_show_picker_true() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 1, 1);

        let date_picker = TestDatePicker::new(
            true,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        assert!(date_picker.show_picker);
    }

    #[test]
    fn date_picker_font_size_sets_value() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 1, 1);

        let date_picker = TestDatePicker::new(
            false,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        )
        .font_size(20.0);

        assert_eq!(date_picker.font_size, Some(Pixels(20.0)));
    }

    #[test]
    fn date_picker_font_size_from_integer() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 1, 1);

        let date_picker = TestDatePicker::new(
            false,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        )
        .font_size(16);

        assert_eq!(date_picker.font_size, Some(Pixels(16.0)));
    }

    #[test]
    fn date_picker_tag_returns_state_tag() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 1, 1);

        let date_picker = TestDatePicker::new(
            false,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let tag = Widget::<TestMessage, iced_widget::Theme, Renderer>::tag(&date_picker);
        assert_eq!(tag, Tag::of::<State>());
    }

    #[test]
    fn date_picker_has_two_children() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 1, 1);

        let date_picker = TestDatePicker::new(
            false,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let children = Widget::<TestMessage, iced_widget::Theme, Renderer>::children(&date_picker);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn date_picker_size_matches_underlay() {
        let underlay = iced_widget::text::Text::new("Pick a date");
        let date = Date::from_ymd(2024, 1, 1);

        let date_picker = TestDatePicker::new(
            false,
            date,
            underlay,
            TestMessage::Cancel,
            TestMessage::Submit,
        );

        let size = Widget::<TestMessage, iced_widget::Theme, Renderer>::size(&date_picker);
        assert_eq!(size.width, Length::Shrink);
        assert_eq!(size.height, Length::Shrink);
    }

    #[test]
    fn date_picker_different_dates() {
        let test_dates = [
            (2024, 1, 1),
            (2024, 12, 31),
            (2025, 6, 15),
            (2020, 2, 29), // Leap year
        ];

        for (year, month, day) in test_dates {
            let date = Date::from_ymd(year, month, day);
            let underlay = iced_widget::text::Text::new("Pick a date");

            let date_picker = TestDatePicker::new(
                false,
                date,
                underlay,
                TestMessage::Cancel,
                TestMessage::Submit,
            );

            assert_eq!(date_picker.date.year, year);
            assert_eq!(date_picker.date.month, month);
            assert_eq!(date_picker.date.day, day);
        }
    }

    #[test]
    fn date_from_ymd_creates_correct_date() {
        let date = Date::from_ymd(2024, 7, 20);

        assert_eq!(date.year, 2024);
        assert_eq!(date.month, 7);
        assert_eq!(date.day, 20);
    }
}

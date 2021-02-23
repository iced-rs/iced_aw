//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
use dodrio::bumpalo;
use iced_web::{Bus, Css, Element, Widget};

pub use crate::core::date::Date;
pub use crate::style::date_picker::{Style, StyleSheet};

use std::rc::Rc;

/// An input element for picking dates.
///
/// # Example
/// ```
/// # use iced_aw::{DatePicker, date_picker};
/// # use iced_web::{Button, Text, button};
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(date_picker::Date),
/// }
///
/// let mut button_state = button::State::new();
/// let mut state = date_picker::State::new();
/// state.show(true);
///
/// let date_picker = DatePicker::new(
///     &mut state,
///     Button::new(&mut button_state, Text::new("Pick date"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct DatePicker<'a, Message> {
    _state: &'a mut State,
    // https://stackoverflow.com/a/24980193
    // http://html5doctor.com/the-woes-of-date-input/
    _underlay: Element<'a, Message>,
    _on_cancel: Message,
    on_submit: Rc<dyn Fn(Date) -> Message>,
    // You cannot currently style the appearance of the date picker.
    // https://developers.google.com/web/updates/2012/08/Quick-FAQs-on-input-type-date-in-Google-Chrome
    _style: Box<dyn StyleSheet>,
}

impl<'a, Message> DatePicker<'a, Message> {
    /// Creates a new [`DatePicker`](DatePicker) wrapping around the given underlay.
    ///
    /// The underlay element will be ignored on the web, since the date input can't be
    /// customized that way.
    ///
    /// It expects:
    ///     * a mutable reference to the [`DatePicker`](DatePicker)'s [`State`](State).
    ///     * the underlay [`Element`](iced_web::Element) on which this [`DatePicker`](DatePicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`DatePicker`](DatePicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`DatePicker`](DatePicker)
    ///         is pressed, which takes the picked [`Date`](crate::date_picker::Date) value.
    pub fn new<U, F>(_state: &'a mut State, _underlay: U, _on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message>>,
        F: 'static + Fn(Date) -> Message,
    {
        Self {
            _state,
            _underlay: _underlay.into(),
            _on_cancel,
            on_submit: Rc::new(on_submit),
            _style: Default::default(),
        }
    }

    /// Sets the style of the [`DateBicker`](DatePicker).
    ///
    /// The style will be ignored on the web, since the date input can't be styled.
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self._style = style.into();
        self
    }
}

/// The state of the [`DatePicker`](DatePicker).
#[derive(Debug)]
pub struct State {
    pub(crate) show: bool,
}

impl State {
    /// Creates a new [`State`](State).
    pub fn now() -> Self {
        State { show: false }
    }

    /// Sets the visibility of the [`DatePicker`](DatePicker).
    ///
    /// Currently ignored on the web.
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// Resets the date of the state to the current date.
    ///
    /// Currently ignored on the web.
    pub fn reset(&mut self) {
        //self.date = Local::today().naive_local();
    }
}

impl<'a, Message> Widget<Message> for DatePicker<'a, Message>
where
    Message: 'static + Clone,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        _style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;
        use wasm_bindgen::JsCast;

        //let style = self.style.active();

        let on_submit = self.on_submit.clone();
        let input_event_bus = bus.clone();

        let date_picker = input(bump)
            .attr("type", "date")
            // https://www.w3schools.com/jsref/event_onchange.asp
            .on("change", move |_root, _vdom, event| {
                let date_input = match event
                    .target()
                    .and_then(|d| d.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    None => return,
                    Some(date_input) => date_input,
                };
                // ... but the parsed value is always formatted yyyy-mm-dd.
                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/date#value
                let value = date_input.value();
                let mut split = value.split("-");
                let year: i32 = split.next().unwrap().parse().unwrap();
                let month: u32 = split.next().unwrap().parse().unwrap();
                let day: u32 = split.next().unwrap().parse().unwrap();

                input_event_bus.publish(on_submit(Date::from_ymd(year, month, day)));
            })
            .finish();

        let node = label(bump).children(vec![date_picker]);

        node.finish()
    }
}

impl<'a, Message> From<DatePicker<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(date_picker: DatePicker<'a, Message>) -> Element<'a, Message> {
        Element::new(date_picker)
    }
}

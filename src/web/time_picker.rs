//! TODO
use dodrio::bumpalo;
use iced_web::{Bus, Css, Element, Widget};

pub use crate::core::time::{Period, Time};
pub use crate::style::time_picker::{Style, StyleSheet};

use std::rc::Rc;

/// TODO
#[allow(missing_debug_implementations)]
pub struct TimePicker<'a, Message> {
    _state: &'a mut State,
    // https://stackoverflow.com/a/24980193
    // http://html5doctor.com/the-woes-of-date-input/
    _underlay: Element<'a, Message>,
    _on_cancel: Message,
    on_submit: Rc<dyn Fn(Time) -> Message>,
    use_24h: bool,
    show_seconds: bool,
    // You cannot currently style the appearance of the time picker.
    // https://developers.google.com/web/updates/2012/08/Quick-FAQs-on-input-type-date-in-Google-Chrome
    _style: Box<dyn StyleSheet>,
}

impl<'a, Message> TimePicker<'a, Message> {
    /// TODO
    pub fn new<U, F>(_state: &'a mut State, _underlay: U, _on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message>>,
        F: 'static + Fn(Time) -> Message,
    {
        Self {
            _state,
            _underlay: _underlay.into(),
            _on_cancel,
            on_submit: Rc::new(on_submit),
            use_24h: false,
            show_seconds: false,
            _style: Default::default(),
        }
    }

    /// Use 24 hour format instead of AM/PM.
    pub fn use_24h(mut self) -> Self {
        self.use_24h = true;
        self
    }

    /// Enables the picker to also pick seconds.
    pub fn show_seconds(mut self) -> Self {
        self.show_seconds = true;
        self
    }

    /// Sets the style of the [`TimePicker`](TimePicker).
    ///
    /// The style will be ignored on the web, since the time input can't be styled.
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self._style = style.into();
        self
    }
}

/// The state of the [`TimePicker`](TimePicker).
#[derive(Debug)]
pub struct State {
    pub(crate) show: bool,
}

impl State {
    /// Creates a new [`State`](State).
    pub fn now() -> Self {
        State { show: false }
    }

    /// Sets the visibility of the [`TimePickerOverlay`](TimePickerOverlay).
    ///
    /// Currently ignored on the web.
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// Resets the time of the state to the current time.
    ///
    /// Currently ignored on the web.
    pub fn reset(&mut self) {
        //self.date = Local::today().naive_local();
    }
}

impl<'a, Message> Widget<Message> for TimePicker<'a, Message>
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
        let show_seconds = self.show_seconds;
        let use_24h = self.use_24h;

        // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/time#Using_the_step_attribute
        let step = if show_seconds { 1 } else { 60 };

        let time_picker = input(bump)
            .attr("type", "time")
            .attr(
                "step",
                bumpalo::format!(in bump, "{}", step).into_bump_str(),
            )
            // https://www.w3schools.com/jsref/event_onchange.asp
            .on("change", move |_root, _vdow, event| {
                let time_input = match event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    None => return,
                    Some(time_input) => time_input,
                };
                // ...is always in 24-hour format that includes leading zeros: hh:mm, ...
                // ... If the time includes seconds ..., the format is always hh:mm:ss ...
                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/time#Time_value_format
                let value = time_input.value();
                let mut split = value.split(":");
                let hour: u32 = split.next().unwrap().parse().unwrap();
                let minute: u32 = split.next().unwrap().parse().unwrap();
                let second: u32 = if show_seconds {
                    split.next().unwrap().parse().unwrap()
                } else {
                    0
                };

                let (hour, period) = if !use_24h {
                    let (hour, period) = if hour >= 12 {
                        (hour - 12, Period::Pm)
                    } else {
                        (hour, Period::Am)
                    };

                    let hour = if hour == 0 { 12 } else { hour };

                    (hour, period)
                } else {
                    (hour, Period::H24)
                };

                let time = if show_seconds {
                    Time::Hms {
                        hour,
                        minute,
                        second,
                        period,
                    }
                } else {
                    Time::Hm {
                        hour,
                        minute,
                        period,
                    }
                };

                input_event_bus.publish(on_submit(time));
            })
            .finish();

        let node = label(bump).children(vec![time_picker]);

        node.finish()
    }
}

impl<'a, Message> From<TimePicker<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(time_picker: TimePicker<'a, Message>) -> Element<'a, Message> {
        Element::new(time_picker)
    }
}

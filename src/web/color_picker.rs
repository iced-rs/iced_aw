//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: color_picker*
use dodrio::bumpalo;
use iced_web::{Bus, Color, Css, Element, Widget};

pub use crate::style::color_picker::{Style, StyleSheet};

use std::rc::Rc;

/// An input element for picking colors.
///
/// TODO: Example
#[allow(missing_debug_implementations)]
pub struct ColorPicker<'a, Message> {
    _state: &'a mut State,
    // https://stackoverflow.com/a/24980193
    // http://html5doctor.com/the-woes-of-date-input/
    _underlay: Element<'a, Message>,
    _on_cancel: Message,
    on_submit: Rc<dyn Fn(Color) -> Message>,
    // You cannot currently style the appearance of the color picker.
    // https://developers.google.com/web/updates/2012/08/Quick-FAQs-on-input-type-date-in-Google-Chrome
    _style: Box<dyn StyleSheet>,
}

impl<'a, Message> ColorPicker<'a, Message> {
    /// Creates a new [`ColorPicker`](ColorPicker) wrapping around the given underlay.
    ///
    /// The underlay element will be ignored on the web, since the color input can't be
    /// customized that way.
    pub fn new<U, F>(_state: &'a mut State, _underlay: U, _on_cancel: Message, on_submit: F) -> Self
    where
        U: Into<Element<'a, Message>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            _state,
            _underlay: _underlay.into(),
            _on_cancel,
            on_submit: Rc::new(on_submit),
            _style: Default::default(),
        }
    }

    /// Sets the style of the [`ColorPicker`](ColorPicker).
    ///
    /// The style will be ignored on the web, since the color input can't be styled.
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self._style = style.into();
        self
    }
}

/// The state of the [`ColorPicker`](ColorPicker)
#[derive(Debug)]
pub struct State {
    pub(crate) show: bool,
}

impl State {
    /// Creates a new [`State`](State).
    pub fn new() -> Self {
        State { show: false }
    }

    /// Sets the visibility of the [`ColorPickerOverlay`](ColorPickerOverlay).
    ///
    /// Currently ignored on the web.
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// Resets the color of the state to the default color.
    ///
    /// Currently ignored on the web.
    pub fn reset(&mut self) {
        // nothing
    }
}

impl<'a, Message> Widget<Message> for ColorPicker<'a, Message>
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

        let on_submit = self.on_submit.clone();
        let input_event_bus = bus.clone();

        let color_picker = input(bump)
            .attr("type", "color")
            .on("change", move |_root, _vdom, event| {
                let color_input = match event
                    .target()
                    .and_then(|c| c.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    None => return,
                    Some(color_input) => color_input,
                };

                // The value of an <input> element of type color is always a
                // DOMString which contains a 7-character string specifying an
                // RGB color in hexadecimal format.
                //
                // ... In addition, colors with an alpha channel are not supported. T.T
                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/color#value
                let value = color_input.value();
                let color = hex_to_color(&value);

                input_event_bus.publish(on_submit(color));
            })
            .finish();

        let node = label(bump).children(vec![color_picker]);

        node.finish()
    }
}

impl<'a, Message> From<ColorPicker<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(color_picker: ColorPicker<'a, Message>) -> Element<'a, Message> {
        Element::new(color_picker)
    }
}

/// Converts a hex representation of a color into a [`Color`](iced_web::Color).
fn hex_to_color(hex: &str) -> Color {
    // #RRGGBB
    let hex_char_to_value = |c: char| match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c as u8 - '0' as u8,
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' => c as u8 - 'a' as u8 + 10,
        _ => 0,
    };

    let scan = |hex: &str| {
        let mut chars = hex.chars();
        hex_char_to_value(chars.next().unwrap()) * 16 + hex_char_to_value(chars.next().unwrap())
    };

    let red = scan(&hex[1..=2]);
    let green = scan(&hex[3..=4]);
    let blue = scan(&hex[5..=6]);

    Color {
        r: red as f32 / 255.0,
        g: green as f32 / 255.0,
        b: blue as f32 / 255.0,
        a: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::hex_to_color;

    use iced_web::Color;

    #[test]
    fn hex_to_color_test() {
        let result = hex_to_color("#FF0000");
        let expected = Color::from_rgb(1.0, 0.0, 0.0);
        assert_eq!(result, expected);

        let result = hex_to_color("#00FF00");
        let expected = Color::from_rgb(0.0, 1.0, 0.0);
        assert_eq!(result, expected);

        let result = hex_to_color("#0000FF");
        let expected = Color::from_rgb(0.0, 0.0, 1.0);
        assert_eq!(result, expected);
    }
}

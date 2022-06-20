//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};

/// The appearance of a [`ColorPicker`](crate::native::ColorPicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`ColorPicker`](crate::native::ColorPicker).
    pub background: Background,

    /// The border radius of the [`ColorPicker`](crate::native::ColorPicker).
    pub border_radius: f32,

    /// The border with of the [`ColorPicker`](crate::native::ColorPicker).
    pub border_width: f32,

    /// The border color of the [`ColorPicker`](crate::native::ColorPicker).
    pub border_color: Color,

    /// The border radius of the bars of the [`ColorPicker`](crate::native::ColorPicker).
    pub bar_border_radius: f32,

    /// The border width of the bars of the [`ColorPicker`](crate::native::ColorPicker).
    pub bar_border_width: f32,

    /// The border color of the bars of the [`ColorPicker`](crate::native::ColorPicker).
    pub bar_border_color: Color,
}

/// The appearance of a [`ColorPicker`](crate::native::ColorPicker).
pub trait StyleSheet {
    /// The normal appearance of a [`ColorPicker`](crate::native::ColorPicker).
    fn active(&self) -> Style;

    /// The appearance when something is selected of the
    /// [`ColorPicker`](crate::native::ColorPicker).
    fn selected(&self) -> Style;

    /// The appearance when something is hovered of the
    /// [`ColorPicker`](crate::native::ColorPicker).
    fn hovered(&self) -> Style;

    /// The appearance when something is focused of the
    /// [`ColorPicker`](crate::native::ColorPicker).
    fn focused(&self) -> Style;
}

/// The default appearance of the [`ColorPicker`](crate::native::ColorPicker).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: Color::WHITE.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::BLACK,
        }
    }

    fn selected(&self) -> Style {
        Style { ..self.active() }
    }

    fn hovered(&self) -> Style {
        Style { ..self.active() }
    }

    fn focused(&self) -> Style {
        Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            bar_border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active()
        }
    }
}

#[allow(clippy::use_self)]
impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

#[allow(clippy::use_self)]
impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}

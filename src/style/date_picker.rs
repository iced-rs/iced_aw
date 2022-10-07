//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
use iced_style::Theme;

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`DatePicker`](crate::native::DatePicker).
    pub background: Background,

    /// The border radius of the [`DatePicker`](crate::native::DatePicker).
    pub border_radius: f32,

    /// The border with of the [`DatePicker`](crate::native::DatePicker).
    pub border_width: f32,

    /// The border color of the [`DatePicker`](crate::native::DatePicker).
    pub border_color: Color,

    /// The text color of the [`DatePicker`](crate::native::DatePicker).
    pub text_color: Color,

    /// The attenuated color of the days which are not in the selected month
    /// of the [`DatePicker`](crate::native::DatePicker).
    pub text_attenuated_color: Color,

    /// The background of the days in the calender of the
    /// [`DatePicker`](crate::native::DatePicker).
    pub day_background: Background,
}

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
pub trait StyleSheet {
    /// The style type of this stylesheet
    type Style: std::default::Default + Copy;
    /// The normal appearance of a [`DatePicker`](crate::native::DatePicker).
    fn active(&self, style: Self::Style) -> Appearance;

    /// The appearance when something is selected of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn selected(&self, style: Self::Style) -> Appearance;

    /// The appearance when something is hovered of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn hovered(&self, style: Self::Style) -> Appearance;

    /// The appearance when something is focused of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn focused(&self, style: Self::Style) -> Appearance;
}

/// The default appearance of the [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Copy, Default, Debug)]
pub struct Default;

impl StyleSheet for Theme {
    type Style = Default;

    fn active(&self, _style: Self::Style) -> Appearance {
        let palette = self.extended_palette();
        let foreground = self.palette();

        Appearance {
            background: palette.background.base.color.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: foreground.text,
            text_color: foreground.text,
            text_attenuated_color: [0.87, 0.87, 0.87].into(),
            day_background: palette.background.base.color.into(),
        }
    }

    fn selected(&self, style: Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance {
            day_background: palette.primary.strong.color.into(),
            text_color: palette.primary.strong.text,
            ..self.active(style)
        }
    }

    fn hovered(&self, style: Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance {
            day_background: palette.primary.weak.color.into(),
            text_color: palette.primary.weak.text,
            ..self.active(style)
        }
    }

    fn focused(&self, style: Self::Style) -> Appearance {
        Appearance {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active(style)
        }
    }
}

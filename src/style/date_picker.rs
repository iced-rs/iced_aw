//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use std::rc::Rc;

use iced_widget::{
    core::{Background, Color},
    style::Theme,
};

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
    type Style: Default + Clone;
    /// The normal appearance of a [`DatePicker`](crate::native::DatePicker).
    fn active(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is selected of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn selected(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is hovered of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is focused of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn focused(&self, style: &Self::Style) -> Appearance;
}

/// The default appearance of the [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum DatePickerStyle {
    #[default]
    Default,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl DatePickerStyle {
    /// Creates a custom [`DatePickerStyle`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = DatePickerStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        if let DatePickerStyle::Custom(custom) = style {
            return custom.active(self);
        }

        let palette = self.extended_palette();
        let foreground = self.palette();

        Appearance {
            background: palette.background.base.color.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: foreground.text,
            text_color: foreground.text,
            text_attenuated_color: Color {
                a: foreground.text.a * 0.5,
                ..foreground.text
            },
            day_background: palette.background.base.color.into(),
        }
    }

    fn selected(&self, style: &Self::Style) -> Appearance {
        if let DatePickerStyle::Custom(custom) = style {
            return custom.selected(self);
        }

        let palette = self.extended_palette();

        Appearance {
            day_background: palette.primary.strong.color.into(),
            text_color: palette.primary.strong.text,
            ..self.active(style)
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        if let DatePickerStyle::Custom(custom) = style {
            return custom.hovered(self);
        }

        let palette = self.extended_palette();

        Appearance {
            day_background: palette.primary.weak.color.into(),
            text_color: palette.primary.weak.text,
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        if let DatePickerStyle::Custom(custom) = style {
            return custom.focused(self);
        }

        Appearance {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active(style)
        }
    }
}

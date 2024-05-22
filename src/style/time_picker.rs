//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
#![allow(clippy::doc_markdown)]
use std::rc::Rc;

use iced::{Background, Color, Theme};

/// The appearance of a [`TimePicker`](crate::widgets::TimePicker).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`TimePicker`](crate::widgets::TimePicker).
    pub background: Background,

    /// The border radius of the [`TimePicker`](crate::widgets::TimePicker).
    pub border_radius: f32,

    /// The border width of the [`TimePicker`](crate::widgets::TimePicker).
    pub border_width: f32,

    /// The border color of the [`TimePicker`](crate::widgets::TimePicker).
    pub border_color: Color,

    /// The text color of the [`TimePicker`](crate::widgets::TimePicker).
    pub text_color: Color,

    /// The color of the clock numbers of the
    /// [`TimePicker`](crate::widgets::TimePicker).
    pub clock_number_color: Color,

    /// The background of the clock numbers of the
    /// [`TimePicker`](crate::widgets::TimePicker).
    pub clock_number_background: Color,

    /// The color of the dots on the clock of the
    /// [`TimePicker`](crate::widgets::TimePicker).
    pub clock_dots_color: Color,

    /// The color of the hands of the clock of the
    /// [`TimePicker`](crate::widgets::TimePicker).
    pub clock_hand_color: Color,

    /// The with of the hands of the clock of the
    /// [`TimePicker](crate::widgets::TimePicker).
    pub clock_hand_width: f32,
}

/// The appearance of a [`TimePicker`](crate::widgets::TimePicker).
pub trait StyleSheet {
    /// The style type of this stylesheet
    type Style: Default + Clone;
    /// The normal appearance of a [`TimePicker`](crate::widgets::TimePicker).
    fn active(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is selected of the
    /// [`TimePicker`](crate::widgets::TimePicker)
    fn selected(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is hovered of the
    /// [`TimePicker`](crate::widgets::TimePicker).
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is focused of the
    /// [`TimePicker`](crate::widgets::TimePicker).
    fn focused(&self, style: &Self::Style) -> Appearance;
}

/// The style appearance of the [`TimePicker`](crate::widgets::TimePicker)
#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum TimePickerStyle {
    #[default]
    Default,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl TimePickerStyle {
    /// Creates a custom [`TimePickerStyle`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = TimePickerStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        if let TimePickerStyle::Custom(custom) = style {
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
            clock_number_color: foreground.text,
            clock_number_background: palette.background.base.color,
            clock_dots_color: [0.87, 0.87, 0.87].into(),
            clock_hand_color: [0.87, 0.87, 0.87].into(),
            clock_hand_width: 3.0,
        }
    }

    fn selected(&self, style: &Self::Style) -> Appearance {
        if let TimePickerStyle::Custom(custom) = style {
            return custom.selected(self);
        }

        let palette = self.extended_palette();

        Appearance {
            clock_number_color: palette.primary.strong.text,
            clock_number_background: palette.primary.strong.color,
            ..self.active(style)
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        if let TimePickerStyle::Custom(custom) = style {
            return custom.hovered(self);
        }

        let palette = self.extended_palette();

        Appearance {
            clock_number_color: palette.primary.weak.text,
            clock_number_background: palette.primary.weak.color,
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        if let TimePickerStyle::Custom(custom) = style {
            return custom.focused(self);
        }

        Appearance {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active(style)
        }
    }
}

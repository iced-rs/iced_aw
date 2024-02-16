//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use std::rc::Rc;

use iced::{Background, Color, Theme};

/// The appearance of a [`ColorPicker`](crate::native::ColorPicker).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
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
    /// The style type of this stylesheet
    type Style: Default + Clone;
    /// The normal appearance of a [`ColorPicker`](crate::native::ColorPicker).
    fn active(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is selected of the
    /// [`ColorPicker`](crate::native::ColorPicker).
    fn selected(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is hovered of the
    /// [`ColorPicker`](crate::native::ColorPicker).
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// The appearance when something is focused of the
    /// [`ColorPicker`](crate::native::ColorPicker).
    fn focused(&self, style: &Self::Style) -> Appearance;
}

/// The default appearance of the [`ColorPicker`](crate::native::ColorPicker).
#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum ColorPickerStyles {
    #[default]
    Default,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl ColorPickerStyles {
    /// Creates a custom [`ColorPickerStyles`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = ColorPickerStyles;

    fn active(&self, style: &Self::Style) -> Appearance {
        if let ColorPickerStyles::Custom(custom) = style {
            return custom.active(self);
        }

        let palette = self.extended_palette();
        let foreground = self.palette();

        Appearance {
            background: palette.background.base.color.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: foreground.text,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: foreground.text,
        }
    }

    fn selected(&self, style: &Self::Style) -> Appearance {
        if let ColorPickerStyles::Custom(custom) = style {
            return custom.selected(self);
        }

        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        if let ColorPickerStyles::Custom(custom) = style {
            return custom.hovered(self);
        }

        self.active(style)
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        if let ColorPickerStyles::Custom(custom) = style {
            return custom.focused(self);
        }

        let palette = self.extended_palette();
        Appearance {
            border_color: palette.background.strong.color,
            bar_border_color: palette.background.strong.color,
            ..self.active(style)
        }
    }
}

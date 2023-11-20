//! Use a `segmented_button` as an alternative to radio button.

use iced_widget::{
    core::{Background, Color},
    style::Theme,
};
/// The appearance of a [`SegmentedButton`]
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`SegmentedButton`]
    pub background: Background,
    /// selection hightlight color
    pub selected_color: Color,

    /// The border radius of the [`SegmentedButton`]
    /// If no radius is specified the default one will be used.
    pub border_radius: Option<f32>,

    /// The border with of the [`SegmentedButton`]
    pub border_width: f32,

    /// The border color of the [`SegmentedButton`]
    pub border_color: Option<Color>,

    /// The default text color of the [`SegmentedButton`]
    pub text_color: Color,
}

/// The appearance of a [`SegmentedButton`]
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default;
    /// The normal appearance of a [`SegmentedButton`](crate::native::segmented_button::SegmentedButton).
    fn active(&self, style: &Self::Style) -> Appearance;

    /// The appearance when the [`SegmentedButton`]
    fn hovered(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        Appearance {
            background: Background::Color([0.33, 0.87, 0.33].into()),
            selected_color: Color::from_rgb(0.208, 0.576, 0.961),
            ..active
        }
    }
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: Background::Color([0.87, 0.87, 0.87].into()),
            selected_color: Color::from_rgb(
                0x5E as f32 / 255.0,
                0x7C as f32 / 255.0,
                0xE2 as f32 / 255.0,
            ),
            border_radius: None,
            border_width: 1.0,
            border_color: Some([0.8, 0.8, 0.8].into()),
            text_color: Color::BLACK,
        }
    }
}

#[derive(Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default ``SegmentedButton`` Styles
pub enum SegmentedButton {
    #[default]
    Default,
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl SegmentedButton {
    /// Creates a custom [`SegmentedButtonStyles`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Box::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = SegmentedButton;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            SegmentedButton::Default => Appearance::default(),
            SegmentedButton::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        if let SegmentedButton::Custom(custom) = style {
            return custom.hovered(self);
        }

        self.active(style)
    }
}

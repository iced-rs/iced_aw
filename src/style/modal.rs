//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use std::rc::Rc;

use iced::{Background, Color, Theme};

/// The appearance of a [`Modal`](crate::native::Modal).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`Modal`](crate::native::Modal).
    ///
    /// This is used to color the backdrop of the modal.
    pub background: Background,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: Background::Color([0.87, 0.87, 0.87, 0.30].into()),
        }
    }
}
/// The appearance of a [`Modal`](crate::native::Modal).
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default + Clone;
    /// The normal appearance of a [`Modal`](crate::native::Modal).
    fn active(&self, style: &Self::Style) -> Appearance;
}

/// The default appearance of a [`Modal`](crate::native::Modal).
#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum ModalStyles {
    #[default]
    Default,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl ModalStyles {
    /// Creates a custom [`ModalStyles`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = ModalStyles;

    fn active(&self, style: &Self::Style) -> Appearance {
        if let ModalStyles::Custom(custom) = style {
            return custom.active(self);
        }

        let palette = self.extended_palette();

        Appearance {
            background: Color {
                a: palette.background.base.color.a * 0.5,
                ..palette.background.base.color
            }
            .into(),
        }
    }
}

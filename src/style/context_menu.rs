//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use std::rc::Rc;

use iced_widget::{
    core::{Background, Color},
    style::Theme,
};

/// The appearance of a [`ContextMenu`](crate::native::ContextMenu).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`ContextMenu`](crate::native::ContextMenu).
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

/// The appearance of a [`ContextMenu`](crate::native::ContextMenu).
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default + Clone;
    /// The normal appearance of a [`ContextMenu`](crate::native::ContextMenu).
    fn active(&self, style: &Self::Style) -> Appearance;
}

/// The default appearance of a [`ContextMenu`](crate::native::ContextMenu).
#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum ContextMenuStyle {
    #[default]
    Default,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl ContextMenuStyle {
    /// Creates a custom [`ContextMenuStyle`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = ContextMenuStyle;

    fn active(&self, _style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance {
            background: Color {
                a: 0f32,
                ..palette.background.base.color
            }
            .into(),
        }
    }
}

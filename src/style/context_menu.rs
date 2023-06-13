//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use iced_graphics::Color;
use iced_native::Background;
use iced_style::Theme;

/// The appearance of a [`ContextMenu`](crate::native::ContextMenu).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The backgronud of the [`ContextMenu`](crate::native::ContextMenu).
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
    type Style: Default + Copy;
    /// The normal appearance of a [`ContextMenu`](crate::native::ContextMenu).
    fn active(&self, style: Self::Style) -> Appearance;
}

/// The default appearance of a [`ContextMenu`](crate::native::ContextMenu).
#[derive(Clone, Copy, Debug, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum ContextMenuStyle {
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = ContextMenuStyle;

    fn active(&self, _style: Self::Style) -> Appearance {
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

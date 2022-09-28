//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*

#[cfg(not(target_arch = "wasm32"))]
use iced_native::Background;

/// The appearance of a [`Modal`](crate::native::Modal).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The backgronud of the [`Modal`](crate::native::Modal).
    ///
    /// This is used to color the backdrop of the modal.
    pub background: Background,
}

/// The appearance of a [`Modal`](crate::native::Modal).
pub trait StyleSheet {
    type Appearance: std::default::Default + Copy;
    /// The normal appearance of a [`Modal`](crate::native::Modal).
    fn active(&self) -> Appearance;
}

/// The default appearance of a [`Modal`](crate::native::Modal).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Appearance {
        Appearance {
            background: Background::Color([0.87, 0.87, 0.87, 0.30].into()),
        }
    }
}

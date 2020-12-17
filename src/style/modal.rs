//! Use a badge for color highlighting important information.
//! 
//! *This API requires the following crate features to be activated: badge*

#[cfg(not(target_arch = "wasm32"))]
use iced_native::Background;
#[cfg(target_arch = "wasm32")]
use iced_web::{Background};

/// The appearance of a [`ModalOverlay`](crate::native::modal::ModalOverlay).
#[allow(missing_debug_implementations)]
pub struct Style {
    /// The backgronud of the [`ModalOverlay`](crate::native::modal::ModalOverlay).
    /// 
    /// This is used to color the backdrop of the modal.
    pub background: Background,
}

/// The appearance of a [`ModalOverlay`](crate::native::modal::ModalOverlay).
pub trait StyleSheet {
    /// The normal appearance of a [`ModalOverlay`](crate::native::modal::ModalOverlay).
    fn active(&self) -> Style;
}

/// The default appearance of a [`ModalOverlay`](crate::native::modal::ModalOverlay).
#[derive(Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: Background::Color([0.87, 0.87, 0.87, 0.30].into()),
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where 
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
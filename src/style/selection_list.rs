//! Selection List
//!
//! *This API requires the following crate features to be activated: `selection_list`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color, Length};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color, Length};

/// The appearance of a menu.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The List Label Text Color
    pub text_color: Color,
    /// The background
    pub background: Background,
    /// The container Border width
    pub border_width: f32,
    /// The container Border color
    pub border_color: Color,
    /// The List Label Text Select Color
    pub selected_text_color: Color,
    /// The List Label Text Select Background Color
    pub selected_background: Background,
    /// The Containers Width
    pub width: Length,
    /// The Containers height
    pub height: Length,
    /// The padding Width
    pub padding: u16,
    /// The Text Size
    pub text_size: u16,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            text_color: Color::BLACK,
            background: Background::Color([0.87, 0.87, 0.87].into()),
            border_width: 1.0,
            border_color: [0.7, 0.7, 0.7].into(),
            selected_text_color: Color::WHITE,
            selected_background: Background::Color([0.4, 0.4, 1.0].into()),
            width: Length::Fill,
            height: Length::Fill,
            padding: 5,
            text_size: 12,
        }
    }
}

/// A set of rules that dictate the style of a container.
pub trait StyleSheet {
    /// Produces the style of a container.
    fn style() -> Style;
}

/// default Style holder.
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn style() -> Style {
        Style::default()
    }
}

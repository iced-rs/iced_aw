//! Text widget for rendering icons.
//!
//! Nearly a complete copy of the `iced_native::Text` widget, but with the
//! icons font as a default font. Maybe I'll find a better way in the future.
//!
//! //! *This API requires the following crate features to be activated: `icon_text`*
use iced_graphics::Renderer;

/// Text widget with icon font.
///
/// This is an alias of an `iced_native` `IconText` with an `iced_wgpu::Renderer`.
pub type IconText<Backend, Theme> = crate::native::icon_text::IconText<Renderer<Backend, Theme>>;

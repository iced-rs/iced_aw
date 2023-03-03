//! Use a spinner to suggest to the user something is loading.
//!
//! *This API requires the following crate features to be activated: spinner*
use crate::native::spinner;
use iced_graphics::Renderer;

/// This is an alias of an `iced_native` Spinner with an `iced_wgpu::Renderer`.
pub type Spinner<Backend, Theme> = spinner::Spinner<Renderer<Backend, Theme>>;

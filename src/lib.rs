//! Additional widgets for the Iced GUI library.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]

#[cfg(not(target_arch = "wasm32"))]
pub mod graphics;
#[cfg(not(target_arch = "wasm32"))]
pub mod native;

pub mod style;

#[cfg(not(target_arch = "wasm32"))]
mod platform {

    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use {
        badge::Badge,
        crate::graphics::badge,
    };

    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use {
        tab_bar::{
            TabBar, TabLabel,
        },
        crate::graphics::tab_bar,
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use {
        tabs::{
            Tabs, TabBarPosition,
        },
        crate::graphics::tabs,
    };
}
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use crate::web::{
        badge, badge::Badge
    };
}

#[doc(no_inline)]
pub use platform::*;
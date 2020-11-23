//! Additional widgets for the Iced GUI library.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]

pub mod graphics;
pub mod native;
pub mod style;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use crate::graphics::{
        tab_bar,
    };    

    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use {
        tab_bar::{TabBar, TabLabel},
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use crate::graphics::{
        tabs,
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use {
        tabs::{Tabs, TabBarPosition},
    };
}

#[doc(no_inline)]
pub use platform::*;
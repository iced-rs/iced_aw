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
pub mod core;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[doc(no_inline)]
    #[cfg(feature = "icons")]
    pub use {
        crate::graphics::icons::ICON_FONT,
        crate::graphics::icons::Icon,
    };

    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use {
        badge::Badge,
        crate::graphics::badge,
    };

    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use {
        card::Card,
        crate::graphics::card,
    };
    
    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use {
        date_picker::DatePicker,
        crate::graphics::date_picker,
    };

    #[doc(no_inline)]
    #[cfg(feature = "floating_button")]
    pub use {
        floating_button::FloatingButton,
        crate::graphics::floating_button,
    };
    
    /*#[doc(no_inline)]
    #[cfg(feature = "icon_text")]
    use {
        icon_text::IconText,
        crate::graphics::icon_text,
    };*/

    #[doc(no_inline)]
    #[cfg(feature = "modal")]
    pub use {
        modal::Modal,
        crate::graphics::modal,
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
    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use crate::web::{
        badge, badge::Badge,
    };
    
    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use crate::web::{
        card, card::Card,
    };
    
    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use crate::web::{
        date_picker, date_picker::DatePicker,
    };

    #[doc(no_inline)]
    #[cfg(feature = "floating_button")]
    pub use crate::web::{
        floating_button, floating_button::FloatingButton,
    };
    
    #[doc(no_inline)]
    #[cfg(feature = "modal")]
    pub use crate::web::{
        modal, modal::Modal,
    };
}

#[doc(no_inline)]
pub use platform::*;
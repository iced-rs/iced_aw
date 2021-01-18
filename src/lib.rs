//! Additional widgets for the Iced GUI library.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]

#[cfg(not(target_arch = "wasm32"))]
pub mod graphics;
#[cfg(not(target_arch = "wasm32"))]
pub mod native;

pub mod core;
pub mod style;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[doc(no_inline)]
    #[cfg(feature = "icons")]
    pub use {crate::graphics::icons::Icon, crate::graphics::icons::ICON_FONT};

    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use {crate::graphics::badge, badge::Badge};

    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use {crate::graphics::card, card::Card};

    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use {crate::graphics::date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(feature = "floating_button")]
    pub use {crate::graphics::floating_button, floating_button::FloatingButton};

    /*#[doc(no_inline)]
    #[cfg(feature = "icon_text")]
    use {
        icon_text::IconText,
        crate::graphics::icon_text,
    };*/

    #[doc(no_inline)]
    #[cfg(feature = "modal")]
    pub use {crate::graphics::modal, modal::Modal};

    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use {
        crate::graphics::tab_bar,
        tab_bar::{TabBar, TabLabel},
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use {
        crate::graphics::tabs,
        tabs::{TabBarPosition, Tabs},
    };

    #[doc(no_inline)]
    #[cfg(feature = "time_picker")]
    pub use {crate::graphics::time_picker, time_picker::TimePicker};
}
#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(target_arch = "wasm32")]
mod platform {
    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use crate::web::{badge, badge::Badge};

    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use crate::web::{card, card::Card};

    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use crate::web::{date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(feature = "floating_button")]
    pub use crate::web::{floating_button, floating_button::FloatingButton};

    #[doc(no_inline)]
    #[cfg(feature = "modal")]
    pub use crate::web::{modal, modal::Modal};

    #[doc(no_inline)]
    #[cfg(feature = "time_picker")]
    pub use crate::web::{time_picker, time_picker::TimePicker};
}

#[doc(no_inline)]
pub use platform::*;

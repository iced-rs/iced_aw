//! Additional widgets for the Iced GUI library.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,

    // Restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
)]
#![allow(
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::borrowed_box,
    clippy::missing_const_for_fn,
    clippy::too_many_lines
)]

#[cfg(not(target_arch = "wasm32"))]
pub mod graphics;
#[cfg(not(target_arch = "wasm32"))]
pub mod native;

pub mod core;
pub mod style;

#[cfg(not(target_arch = "wasm32"))]
/// Exports for all platforms that are not WASM32.
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
    #[cfg(feature = "color_picker")]
    pub use {crate::graphics::color_picker, color_picker::ColorPicker};

    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use {crate::graphics::date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(feature = "floating_element")]
    pub use {crate::graphics::floating_element, floating_element::FloatingElement};

    #[doc(no_inline)]
    #[cfg(feature = "grid")]
    pub use {crate::graphics::grid, grid::Grid};

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

    #[doc(no_inline)]
    #[cfg(feature = "wrap")]
    pub use {crate::graphics::wrap, wrap::Wrap};

    #[doc(no_inline)]
    #[cfg(feature = "number_input")]
    pub use {crate::graphics::number_input, number_input::NumberInput};

    #[doc(no_inline)]
    #[cfg(feature = "selection_list")]
    pub use {crate::graphics::selection_list, selection_list::SelectionList};

    #[doc(no_inline)]
    #[cfg(feature = "split")]
    pub use {crate::graphics::split, split::Split};
}

#[doc(no_inline)]
pub use platform::*;

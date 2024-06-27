//! Additional widgets for the Iced GUI library.
#![deny(missing_docs)]
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
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::missing_docs_in_private_items,
    clippy::unit_arg,
    clippy::trivially_copy_pass_by_ref,
    clippy::let_unit_value
)]

pub mod widgets;

pub mod core;
pub mod style;

pub use iced::Element;

/// Exports for all platforms that are not WASM32.
mod platform {
    #[allow(unused_imports)]
    pub use crate::style;
    pub use crate::widgets::helpers;

    cfg_if::cfg_if! {
        if #[cfg(feature = "icons")] {
            pub use
                crate::core::icons::{
                    Bootstrap, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES, Nerd, NERD_FONT, NERD_FONT_BYTES, bootstrap, nerd
                };
        } else {
            pub use crate::core::icons::{Bootstrap, BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES, bootstrap};
        }
    }

    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use {crate::widgets::badge, badge::Badge};

    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use {crate::widgets::card, card::Card};

    #[doc(no_inline)]
    #[cfg(feature = "color_picker")]
    pub use {crate::widgets::color_picker, color_picker::ColorPicker};

    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use {crate::widgets::date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(feature = "grid")]
    pub use crate::widgets::grid::{Grid, GridRow};

    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use {
        crate::widgets::tab_bar,
        tab_bar::{TabBar, TabLabel},
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use {
        crate::widgets::tabs,
        tabs::{TabBarPosition, Tabs},
    };

    #[doc(no_inline)]
    #[cfg(feature = "time_picker")]
    pub use {crate::widgets::time_picker, time_picker::TimePicker};

    #[doc(no_inline)]
    #[cfg(feature = "wrap")]
    pub use {crate::widgets::wrap, wrap::direction, wrap::Wrap};

    #[doc(no_inline)]
    #[cfg(feature = "number_input")]
    pub use {crate::widgets::number_input, number_input::NumberInput};

    #[doc(no_inline)]
    #[cfg(feature = "selection_list")]
    pub use {crate::widgets::selection_list, selection_list::SelectionList};

    #[doc(no_inline)]
    #[cfg(feature = "menu")]
    pub use crate::widgets::menu;

    #[doc(no_inline)]
    #[cfg(feature = "quad")]
    pub use crate::widgets::quad;

    #[doc(no_inline)]
    #[cfg(feature = "spinner")]
    pub use {crate::widgets::spinner, spinner::Spinner};

    #[doc(no_inline)]
    #[cfg(feature = "slide_bar")]
    pub use crate::widgets::SlideBar;

    #[doc(no_inline)]
    #[cfg(feature = "context_menu")]
    pub use {crate::widgets::context_menu, context_menu::ContextMenu};

    #[doc(no_inline)]
    #[cfg(feature = "drop_down")]
    pub use {crate::widgets::drop_down, drop_down::DropDown};
}

#[doc(no_inline)]
pub use platform::*;

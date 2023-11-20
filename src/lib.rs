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
    clippy::missing_docs_in_private_items
)]

pub mod graphics;

pub mod native;

pub mod core;
pub mod style;

pub use iced_widget::core::Element;
use iced_widget::{renderer, style as iced_style};

/// Exports for all platforms that are not WASM32.
mod platform {
    #[doc(no_inline)]
    #[cfg(feature = "icons")]
    pub use {crate::graphics::icons::Icon, crate::graphics::icons::ICON_FONT};

    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use {crate::native::badge, crate::style::BadgeStyles, badge::Badge};

    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use {crate::native::card, crate::style::CardStyles, card::Card};

    #[doc(no_inline)]
    #[cfg(feature = "color_picker")]
    pub use {crate::native::color_picker, color_picker::ColorPicker};

    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use {crate::native::date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(feature = "floating_element")]
    pub use {crate::native::floating_element, floating_element::FloatingElement};

    #[doc(no_inline)]
    #[cfg(feature = "grid")]
    pub use crate::native::grid::{Grid, GridRow};

    #[doc(no_inline)]
    #[cfg(feature = "modal")]
    pub use {crate::native::modal, crate::style::ModalStyles, modal::Modal};

    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use {
        crate::native::tab_bar,
        crate::style::TabBarStyles,
        tab_bar::{TabBar, TabLabel},
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use {
        crate::native::tabs,
        tabs::{TabBarPosition, Tabs},
    };

    #[doc(no_inline)]
    #[cfg(feature = "time_picker")]
    pub use {crate::native::time_picker, time_picker::TimePicker};

    #[doc(no_inline)]
    #[cfg(feature = "wrap")]
    pub use {crate::native::wrap, wrap::direction, wrap::Wrap};

    #[doc(no_inline)]
    #[cfg(feature = "number_input")]
    pub use {
        crate::native::number_input, crate::style::NumberInputStyles, number_input::NumberInput,
    };

    #[doc(no_inline)]
    #[cfg(feature = "selection_list")]
    pub use {
        crate::native::selection_list, crate::style::SelectionListStyles,
        selection_list::SelectionList,
    };

    #[doc(no_inline)]
    #[cfg(feature = "split")]
    pub use {crate::native::split, crate::style::SplitStyles, split::Split};

    #[doc(no_inline)]
    #[cfg(feature = "menu")]
    pub use {
        crate::native::menu,
        crate::native::menu::{
            CloseCondition, ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight,
        },
    };

    #[doc(no_inline)]
    #[cfg(feature = "quad")]
    pub use crate::native::quad;

    pub use crate::native::helpers;
    #[doc(no_inline)]
    #[cfg(feature = "spinner")]
    pub use {crate::native::spinner, crate::style::SpinnerStyle, spinner::Spinner};

    pub use crate::native::SlideBar;

    #[doc(no_inline)]
    #[cfg(feature = "context_menu")]
    pub use {
        crate::native::context_menu, crate::style::ContextMenuStyle, context_menu::ContextMenu,
    };
}

#[doc(no_inline)]
pub use platform::*;

#[allow(dead_code)]
type Renderer<Theme = iced_style::Theme> = renderer::Renderer<Theme>;

//! Additional widget for the Iced GUI library.
//!
//! # Examples
//!
//! * `badge` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `card` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `color_picker` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `context_menu` (Author: wiiznokes <wiiznokes2@gmail.com>)
//! * `date_picker` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `drop_down` (Author: wiiznokes <wiiznokes2@gmail.com>)
//! * `menu`
//! * `number_input` (Author: leang27 <52003343+leang27@users.noreply.github.com>)
//! * `selection_list` (Author: Héctor Ramón Jiménez <hector0193@gmail.com> and Andrew Wheeler <genusistimelord@gmail.com>)
//! * `side_bar` (Author: Kaiden42 <gitlab@tinysn.com> and Rizzen Yazston)
//! * `slide_bar` (Author: Andrew Wheeler <genusistimelord@gmail.com>)
//! * `spinner` (Author: Iohann Rabeson <irabeson42@gmail.com>)
//! * `tab_bar` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `tabs` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `time_picker` (Author: Kaiden42 <gitlab@tinysn.com>)
//! * `typed_input` (Author: Ultraxime <36888699+Ultraxime@users.noreply.github.com>)
//! * `widget_id_return` (Author: Andrew Wheeler <genusistimelord@gmail.com>)
//! * `wrap` (Author: owntime <yrainbxqc@gmail.com>)
//! * `labeled_frame` (Author: JL710)
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

pub mod widget;
#[deprecated(since = "0.9.4", note = "use `widget` instead")]
pub use widget as widgets;

pub mod core;
pub mod style;
pub use iced_fonts;

/// Exports for all platforms that are not WASM32.
mod platform {
    #[allow(unused_imports)]
    pub use crate::style;
    pub use crate::widget::helpers;

    #[doc(no_inline)]
    #[cfg(feature = "badge")]
    pub use {crate::widget::badge, badge::Badge};

    #[doc(no_inline)]
    #[cfg(feature = "card")]
    pub use {crate::widget::card, card::Card};

    #[doc(no_inline)]
    #[cfg(feature = "color_picker")]
    pub use {crate::widget::color_picker, color_picker::ColorPicker};

    #[doc(no_inline)]
    #[cfg(feature = "date_picker")]
    pub use {crate::widget::date_picker, date_picker::DatePicker};

    #[doc(no_inline)]
    #[cfg(feature = "tab_bar")]
    pub use {
        crate::widget::tab_bar,
        tab_bar::{TabBar, TabLabel},
    };

    #[doc(no_inline)]
    #[cfg(feature = "tabs")]
    pub use {
        crate::widget::tabs,
        tabs::{TabBarPosition, Tabs},
    };

    #[doc(no_inline)]
    #[cfg(feature = "time_picker")]
    pub use {crate::widget::time_picker, time_picker::TimePicker};

    #[doc(no_inline)]
    #[cfg(feature = "wrap")]
    pub use {crate::widget::wrap, wrap::Wrap, wrap::direction};

    #[doc(no_inline)]
    #[cfg(feature = "number_input")]
    pub use {crate::widget::number_input, number_input::NumberInput};

    #[doc(no_inline)]
    #[cfg(feature = "typed_input")]
    pub use {crate::widget::typed_input, typed_input::TypedInput};

    #[doc(no_inline)]
    #[cfg(feature = "selection_list")]
    pub use {crate::widget::selection_list, selection_list::SelectionList};

    #[doc(no_inline)]
    #[cfg(feature = "menu")]
    pub use {crate::widget::menu, menu::Menu, menu::MenuBar};

    #[doc(no_inline)]
    #[cfg(feature = "quad")]
    pub use {crate::widget::quad, quad::Quad};

    #[doc(no_inline)]
    #[cfg(feature = "spinner")]
    pub use {crate::widget::spinner, spinner::Spinner};

    #[doc(no_inline)]
    #[cfg(feature = "slide_bar")]
    pub use crate::widget::SlideBar;

    #[doc(no_inline)]
    #[cfg(feature = "context_menu")]
    pub use {crate::widget::context_menu, context_menu::ContextMenu};

    #[doc(no_inline)]
    #[cfg(feature = "drop_down")]
    pub use {crate::widget::drop_down, drop_down::DropDown};

    #[doc(no_inline)]
    #[cfg(feature = "sidebar")]
    pub use crate::widget::sidebar;
}

#[doc(no_inline)]
pub use platform::*;

use iced_core::Font;
use iced_fonts::generate_icon_functions;

/// Embedded font file. There a handfull of glyphs so no need to worry.
pub const ICED_AW_FONT_BYTES: &[u8] = include_bytes!("../font.ttf");
/// Font type to use in text widgets.
pub const ICED_AW_FONT: Font = Font::with_name("iced_aw");
generate_icon_functions!("font.ttf", iced_aw_font, ICED_AW_FONT);

//! Stateless, pure widgets for iced
//use iced_widget::{renderer, style};
pub mod helpers;
pub use helpers::*;

pub mod overlay;

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
/// A badge for color highlighting small information.
pub type Badge<'a, Message, Renderer> = badge::Badge<'a, Message, Renderer>;

#[cfg(feature = "number_input")]
pub mod number_input;
#[cfg(feature = "number_input")]
/// A field that can only be filled with numeric type.
pub type NumberInput<'a, T, Message, Renderer> =
    number_input::NumberInput<'a, T, Message, Renderer>;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
/// A card consisting of a head, body and optional foot.
pub type Card<'a, Message, Renderer> = card::Card<'a, Message, Renderer>;

#[cfg(feature = "color_picker")]
pub mod color_picker;
#[cfg(feature = "color_picker")]
pub use color_picker::ColorPicker;

#[cfg(feature = "cupertino")]
/// Cupertino-style widgets
pub mod cupertino;
#[cfg(feature = "cupertino")]
pub use crate::native::cupertino::cupertino_spinner::CupertinoSpinner;

#[cfg(feature = "date_picker")]
pub mod date_picker;
#[cfg(feature = "date_picker")]
pub use date_picker::DatePicker;

#[cfg(feature = "selection_list")]
pub mod selection_list;
#[cfg(feature = "selection_list")]
/// A widget for selecting a single value from a dynamic scrollable list of options.
pub type SelectionList<'a, T, Message, Backend, Theme> =
    selection_list::SelectionList<'a, T, Message, Renderer<Backend, Theme>>;
#[cfg(feature = "selection_list")]
pub use selection_list::List;

#[cfg(feature = "floating_element")]
pub mod floating_element;
#[cfg(feature = "floating_element")]
/// A floating element floating over some content.
pub type FloatingElement<'a, B, Message, Renderer> =
    floating_element::FloatingElement<'a, B, Message, Renderer>;

#[cfg(feature = "grid")]
pub mod grid;
#[cfg(feature = "grid")]
/// A container that distributes its contents in a grid.
pub type Grid<'a, Message, Renderer> = grid::Grid<'a, Message, Renderer>;
#[cfg(feature = "grid")]
pub use grid::Strategy;

#[cfg(feature = "icon_text")]
pub mod icon_text;
#[cfg(feature = "icon_text")]
/// Text widget with icon font.
pub type IconText<Renderer> = crate::native::icon_text::IconText<Renderer>;

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
/// A modal content as an overlay.
pub type Modal<'a, Content, Message, Renderer> = modal::Modal<'a, Content, Message, Renderer>;

#[cfg(feature = "split")]
pub mod split;
#[cfg(feature = "split")]
/// A split can divide the available space by half to display two different elements.
pub type Split<'a, Message, Renderer> = split::Split<'a, Message, Renderer>;

#[cfg(feature = "tab_bar")]
pub mod tab_bar;
#[cfg(feature = "tab_bar")]
/// A tab bar to show tabs.
pub type TabBar<Message, TabId, Backend, Theme> =
    tab_bar::TabBar<Message, TabId, Renderer<Backend, Theme>>;

#[cfg(feature = "tab_bar")]
pub use tab_bar::TabLabel;

#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "tabs")]
/// A [`Tabs`](Tabs) widget for showing a [`TabBar`](super::tab_bar::TabBar)
pub type Tabs<'a, Message, TabId, Backend, Theme> =
    tabs::Tabs<'a, Message, TabId, Renderer<Backend, Theme>>;

#[cfg(feature = "tabs")]
pub use tabs::TabBarPosition;

#[cfg(feature = "time_picker")]
pub mod time_picker;
#[cfg(feature = "time_picker")]
pub use time_picker::TimePicker;

#[cfg(feature = "wrap")]
pub mod wrap;
#[cfg(feature = "wrap")]
/// A container that distributes its contents horizontally.
pub type Wrap<'a, B, Message, Direction> = wrap::Wrap<'a, B, Message, Direction>;

#[cfg(feature = "menu")]
pub mod menu;

#[cfg(feature = "quad")]
pub mod quad;

#[cfg(feature = "spinner")]
pub mod spinner;

#[cfg(feature = "spinner")]
/// A spinner widget, a circle spinning around the center of the widget.
pub type Spinner<Backend, Theme> = spinner::Spinner<Renderer<Backend, Theme>>;

#[cfg(feature = "context_menu")]
pub mod context_menu;
#[cfg(feature = "context_menu")]
/// A context menu
pub type ContextMenu<'a, Overlay, Message, Backend, Theme> =
    context_menu::ContextMenu<'a, Overlay, Message, Renderer<Backend, Theme>>;

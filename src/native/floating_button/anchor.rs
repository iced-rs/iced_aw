//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*

/// Positional [`Anchor`](Anchor) for the [`FloatingButton`](super::FloatingButton).
#[derive(Copy, Clone, Debug, Hash)]
pub enum Anchor {
    /// NortWest [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the top left of the
    /// underlying element.
    NorthWest,

    /// NorthEast [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the top right of the
    /// underlying element.
    NorthEast,

    /// SouthWest [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the bottom left of the
    /// underlying element.
    SouthWest,

    /// SouthEast [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the bottom right of the
    /// underlying element.
    SouthEast,
}

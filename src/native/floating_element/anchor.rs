//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*

/// Positional [`Anchor`](Anchor) for the [`FloatingButton`](super::FloatingButton).
#[derive(Copy, Clone, Debug, Hash)]
pub enum Anchor {
    /// NorthWest [`Anchor`](Anchor) for positioning the
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

    /// North [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the top of the
    /// underlying element.
    North,

    /// East [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the right of the
    /// underlying element.
    East,

    /// South [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the bottom of the
    /// underlying element.
    South,

    /// West [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_native::widget::button::Button) on the left of the
    /// underlying element.
    West,
}

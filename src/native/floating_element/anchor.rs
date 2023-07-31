//! Use a floating element to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

/// Positional [`Anchor`](Anchor) for the [`FloatingElement`](super::FloatingElement).
#[derive(Copy, Clone, Debug, Hash)]
pub enum Anchor {
    /// NorthWest [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the top left of the underlying element.
    NorthWest,

    /// NorthEast [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the top right of the underlying element.
    NorthEast,

    /// SouthWest [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the bottom left of the underlying element.
    SouthWest,

    /// SouthEast [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the bottom right of the underlying element.
    SouthEast,

    /// North [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the top of the underlying element.
    North,

    /// East [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the right of the underlying element.
    East,

    /// South [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the bottom of the underlying element.
    South,

    /// West [`Anchor`](Anchor) for positioning the
    /// [`Button`](iced_widget::Button) on the left of the underlying element.
    West,
}

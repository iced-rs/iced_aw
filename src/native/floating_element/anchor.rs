//! Use a floating element to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

/// Positional [`Anchor`] for the [`FloatingElement`](super::FloatingElement).
#[derive(Copy, Clone, Debug, Hash)]
pub enum Anchor {
    /// NorthWest [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    /// on the top left of the underlying element.
    NorthWest,

    /// NorthEast [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the top right of the underlying element.
    NorthEast,

    /// SouthWest [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the bottom left of the underlying element.
    SouthWest,

    /// SouthEast [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the bottom right of the underlying element.
    SouthEast,

    /// North [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the top of the underlying element.
    North,

    /// East [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the right of the underlying element.
    East,

    /// South [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the bottom of the underlying element.
    South,

    /// West [`Anchor`] for positioning the [`Button`](iced_widget::Button)
    ///  on the left of the underlying element.
    West,
}

//! Alignment enum, similar to an anchor

/// ```text
/// +-----------+-----------+-----------+
/// | TopStart  |   Top     |  TopEnd   |
/// +-----------+-----------+-----------+
/// |  Start    |           |   End     |
/// +-----------+-----------+-----------+
/// |BottomStart|  Bottom   | BottomEnd |
/// +-----------+-----------+-----------+
/// ```

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    TopStart,
    Top,
    TopEnd,

    End,

    BottomEnd,
    Bottom,
    BottomStart,

    Start,
}

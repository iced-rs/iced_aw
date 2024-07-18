//! Contains the sidebar related widgets and data enums.

#[allow(clippy::module_inception)]
pub mod sidebar;
pub use sidebar::*;
pub mod column;
pub use column::*;

// Not used by `Sidebar` itself, but included for completeness.
// The horizontal version of the vertical `Column` for the `Row`.
pub mod row;
pub use row::*;

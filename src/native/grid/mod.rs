//! Displays a [`Grid`].
//!
//! *This API requires the following crate features to be activated: grid*

mod grid;
mod row;

pub use grid::{Grid, Strategy};
pub use row::GridRow;

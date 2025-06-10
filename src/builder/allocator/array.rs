//! Defines a cell that contains an array of cells.

use super::{cell::IntoCell, core::AllocatingBuilder};

/// An array of cells.
pub struct CellArray<'a, const N: usize, const U: usize, T: IntoCell<'a, N>> {
    memory: &'a AllocatingBuilder<N>,
    data: [T; U],
}

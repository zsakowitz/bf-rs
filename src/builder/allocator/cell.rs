//! Defines a trait from which arbitrary cells can be constructed.

use super::core::AllocatingBuilder;

/// A trait that allows arbitrary values to be converted into cells.
pub trait IntoCell<'a, const N: usize>: Sized {
    /// The output cell type, e.g. `CellU8` for a `u8`.
    type Output;

    /// Converts the value into a cell given an allocator.
    fn into_cell(self, memory: &'a AllocatingBuilder<N>) -> Self::Output;
}

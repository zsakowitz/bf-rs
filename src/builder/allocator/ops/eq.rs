//! Provides traits for equality comparisons that evaluate to `CellBool`s rather than `bool`s.

use crate::builder::allocator::bool::CellBool;

/// Trait for partial equality comparisons between two values. The left hand side and right hand
/// side do not need to be of the same type, which is useful for comparing a cell with a value. For
/// example, `CellU8` implements `PartialEq<u8>`, `PartialEq<&CellU8>`, and `PartialEq<CellU8>`,
/// allowing it to be compared to any of these.
///
/// These traits output a `CellBool` value. This allows their results to be computed and used at
/// runtime. If compile-time checking is necessary, consider refactoring your code or using regular
/// equality checks instead.
pub trait PartialEq<'a, const N: usize, Rhs = Self> {
    /// Checks if two values are equal, and outputs a `CellBool` containing the result, which is
    /// determined at runtime.
    fn eq(self, other: Rhs) -> CellBool<'a, N>;

    /// Checks if two values are not equal, and outputs a `CellBool` containing the result, which is
    /// determined at runtime.
    fn ne(self, other: Rhs) -> CellBool<'a, N>;
}

/// Trait for equality comparisons between two values. The left hand side and right hand side do not
/// need to be of the same type, which is useful for comparing a cell with a value. For example,
/// `CellU8` implements `Eq<u8>`, `Eq<&CellU8>`, and `Eq<CellU8>`, allowing it to be compared to any
/// of these.
///
/// These traits output a `CellBool` value. This allows their results to be computed and used at
/// runtime. If compile-time checking is necessary, consider refactoring your code or using regular
/// equality checks instead.
pub trait Eq<'a, const N: usize, Rhs = Self>: PartialEq<'a, N, Rhs> {}

use crate::builder::allocating::bool::CellBool;

pub trait PartialEq<'a, const N: usize, Rhs = Self> {
    fn eq(self, other: Rhs) -> CellBool<'a, N>;

    fn ne(self, other: Rhs) -> CellBool<'a, N>;
}

pub trait Eq<'a, const N: usize, Rhs = Self>: PartialEq<'a, N, Rhs> {}

//! Provides a cell wrapping a boolean value at runtime.

use crate::builder::tracking::TrackingBuilder;

use super::{cell::IntoCell, core::AllocatingBuilder, u8::CellU8};
use std::{
    cell::RefMut,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Debug)]
/// A cell containing a boolean value which is guaranteed to either be zero or one.
pub struct CellBool<'a, const N: usize>(
    /// Invariant: this cell must always contain either a zero or a one once any algorithm finishes.
    pub(super) CellU8<'a, N>,
);

impl<'a, const N: usize> CellBool<'a, N> {
    fn borrow_builder_mut(&self) -> RefMut<'_, TrackingBuilder<N>> {
        self.0.memory.builder.borrow_mut()
    }

    /// Executes code while this cell is true.
    pub fn while_true(&self, f: impl FnOnce(&Self)) {
        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push('[');
        }

        f(self);

        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push(']');
        }
    }

    /// Executes code while this cell is true.
    pub fn while_true_mut(&mut self, f: impl FnOnce(&mut Self)) {
        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push('[');
        }

        f(self);

        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push(']');
        }
    }

    /// Executes code if this cell is true.
    pub fn if_true(mut self, f: impl FnOnce()) {
        self.while_true_mut(|this| {
            f();
            this.0.zero();
        });
    }

    /// Sets the value of this cell.
    pub fn set(&mut self, value: bool) {
        self.0.set(value as u8);
    }

    /// Negates the value contained in this cell.
    pub fn negate(&mut self) {
        let mut temp = self.0.clone();
        self.0.set(1);
        self.0.sub_and_zero(&mut temp);
    }

    /// Moves the value of this cell into another cell, leaving a `false` behind in this cell.
    pub fn move_into(&mut self, other: &mut CellBool<N>) {
        self.0.move_into(&mut other.0);
    }

    /// Moves the value of another cell into this cell, leaving a `false` behind in the other cell.
    pub fn move_from(&mut self, other: &mut CellBool<N>) {
        self.0.move_from(&mut other.0);
    }

    /// Copies the value of this cell into another cell.
    pub fn copy_into(&self, other: &mut CellBool<N>) {
        self.0.copy_into(&mut other.0);
    }

    /// Copies the value of another cell into this cell.
    pub fn copy_from(&mut self, other: &CellBool<N>) {
        self.0.copy_from(&other.0);
    }
}

impl<'a, const N: usize> IntoCell<'a, N> for bool {
    type Output = CellBool<'a, N>;

    fn into_cell(self, memory: &'a AllocatingBuilder<N>) -> Self::Output {
        memory.bool(self)
    }
}

impl<'a, const N: usize> Clone for CellBool<'a, N> {
    fn clone(&self) -> Self {
        let mut output = self.0.memory.bool_uninit();
        self.copy_into(&mut output);
        output
    }

    fn clone_from(&mut self, source: &Self) {
        source.copy_into(self);
    }
}

impl<'a, const N: usize> Not for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn not(mut self) -> Self::Output {
        self.negate();
        self
    }
}

impl<'a, const N: usize> Not for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn not(self) -> Self::Output {
        let mut temp = self.0.memory.bool(true);
        temp.0 -= &self.0;
        temp
    }
}

impl<'a, const N: usize> BitOrAssign for CellBool<'a, N> {
    fn bitor_assign(&mut self, mut rhs: Self) {
        rhs.0.add_and_zero(&mut self.0);
        rhs.if_true(|| self.0.inc());
    }
}

impl<'a, const N: usize> BitOrAssign<&CellBool<'a, N>> for CellBool<'a, N> {
    fn bitor_assign(&mut self, rhs: &CellBool<'a, N>) {
        self.bitor_assign(rhs.clone());
    }
}

impl<'a, const N: usize> BitOrAssign<bool> for CellBool<'a, N> {
    fn bitor_assign(&mut self, rhs: bool) {
        if rhs {
            self.set(true);
        }
    }
}

impl<'a, const N: usize> BitOr for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut output = self.clone();
        output |= rhs.clone();
        output
    }
}

impl<'a, const N: usize> BitOr<CellBool<'a, N>> for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitor(self, mut rhs: CellBool<'a, N>) -> Self::Output {
        rhs |= self;
        rhs
    }
}

impl<'a, const N: usize> BitOr<&CellBool<'a, N>> for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitor(mut self, rhs: &CellBool<'a, N>) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<'a, const N: usize> BitOr for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitor(self, mut rhs: CellBool<'a, N>) -> Self::Output {
        rhs |= self;
        rhs
    }
}

impl<'a, const N: usize> BitOr<bool> for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitor(mut self, rhs: bool) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<'a, const N: usize> BitOr<bool> for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitor(self, rhs: bool) -> Self::Output {
        self.clone() | rhs
    }
}

impl<'a, const N: usize> BitOr<CellBool<'a, N>> for bool {
    type Output = CellBool<'a, N>;

    fn bitor(self, rhs: CellBool<'a, N>) -> Self::Output {
        rhs | self
    }
}

impl<'a, const N: usize> BitOr<&CellBool<'a, N>> for bool {
    type Output = CellBool<'a, N>;

    fn bitor(self, rhs: &CellBool<'a, N>) -> Self::Output {
        rhs | self
    }
}

impl<'a, const N: usize> BitAndAssign for CellBool<'a, N> {
    fn bitand_assign(&mut self, rhs: Self) {
        let mut rhs = rhs.0;
        // rhs = 0 (false) or 1 (true)
        rhs.dec();
        // rhs = 255 (false) or 0 (true)
        rhs.while_nonzero_mut(|rhs| {
            // rhs = 255 (false)
            rhs.inc();
            // rhs = 0 (false)
            self.set(false);
        });
    }
}

impl<'a, const N: usize> BitAndAssign<&CellBool<'a, N>> for CellBool<'a, N> {
    fn bitand_assign(&mut self, rhs: &CellBool<'a, N>) {
        let rhs = rhs.clone();
        *self &= rhs;
    }
}

impl<'a, const N: usize> BitAndAssign<bool> for CellBool<'a, N> {
    fn bitand_assign(&mut self, rhs: bool) {
        if !rhs {
            self.set(false);
        }
    }
}

impl<'a, const N: usize> BitAnd for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<'a, const N: usize> BitAnd<CellBool<'a, N>> for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitand(self, mut rhs: CellBool<'a, N>) -> Self::Output {
        rhs &= self;
        rhs
    }
}

impl<'a, const N: usize> BitAnd<&CellBool<'a, N>> for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitand(mut self, rhs: &CellBool<'a, N>) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<'a, const N: usize> BitAnd for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut value = self.clone();
        value &= rhs.clone();
        value
    }
}

impl<'a, const N: usize> BitAnd<bool> for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitand(mut self, rhs: bool) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<'a, const N: usize> BitAnd<bool> for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitand(self, rhs: bool) -> Self::Output {
        self.clone() & rhs
    }
}

impl<'a, const N: usize> BitAnd<CellBool<'a, N>> for bool {
    type Output = CellBool<'a, N>;

    fn bitand(self, rhs: CellBool<'a, N>) -> Self::Output {
        rhs & self
    }
}

impl<'a, const N: usize> BitAnd<&CellBool<'a, N>> for bool {
    type Output = CellBool<'a, N>;

    fn bitand(self, rhs: &CellBool<'a, N>) -> Self::Output {
        rhs & self
    }
}

impl<'a, const N: usize> BitXorAssign for CellBool<'a, N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        rhs.if_true(|| {
            self.negate();
        });
    }
}

impl<'a, const N: usize> BitXorAssign<&CellBool<'a, N>> for CellBool<'a, N> {
    fn bitxor_assign(&mut self, rhs: &CellBool<'a, N>) {
        rhs.clone().if_true(|| {
            self.negate();
        })
    }
}

impl<'a, const N: usize> BitXorAssign<bool> for CellBool<'a, N> {
    fn bitxor_assign(&mut self, rhs: bool) {
        if rhs {
            self.negate();
        }
    }
}

impl<'a, const N: usize> BitXor for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitxor(mut self, rhs: CellBool<'a, N>) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl<'a, const N: usize> BitXor<&CellBool<'a, N>> for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitxor(mut self, rhs: &CellBool<'a, N>) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl<'a, const N: usize> BitXor<CellBool<'a, N>> for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitxor(self, mut rhs: CellBool<'a, N>) -> Self::Output {
        rhs ^= self;
        rhs
    }
}

impl<'a, const N: usize> BitXor for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitxor(self, rhs: &CellBool<'a, N>) -> Self::Output {
        self.clone() ^ rhs
    }
}

impl<'a, const N: usize> BitXor<bool> for CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitxor(mut self, rhs: bool) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl<'a, const N: usize> BitXor<bool> for &CellBool<'a, N> {
    type Output = CellBool<'a, N>;

    fn bitxor(self, rhs: bool) -> Self::Output {
        self.clone() ^ rhs
    }
}

impl<'a, const N: usize> BitXor<CellBool<'a, N>> for bool {
    type Output = CellBool<'a, N>;

    fn bitxor(self, mut rhs: CellBool<'a, N>) -> Self::Output {
        rhs ^= self;
        rhs
    }
}

impl<'a, const N: usize> BitXor<&CellBool<'a, N>> for bool {
    type Output = CellBool<'a, N>;

    fn bitxor(self, rhs: &CellBool<'a, N>) -> Self::Output {
        rhs.clone() ^ self
    }
}

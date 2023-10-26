//! Provides a cell wrapping a boolean value at runtime.

use crate::builder::tracking::TrackingBuilder;

use super::u8::CellU8;
use std::{
    cell::RefMut,
    ops::{BitOr, BitOrAssign, Not},
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
    pub fn while_true(&self, f: impl FnOnce()) {
        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push('[');
        }

        f();

        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push(']');
        }
    }

    /// Executes code if this cell is true.
    pub fn if_true(self, f: impl FnOnce()) {
        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.source_mut().push('[');
        }

        f();

        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.0.location);
            builder.zero();
            builder.source_mut().push(']');
        }
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

//! Provides a cell wrapping a `u8` value at runtime.

use super::{
    bool::CellBool,
    core::AllocatingBuilder,
    ops::eq::{Eq, PartialEq},
};
use crate::builder::tracking::TrackingBuilder;
use std::{
    cell::RefMut,
    ops::{Add, AddAssign, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug)]
/// A cell containing an unsigned 8-bit value.
pub struct CellU8<'a, const N: usize> {
    pub(super) memory: &'a AllocatingBuilder<N>,
    pub(super) location: usize,
}

impl<'a, const N: usize> CellU8<'a, N> {
    fn borrow_builder_mut(&self) -> RefMut<'_, TrackingBuilder<N>> {
        self.memory.builder.borrow_mut()
    }

    /// Reads a value from input into this cell.
    pub fn read(&mut self) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.read();
    }

    /// Writes the value from this cell into output.
    pub fn write(&self) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.write();
    }

    /// Sets the value of this cell to zero.
    pub fn zero(&mut self) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.zero();
    }

    /// Sets the value of this cell to a `u8` value.
    pub fn set(&mut self, value: u8) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.set(value);
    }

    /// Increments the value of this cell.
    pub fn inc(&mut self) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.inc();
    }

    /// Increments the value of this cell by a `u8` value.
    pub fn inc_by(&mut self, value: u8) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.inc_by(value);
    }

    /// Decrements the value of this cell.
    pub fn dec(&mut self) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.dec();
    }

    /// Decrements the value of this cell by a `u8` value.
    pub fn dec_by(&mut self, value: u8) {
        let mut builder = self.borrow_builder_mut();
        builder.goto(self.location);
        builder.dec_by(value);
    }

    /// Creates a loop while this cell value is nonzero.
    pub fn while_nonzero(&self, f: impl FnOnce(&Self)) {
        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.location);
            builder.source_mut().push('[');
        }

        f(self);

        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.location);
            builder.source_mut().push(']');
        }
    }

    /// Creates a loop while this cell value is nonzero.
    pub fn while_nonzero_mut(&mut self, f: impl FnOnce(&mut Self)) {
        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.location);
            builder.source_mut().push('[');
        }

        f(self);

        {
            let mut builder = self.borrow_builder_mut();
            builder.goto(self.location);
            builder.source_mut().push(']');
        }
    }

    /// Moves the value of this cell into another cell, leaving a `0` behind in this cell.
    pub fn move_into(&mut self, other: &mut CellU8<N>) {
        other.zero();

        self.while_nonzero_mut(|this| {
            this.dec();
            other.inc();
        })
    }

    /// Moves the value of another cell into this cell, leaving a `0` behind in the other cell.
    pub fn move_from(&mut self, other: &mut CellU8<N>) {
        other.move_into(self);
    }

    /// Copies the value of this cell into another cell.
    pub fn copy_into(&self, other: &mut CellU8<N>) {
        let temp = self.memory.u8(0);

        // We have to resort to a low-level implementation here because all the methods that we need
        // mutate `self`, but we only have a regular reference.

        let mut builder = self.borrow_builder_mut();

        // This moves `self` into `temp`, leaving `self` as 0.
        builder.repeat_at(self.location, |builder| {
            builder.dec();
            builder.goto(temp.location);
            builder.inc();
        });

        // This moves `temp` into `self` and `other`, leaving it as 0.
        builder.repeat_at(temp.location, |builder| {
            builder.dec();
            builder.goto(self.location);
            builder.inc();
            builder.goto(other.location);
            builder.inc();
        });
    }

    /// Copies the value of another cell into this cell.
    pub fn copy_from(&mut self, other: &CellU8<N>) {
        other.copy_into(self);
    }

    /// Adds the value of `other` into `self`, zeroing `other` in the process.
    pub fn add_and_zero(&mut self, other: &mut CellU8<N>) {
        other.while_nonzero_mut(|other| {
            self.inc();
            other.dec();
        });
    }

    /// Subtracts the value of `other` from `self`, zeroing `other` in the process.
    pub fn sub_and_zero(&mut self, other: &mut CellU8<N>) {
        other.while_nonzero_mut(|other| {
            self.dec();
            other.dec();
        });
    }

    /// Returns a `CellBool` indicating if `self` is nonzero.
    pub fn is_nonzero(mut self) -> CellBool<'a, N> {
        let mut output = self.memory.bool(false);

        self.while_nonzero_mut(|this| {
            this.zero();
            output.0.inc();
        });

        output
    }

    /// Returns a `CellBool` indicating if `self` is zero.
    pub fn is_zero(mut self) -> CellBool<'a, N> {
        let mut output = self.memory.bool(true);

        self.while_nonzero_mut(|this| {
            this.zero();
            output.0.dec();
        });

        output
    }
}

impl<'a, const N: usize> Clone for CellU8<'a, N> {
    fn clone(&self) -> Self {
        let mut output = self.memory.u8_uninit();
        self.copy_into(&mut output);
        output
    }

    fn clone_from(&mut self, source: &Self) {
        source.copy_into(self);
    }
}

impl<'a, const N: usize> Drop for CellU8<'a, N> {
    fn drop(&mut self) {
        self.memory.deallocate(self.location);
    }
}

impl<'a, const N: usize> AddAssign<u8> for CellU8<'a, N> {
    fn add_assign(&mut self, rhs: u8) {
        self.inc_by(rhs);
    }
}

impl<'a, const N: usize> AddAssign<&CellU8<'a, N>> for CellU8<'a, N> {
    fn add_assign(&mut self, rhs: &CellU8<'a, N>) {
        let mut temp = rhs.clone();
        self.add_and_zero(&mut temp);
    }
}

impl<'a, const N: usize> AddAssign for CellU8<'a, N> {
    fn add_assign(&mut self, mut rhs: CellU8<'a, N>) {
        self.add_and_zero(&mut rhs);
    }
}

impl<'a, const N: usize> SubAssign<u8> for CellU8<'a, N> {
    fn sub_assign(&mut self, rhs: u8) {
        self.dec_by(rhs);
    }
}

impl<'a, const N: usize> SubAssign<&CellU8<'a, N>> for CellU8<'a, N> {
    fn sub_assign(&mut self, rhs: &CellU8<'a, N>) {
        let mut temp = rhs.clone();
        self.sub_and_zero(&mut temp);
    }
}

impl<'a, const N: usize> SubAssign for CellU8<'a, N> {
    fn sub_assign(&mut self, mut rhs: CellU8<'a, N>) {
        self.sub_and_zero(&mut rhs);
    }
}

impl<'a, const N: usize> Add<u8> for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn add(self, rhs: u8) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}

impl<'a, const N: usize> Add<&CellU8<'a, N>> for u8 {
    type Output = CellU8<'a, N>;

    fn add(self, rhs: &CellU8<'a, N>) -> Self::Output {
        let mut output = rhs.clone();
        output += self;
        output
    }
}

impl<'a, const N: usize> Add<u8> for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn add(mut self, rhs: u8) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, const N: usize> Add<CellU8<'a, N>> for u8 {
    type Output = CellU8<'a, N>;

    fn add(self, mut rhs: CellU8<'a, N>) -> Self::Output {
        rhs += self;
        rhs
    }
}

impl<'a, const N: usize> Add for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}

impl<'a, const N: usize> Add<&CellU8<'a, N>> for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn add(mut self, rhs: &CellU8<'a, N>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, const N: usize> Add<CellU8<'a, N>> for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn add(self, mut rhs: CellU8<'a, N>) -> Self::Output {
        rhs += self;
        rhs
    }
}

impl<'a, const N: usize> Add for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn add(mut self, mut rhs: CellU8<'a, N>) -> Self::Output {
        self.add_and_zero(&mut rhs);
        self
    }
}

impl<'a, const N: usize> Sub<u8> for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn sub(self, rhs: u8) -> Self::Output {
        let mut output = self.clone();
        output -= rhs;
        output
    }
}

impl<'a, const N: usize> Sub<&CellU8<'a, N>> for u8 {
    type Output = CellU8<'a, N>;

    fn sub(self, rhs: &CellU8<'a, N>) -> Self::Output {
        let mut output = rhs.memory.u8(self);
        output -= rhs;
        output
    }
}

impl<'a, const N: usize> Sub<u8> for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn sub(mut self, rhs: u8) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, const N: usize> Sub<CellU8<'a, N>> for u8 {
    type Output = CellU8<'a, N>;

    fn sub(self, rhs: CellU8<'a, N>) -> Self::Output {
        let mut output = rhs.memory.u8(self);
        output -= rhs;
        output
    }
}

impl<'a, const N: usize> Sub for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = self.clone();
        output -= rhs;
        output
    }
}

impl<'a, const N: usize> Sub<&CellU8<'a, N>> for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn sub(mut self, rhs: &CellU8<'a, N>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, const N: usize> Sub<CellU8<'a, N>> for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn sub(self, rhs: CellU8<'a, N>) -> Self::Output {
        let mut output = self.clone();
        output -= rhs;
        output
    }
}

impl<'a, const N: usize> Sub for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn sub(mut self, mut rhs: CellU8<'a, N>) -> Self::Output {
        self.sub_and_zero(&mut rhs);
        self
    }
}

impl<'a, const N: usize> Neg for &CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn neg(self) -> Self::Output {
        let mut output = self.memory.u8(0);
        output -= self;
        output
    }
}

impl<'a, const N: usize> Neg for CellU8<'a, N> {
    type Output = CellU8<'a, N>;

    fn neg(mut self) -> Self::Output {
        let mut temp = self.memory.u8_uninit();
        self.move_into(&mut temp);
        self -= temp;
        self
    }
}

impl<'a, const N: usize> PartialEq<'a, N, u8> for CellU8<'a, N> {
    fn eq(mut self, other: u8) -> CellBool<'a, N> {
        self -= other;
        self.is_zero()
    }

    fn ne(mut self, other: u8) -> CellBool<'a, N> {
        self -= other;
        self.is_nonzero()
    }
}

impl<'a, const N: usize> Eq<'a, N, u8> for CellU8<'a, N> {}

impl<'a, const N: usize> PartialEq<'a, N, u8> for &CellU8<'a, N> {
    fn eq(self, other: u8) -> CellBool<'a, N> {
        let mut output = self.clone();
        output -= other;
        output.is_zero()
    }

    fn ne(self, other: u8) -> CellBool<'a, N> {
        let mut output = self.clone();
        output -= other;
        output.is_nonzero()
    }
}

impl<'a, const N: usize> Eq<'a, N, u8> for &CellU8<'a, N> {}

impl<'a, const N: usize> PartialEq<'a, N> for &CellU8<'a, N> {
    fn eq(self, other: Self) -> CellBool<'a, N> {
        let mut output = self.clone();
        output -= other;
        output.is_zero()
    }

    fn ne(self, other: Self) -> CellBool<'a, N> {
        let mut output = self.clone();
        output -= other;
        output.is_nonzero()
    }
}

impl<'a, const N: usize> Eq<'a, N> for &CellU8<'a, N> {}

impl<'a, const N: usize> PartialEq<'a, N, CellU8<'a, N>> for &CellU8<'a, N> {
    fn eq(self, mut other: CellU8<'a, N>) -> CellBool<'a, N> {
        other -= self;
        other.is_zero()
    }

    fn ne(self, mut other: CellU8<'a, N>) -> CellBool<'a, N> {
        other -= self;
        other.is_nonzero()
    }
}

impl<'a, const N: usize> Eq<'a, N, CellU8<'a, N>> for &CellU8<'a, N> {}

impl<'a, const N: usize> PartialEq<'a, N, &CellU8<'a, N>> for CellU8<'a, N> {
    fn eq(self, other: &CellU8<'a, N>) -> CellBool<'a, N> {
        let mut output = self.clone();
        output -= other;
        output.is_zero()
    }

    fn ne(self, other: &CellU8<'a, N>) -> CellBool<'a, N> {
        let mut output = self.clone();
        output -= other;
        output.is_nonzero()
    }
}

impl<'a, const N: usize> Eq<'a, N, &CellU8<'a, N>> for CellU8<'a, N> {}

impl<'a, const N: usize> PartialEq<'a, N> for CellU8<'a, N> {
    fn eq(mut self, other: Self) -> CellBool<'a, N> {
        self -= other;
        self.is_zero()
    }

    fn ne(mut self, other: Self) -> CellBool<'a, N> {
        self -= other;
        self.is_nonzero()
    }
}

impl<'a, const N: usize> Eq<'a, N> for CellU8<'a, N> {}

impl<'a, const N: usize> MulAssign for CellU8<'a, N> {
    fn mul_assign(&mut self, rhs: Self) {
        let mut x = self.memory.u8_uninit();
        x.move_from(self);

        x.while_nonzero_mut(|x| {
            x.dec();
            self.add_and_zero(&mut rhs.clone());
        });
    }
}

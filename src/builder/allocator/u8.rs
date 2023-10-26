//! Provides a cell wrapping a `u8` value at runtime.

use super::{
    bool::CellBool,
    core::AllocatingBuilder,
    ops::eq::{Eq, PartialEq},
};
use crate::builder::tracking::TrackingBuilder;
use std::{
    cell::RefMut,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
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

    /// `others` must be guaranteed to contain valid memory locations.
    fn move_into_all_locations<const U: usize>(&mut self, others: [usize; U]) {
        let mut builder = self.borrow_builder_mut();

        for other in others {
            builder.goto(other);
            builder.zero();
        }

        builder.repeat_at(self.location, |builder| {
            builder.dec();
            for other in others {
                builder.goto(other);
                builder.inc();
            }
        });
    }

    /// Moves the value of this cell into another cell, leaving a `0` behind in this cell.
    pub fn move_into(&mut self, other: &mut CellU8<N>) {
        self.move_into_all_locations([other.location]);
    }

    /// Moves the value of another cell into this cell, leaving a `0` behind in the other cell.
    pub fn move_from(&mut self, other: &mut CellU8<N>) {
        other.move_into(self);
    }

    /// Copies the value of this cell into another cell.
    pub fn copy_into(&self, other: &mut CellU8<N>) {
        let mut temp = self.memory.u8_uninit();

        {
            let other: &mut CellU8<N> = &mut temp;

            let others = [other.location];
            let mut builder = self.borrow_builder_mut();

            for other in others {
                builder.goto(other);
                builder.zero();
            }

            builder.repeat_at(self.location, |builder| {
                builder.dec();
                for other in others {
                    builder.goto(other);
                    builder.inc();
                }
            });
        }

        temp.move_into_all_locations([self.location, other.location]);
    }

    /// Copies the value of another cell into this cell.
    pub fn copy_from(&mut self, other: &CellU8<N>) {
        other.copy_into(self);
    }

    /// Adds the value of `other` into `self`, zeroing `other` in the process.
    pub fn add_and_zero(&mut self, other: &mut CellU8<N>) {
        let mut builder = self.borrow_builder_mut();

        builder.repeat_at(other.location, |builder| {
            builder.dec();
            builder.goto(self.location);
            builder.inc();
        });
    }

    /// Subtracts the value of `other` from `self`, zeroing `other` in the process.
    pub fn sub_and_zero(&mut self, other: &mut CellU8<N>) {
        let mut builder = self.borrow_builder_mut();

        builder.repeat_at(other.location, |builder| {
            builder.dec();
            builder.goto(self.location);
            builder.dec();
        });
    }

    /// Returns a `CellBool` indicating if `self` is nonzero.
    pub fn is_nonzero(self) -> CellBool<'a, N> {
        let output = self.memory.bool(false);
        let mut builder = self.borrow_builder_mut();
        builder.repeat_at(self.location, |builder| {
            builder.zero();
            builder.goto(output.0.location);
            builder.inc();
        });
        output
    }

    /// Returns a `CellBool` indicating if `self` is zero.
    pub fn is_zero(self) -> CellBool<'a, N> {
        let output = self.memory.bool(true);
        let mut builder = self.borrow_builder_mut();
        builder.repeat_at(self.location, |builder| {
            builder.zero();
            builder.goto(output.0.location);
            builder.dec();
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

// u8 == (&)CellU8

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

// (&)CellU8 == &CellU8

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

// (&)CellU8 == CellU8

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

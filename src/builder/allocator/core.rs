//! This module defines a builder which allocates memory into cells and releases them automatically.

use super::bool::CellBool;
use super::u8::CellU8;
use crate::builder::tracking::TrackingBuilder;
use crate::{compiler::Program, runner::Runner};
use std::cell::{Cell, RefCell};

#[derive(Debug)]
/// A builder which allocates memory into cells and releases them automatically.
pub struct AllocatingBuilder<const N: usize> {
    /// A tracking builder which is loaned out to data cells. Using a tracking builder allows cells
    /// to easily move to their locations when operations are done on them.
    pub(super) builder: RefCell<TrackingBuilder<N>>,

    /// An array of memory cells.
    memory: Cell<[bool; N]>,

    /// The earliest open space in the `memory` array. This must always point to the index of a
    /// `false` space in the `memory` array.
    earliest_open_space: Cell<usize>,
}

impl<const N: usize> AllocatingBuilder<N> {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            builder: RefCell::new(TrackingBuilder::new()),
            memory: Cell::new([false; N]),
            earliest_open_space: Cell::new(0),
        }
    }

    /// Compiles the source of this builder, returning an error if it is malformed.
    pub fn compile(&self) -> Result<Program, &'static str> {
        self.builder.borrow().compile()
    }

    /// Runs the code in this builder on a given input, returning an error if it is malformed.
    pub fn run(&self, input: &[u8]) -> Result<Runner<N>, &'static str> {
        self.compile().map(|program| program.run::<N>(input))
    }

    /// Gets the source of this builder. Returns an owned `String` due to technicalities.
    pub fn source(&self) -> String {
        self.builder.borrow().source().to_owned()
    }

    /// Allocates a byte of memory, returning its location.
    fn allocate(&self) -> usize {
        let mut memory = self.memory.get();
        let location = self.earliest_open_space.get();
        memory[location] = true;

        for index in location + 1..N {
            if !memory[index] {
                self.earliest_open_space.set(index);
                return location;
            }
        }

        panic!("out of memory");
    }

    /// Deallocates a byte of memory.
    pub(super) fn deallocate(&self, location: usize) {
        let mut memory = self.memory.get();
        memory[location] = false;

        let previous_earliest_open_space = self.earliest_open_space.get();
        self.earliest_open_space
            .set(location.min(previous_earliest_open_space));
    }

    /// Allocates an uninitialized `u8` value. Its value is not guaranteed to be zero.
    pub(super) fn u8_uninit(&self) -> CellU8<N> {
        let location = self.allocate();

        CellU8 {
            memory: self,
            location,
        }
    }

    /// Allocates a `u8` and gives it a defined value.
    pub fn u8(&self, value: u8) -> CellU8<N> {
        let mut cell = self.u8_uninit();
        cell.set(value);
        cell
    }

    /// Allocates an uninitialized `bool` value. Its value is not guaranteed to be false, or even to
    /// be a valid boolean.
    pub(super) fn bool_uninit(&self) -> CellBool<N> {
        CellBool(self.u8_uninit())
    }

    /// Allocates a `bool` and gives it a defined value.
    pub fn bool(&self, value: bool) -> CellBool<N> {
        let mut cell = self.bool_uninit();
        cell.set(value);
        cell
    }
}

use super::bool::CellBool;
use super::u8::CellU8;
use crate::builder::tracking::TrackingBuilder;
use crate::{compiler::Program, runner::Runner};
use std::cell::{Cell, RefCell};

#[derive(Debug)]
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
    pub fn new() -> Self {
        Self {
            builder: RefCell::new(TrackingBuilder::new()),
            memory: Cell::new([false; N]),
            earliest_open_space: Cell::new(0),
        }
    }

    pub fn compile(&self) -> Result<Program, &'static str> {
        self.builder.borrow().compile()
    }

    pub fn run(&self, input: &[u8]) -> Result<Runner<N>, &'static str> {
        self.compile().map(|program| program.run::<N>(input))
    }

    pub fn source(&self) -> String {
        self.builder.borrow().source().to_owned()
    }

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

    pub(super) fn deallocate(&self, location: usize) {
        let mut memory = self.memory.get();
        memory[location] = false;

        let previous_earliest_open_space = self.earliest_open_space.get();
        self.earliest_open_space
            .set(location.min(previous_earliest_open_space));
    }

    pub(super) fn u8_uninit(&self) -> CellU8<N> {
        let location = self.allocate();

        let mut cell = CellU8 {
            memory: self,
            location,
        };

        cell.set(::rand::random());

        cell
    }

    pub fn u8(&self, value: u8) -> CellU8<N> {
        let mut cell = self.u8_uninit();
        cell.set(value);
        cell
    }

    pub(super) fn bool_uninit(&self) -> CellBool<N> {
        let mut cell = CellBool(self.u8_uninit());
        cell.set(::rand::random());
        cell
    }

    pub fn bool(&self, value: bool) -> CellBool<N> {
        let mut cell = self.bool_uninit();
        cell.set(value);
        cell
    }
}

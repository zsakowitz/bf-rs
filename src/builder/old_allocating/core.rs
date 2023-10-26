use super::u8::CellU8;
use crate::builder::tracking::TrackingBuilder;
use crate::{compiler::Program, runner::Runner};
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct AllocatingBuilder<const N: usize> {
    builder: RefCell<TrackingBuilder<N>>,
    allocated_memory: Cell<usize>,
}

impl<const N: usize> AllocatingBuilder<N> {
    pub fn new() -> Self {
        Self {
            builder: RefCell::new(TrackingBuilder::new()),
            allocated_memory: Cell::new(0),
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

    pub fn allocated_memory(&self) -> usize {
        self.allocated_memory.get()
    }

    pub fn u8(&self) -> CellU8<N> {
        let location = self.allocated_memory.get();
        self.allocated_memory.set(location + 1);

        CellU8 {
            location,
            builder: &self.builder,
        }
    }
}

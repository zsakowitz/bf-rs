pub mod bool;
pub mod u8;
use self::{bool::CellBool, u8::CellU8};
use super::tracking::TrackingBuilder;
use crate::{compiler::Program, runner::Runner};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub struct AllocatingBuilder<const N: usize> {
    builder: Rc<RefCell<TrackingBuilder<N>>>,
    allocated_memory: usize,
}

impl<const N: usize> AllocatingBuilder<N> {
    pub fn new() -> Self {
        Self {
            builder: Rc::new(RefCell::new(TrackingBuilder::new())),
            allocated_memory: 0,
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
        self.allocated_memory
    }

    pub fn u8(&mut self) -> CellU8<N> {
        let location = self.allocated_memory;
        self.allocated_memory += 1;

        CellU8 {
            location,
            builder: Rc::clone(&self.builder),
        }
    }

    pub fn u8_with(&mut self, value: u8) -> CellU8<N> {
        let mut cell = self.u8();
        cell.set_to(value);
        cell
    }

    pub fn bool(&mut self) -> CellBool<N> {
        let location = self.allocated_memory;
        self.allocated_memory += 1;

        CellBool {
            location,
            builder: Rc::clone(&self.builder),
        }
    }

    pub fn bool_with(&mut self, value: bool) -> CellBool<N> {
        let mut cell = self.bool();
        cell.set_to(value);
        cell
    }
}

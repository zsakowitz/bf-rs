use super::u8::CellU8;
use crate::builder::tracking::TrackingBuilder;
use std::{cell::RefCell, rc::Rc};

// Invariants: the cell referenced at this location must always contain either a zero or a one.
pub struct CellBool<const N: usize> {
    pub(super) location: usize,
    pub(super) builder: Rc<RefCell<TrackingBuilder<N>>>,
}

impl<const N: usize> CellBool<N> {
    pub fn set_to(&mut self, value: bool) {
        let mut builder = self.builder.borrow_mut();

        builder.goto(self.location);
        builder.repeat(|builder| builder.dec());

        if value {
            builder.inc();
        }
    }

    pub fn into_u8(self) -> CellU8<N> {
        CellU8 {
            location: self.location,
            builder: self.builder,
        }
    }

    pub fn while_true(&self, f: impl FnOnce(&Self)) {
        {
            let mut builder = self.builder.borrow_mut();
            builder.goto(self.location);
            builder.source_mut().push('[');
        }

        f(self);

        {
            let mut builder = self.builder.borrow_mut();
            builder.goto(self.location);
            builder.source_mut().push(']');
        }
    }

    pub fn if_true(&mut self, f: impl FnOnce(&Self)) {
        {
            let mut builder = self.builder.borrow_mut();
            builder.goto(self.location);
            builder.source_mut().push('[');
        }

        f(self);

        {
            let mut builder = self.builder.borrow_mut();
            builder.goto(self.location);
            builder.repeat(|builder| builder.dec());
            builder.source_mut().push(']');
        }
    }
}

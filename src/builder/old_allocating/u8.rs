use crate::builder::tracking::TrackingBuilder;
use std::cell::RefCell;

#[derive(Debug)]
pub struct CellU8<'a, const N: usize> {
    pub(super) location: usize,
    pub(super) builder: &'a RefCell<TrackingBuilder<N>>,
}

impl<'a, const N: usize> CellU8<'a, N> {
    pub fn goto(&self) {
        self.builder.borrow_mut().goto(self.location);
    }

    pub fn inc(&mut self) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.inc();
    }

    pub fn inc_by(&mut self, value: u8) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.inc_by(value);
    }

    pub fn dec(&mut self) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.dec();
    }

    pub fn dec_by(&mut self, value: u8) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.dec_by(value);
    }

    pub fn set(&mut self, value: u8) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.set(value);
    }

    pub fn move_into(&mut self, other: &mut CellU8<N>) {
        let mut builder = self.builder.borrow_mut();

        builder.goto(other.location);
        builder.zero();
        builder.goto(self.location);
        builder.repeat(|builder| {
            builder.goto(other.location);
            builder.inc();
            builder.goto(self.location);
            builder.dec();
        });
    }

    pub fn move_from(&mut self, other: &mut CellU8<N>) {
        let mut builder = self.builder.borrow_mut();

        builder.goto(self.location);
        builder.zero();
        builder.goto(other.location);
        builder.repeat(|builder| {
            builder.goto(self.location);
            builder.inc();
            builder.goto(other.location);
            builder.dec();
        });
    }
}

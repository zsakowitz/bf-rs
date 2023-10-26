use crate::builder::tracking::TrackingBuilder;
use std::{
    cell::RefCell,
    fmt::{self, Debug, Formatter},
    ops,
    rc::Rc,
};

pub struct CellU8<const N: usize> {
    pub(super) location: usize,
    pub(super) builder: Rc<RefCell<TrackingBuilder<N>>>,
}

impl<const N: usize> CellU8<N> {
    pub fn zero(&mut self) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.repeat(|builder| builder.dec());
    }

    pub fn set_to(&mut self, value: u8) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.repeat(|builder| builder.dec());
        builder.inc_by(value);
    }
}

impl<const N: usize> ops::AddAssign<u8> for CellU8<N> {
    fn add_assign(&mut self, rhs: u8) {
        let mut builder = self.builder.borrow_mut();
        builder.goto(self.location);
        builder.inc_by(rhs);
    }
}

impl<const N: usize> ops::AddAssign<&mut CellU8<N>> for CellU8<N> {
    fn add_assign(&mut self, rhs: &mut CellU8<N>) {
        let mut builder = self.builder.borrow_mut();

        builder.goto(rhs.location);

        builder.repeat(|builder| {
            builder.dec();
            builder.goto(self.location);
            builder.inc();
            builder.goto(rhs.location);
        });
    }
}

impl<const N: usize> ops::SubAssign<u8> for CellU8<N> {
    fn sub_assign(&mut self, rhs: u8) {
        let mut builder = self.builder.borrow_mut();

        builder.goto(self.location);
        builder.dec_by(rhs);
    }
}

impl<const N: usize> ops::SubAssign<&mut CellU8<N>> for CellU8<N> {
    fn sub_assign(&mut self, rhs: &mut CellU8<N>) {
        let mut builder = self.builder.borrow_mut();

        builder.goto(rhs.location);

        builder.repeat(|builder| {
            builder.dec();
            builder.goto(self.location);
            builder.dec();
            builder.goto(rhs.location);
        });
    }
}

impl<const N: usize> Debug for CellU8<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CellU8").field(&self.location).finish()
    }
}

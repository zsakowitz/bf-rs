use super::core::Builder;
use crate::{compiler::Program, runner::Runner};

#[derive(Debug)]
pub struct TrackingBuilder<const N: usize> {
    builder: Builder,
    index: usize,
}

impl<const N: usize> TrackingBuilder<N> {
    pub fn new() -> Self {
        Self {
            builder: Builder::new(),
            index: 0,
        }
    }

    pub fn compile(&self) -> Result<Program, &'static str> {
        self.builder.compile()
    }

    pub fn run(&self, input: &[u8]) -> Result<Runner<N>, &'static str> {
        self.compile().map(|program| program.run::<N>(input))
    }

    pub fn source(&self) -> &str {
        &self.builder.source()
    }

    pub(super) fn source_mut(&mut self) -> &mut String {
        &mut self.builder.source
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn inc(&mut self) {
        self.builder.inc();
    }

    pub fn inc_by(&mut self, value: u8) {
        self.builder.inc_by(value);
    }

    pub fn dec(&mut self) {
        self.builder.dec();
    }

    pub fn dec_by(&mut self, value: u8) {
        self.builder.dec_by(value);
    }

    pub fn zero(&mut self) {
        self.builder.repeat(|builder| builder.dec());
    }

    pub fn set(&mut self, value: u8) {
        self.zero();
        self.inc_by(value);
    }

    pub fn goto(&mut self, index: usize) {
        if index >= N {
            panic!("pointer index cannot be larger than N");
        }

        if index < self.index {
            for _ in 0..self.index - index {
                self.builder.shl();
            }
        } else if index > self.index {
            for _ in 0..index - self.index {
                self.builder.shr();
            }
        }

        self.index = index;
    }

    pub fn shl(&mut self) {
        self.builder.shl();

        if self.index == 0 {
            self.index = N - 1;
        } else {
            self.index -= 1;
        }
    }

    pub fn shr(&mut self) {
        self.builder.shr();

        if self.index == N - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }

    pub fn read(&mut self) {
        self.builder.read();
    }

    pub fn write(&mut self) {
        self.builder.write();
    }

    pub fn repeat(&mut self, f: impl FnOnce(&mut Self)) {
        let old_index = self.index;

        self.builder.source.push('[');
        f(self);
        self.builder.source.push(']');

        if self.index != old_index {
            panic!("the pointer index unexpectedly changed in a [...] loop");
        }
    }

    pub fn repeat_at(&mut self, location: usize, f: impl FnOnce(&mut Self)) {
        self.goto(location);
        self.builder.source.push('[');
        f(self);
        self.goto(location);
        self.builder.source.push(']');
    }

    pub fn preserve_location(&mut self, f: impl FnOnce(&mut Self)) {
        let old_index = self.index;
        f(self);
        self.goto(old_index);
    }
}

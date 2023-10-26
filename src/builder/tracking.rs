//! A builder that tracks the current pointer location, allowing the use of `goto` commands.

use super::core::Builder;
use crate::{compiler::Program, runner::Runner};

#[derive(Debug)]
/// A builder that tracks the current pointer location, allowing the use of `goto` commands.
pub struct TrackingBuilder<const N: usize> {
    builder: Builder,
    index: usize,
}

impl<const N: usize> TrackingBuilder<N> {
    /// Constructs a new `TrackingBuilder`.
    pub fn new() -> Self {
        Self {
            builder: Builder::new(),
            index: 0,
        }
    }

    /// Compiles the source code of this builder, returning an error if the source is malformed.
    pub fn compile(&self) -> Result<Program, &'static str> {
        self.builder.compile()
    }

    /// Runs the source code in this builder, returning an error if the source is malformed.
    pub fn run(&self, input: &[u8]) -> Result<Runner<N>, &'static str> {
        self.compile().map(|program| program.run::<N>(input))
    }

    /// Gets the source code of this builder.
    pub fn source(&self) -> &str {
        &self.builder.source()
    }

    /// Gets a mutable ref to the source code of this builder.
    pub(super) fn source_mut(&mut self) -> &mut String {
        &mut self.builder.source
    }

    /// Gets the currently pointed at memory index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Adds an increment command.
    pub fn inc(&mut self) {
        self.builder.inc();
    }

    /// Adds several increment commands, or several decrement commands if `value > 128`.
    pub fn inc_by(&mut self, value: u8) {
        self.builder.inc_by(value);
    }

    /// Adds a decrement command.
    pub fn dec(&mut self) {
        self.builder.dec();
    }

    /// Adds several decrement commands, or several increment commands if `value > 128`.
    pub fn dec_by(&mut self, value: u8) {
        self.builder.dec_by(value);
    }

    /// Sets the current cell to zero.
    pub fn zero(&mut self) {
        self.builder.repeat(|builder| builder.dec());
    }

    /// Sets the current cell to a given value.
    pub fn set(&mut self, value: u8) {
        self.zero();
        self.inc_by(value);
    }

    /// Adds several pointer shift commands to move the pointer to a given position.
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

    /// Adds a command to read input into the current cell.
    pub fn read(&mut self) {
        self.builder.read();
    }

    /// Adds a command to write the current cell into output.
    pub fn write(&mut self) {
        self.builder.write();
    }

    /// Repeats the commands inside while the current cell is nonzero.
    pub fn repeat(&mut self, f: impl FnOnce(&mut Self)) {
        let old_index = self.index;

        self.builder.source.push('[');
        f(self);
        self.builder.source.push(']');

        if self.index != old_index {
            panic!("the pointer index unexpectedly changed in a [...] loop");
        }
    }

    /// Repeats the commands inside while the cell at `location` is nonzero. Guarantees that the pointer is at cell `location` at the beginning of each loop iteration.
    pub fn repeat_at(&mut self, location: usize, f: impl FnOnce(&mut Self)) {
        self.goto(location);
        self.builder.source.push('[');
        f(self);
        self.goto(location);
        self.builder.source.push(']');
    }
}

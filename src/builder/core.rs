//! A builder that implements very basic operations.

use crate::compiler::Program;

#[derive(Debug)]
/// A builder that implementes very basic operations.
pub struct Builder {
    pub(super) source: String,
}

impl Builder {
    /// Constructs a new `Builder`.
    pub fn new() -> Self {
        Self {
            source: String::new(),
        }
    }

    /// Compiles the source code of this builder, returning an error if the source is malformed.
    pub fn compile(&self) -> Result<Program, &'static str> {
        Program::new(&self.source)
    }

    /// Gets the source code of this builder.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Adds an increment command.
    pub fn inc(&mut self) {
        self.source += "+";
    }

    /// Adds several increment commands, or several decrement commands if `value > 128`.
    pub fn inc_by(&mut self, value: u8) {
        if value > 128 {
            for _ in 0..=255 - value {
                self.source += "-";
            }
        } else {
            for _ in 0..value {
                self.source += "+";
            }
        }
    }

    /// Adds a decrement command.
    pub fn dec(&mut self) {
        self.source += "-";
    }

    /// Adds several decrement commands, or several increment commands if `value > 128`.
    pub fn dec_by(&mut self, value: u8) {
        if value > 128 {
            for _ in 0..=255 - value {
                self.source += "+";
            }
        } else {
            for _ in 0..value {
                self.source += "-";
            }
        }
    }

    /// Adds a command to move the pointer left.
    pub fn shl(&mut self) {
        self.source += "<";
    }

    /// Adds a command to move the pointer right.
    pub fn shr(&mut self) {
        self.source += ">";
    }

    /// Adds a command to read input into the current cell.
    pub fn read(&mut self) {
        self.source += ",";
    }

    /// Adds a command to write the current cell into output.
    pub fn write(&mut self) {
        self.source += ".";
    }

    /// Repeats the commands inside while the current cell is nonzero.
    pub fn repeat(&mut self, f: impl FnOnce(&mut Self)) {
        self.source.push('[');
        f(self);
        self.source.push(']');
    }
}

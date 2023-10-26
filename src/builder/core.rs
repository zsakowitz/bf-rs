use crate::compiler::Program;

#[derive(Debug)]
pub struct Builder {
    pub(super) source: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            source: String::new(),
        }
    }

    pub fn compile(&self) -> Result<Program, &'static str> {
        Program::new(&self.source)
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn inc(&mut self) {
        self.source += "+";
    }

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

    pub fn dec(&mut self) {
        self.source += "-";
    }

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

    pub fn shl(&mut self) {
        self.source += "<";
    }

    pub fn shr(&mut self) {
        self.source += ">";
    }

    pub fn read(&mut self) {
        self.source += ",";
    }

    pub fn write(&mut self) {
        self.source += ".";
    }

    pub fn repeat(&mut self, f: impl FnOnce(&mut Self)) {
        self.source.push('[');
        f(self);
        self.source.push(']');
    }
}

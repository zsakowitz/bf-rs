use std::fmt::Debug;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Runner<const N: usize> {
    data: [u8; N],
    index: usize,
    input: Vec<u8>,
    output: Vec<u8>,
}

impl<const N: usize> Runner<N> {
    pub fn new(input: &[u8]) -> Self {
        if N == 0 {
            panic!("cannot make a runner of size 0");
        }

        let mut input = input.to_vec();
        input.reverse();

        Runner {
            data: [0; N],
            index: 0,
            input,
            output: Vec::new(),
        }
    }

    pub fn inc(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_add(1);
    }

    pub fn dec(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_sub(1);
    }

    pub fn shl(&mut self) {
        if self.index == 0 {
            self.index = N - 1
        } else {
            self.index -= 1
        }
    }

    pub fn shr(&mut self) {
        if self.index == N - 1 {
            self.index = 0
        } else {
            self.index += 1
        }
    }

    pub fn read(&mut self) {
        self.data[self.index] = self.input.pop().unwrap_or(0);
    }

    pub fn write(&mut self) {
        self.output.push(self.data[self.index]);
    }

    pub fn repeat(&mut self, mut f: impl FnMut(&mut Self)) {
        let initial_index = self.index;

        while self.data[self.index] != 0 {
            f(self);

            if self.index != initial_index {
                panic!("the pointer index unexpectedly changed in a [...] loop");
            }
        }
    }
}

struct RunnerData<'a>(&'a [u8], usize);

impl Debug for RunnerData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let mut output = if i == self.1 {
                        "<".to_string()
                    } else {
                        String::new()
                    };

                    output.push(
                        "0123456789ABCDEF"
                            .chars()
                            .nth(*v as usize / 16)
                            .expect("there will always be a character here"),
                    );

                    output.push(
                        "0123456789ABCDEF"
                            .chars()
                            .nth(*v as usize % 16)
                            .expect("there will always be a character here"),
                    );

                    if i == self.1 {
                        output.push('>');
                    }

                    output
                })
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
        )
    }
}

impl<const N: usize> Debug for Runner<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DEBUG_DATA_WIDTH: i32 = 8;

        f.debug_struct("Runner")
            .field(
                "data",
                &RunnerData(
                    &self.data[0i32.max(self.index as i32 - DEBUG_DATA_WIDTH) as usize
                        ..(N as i32).min(self.index as i32 + DEBUG_DATA_WIDTH) as usize],
                    self.index - 0i32.max(self.index as i32 - DEBUG_DATA_WIDTH) as usize,
                ),
            )
            .field("input", &self.input)
            .field("output", &self.output)
            .finish()
    }
}

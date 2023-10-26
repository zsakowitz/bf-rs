//! Hi

#![deny(missing_docs)]

use crate::builder::allocator::{core::AllocatingBuilder, ops::eq::PartialEq};

pub mod builder;
pub mod compiler;
pub mod runner;

fn main() -> Result<(), &'static str> {
    let builder = AllocatingBuilder::<30_000>::new();

    let char_h = builder.u8('h' as u8);
    let char_e = builder.u8('e' as u8);
    let char_l = builder.u8('l' as u8);
    let char_o = builder.u8('o' as u8);

    let output = || {
        char_h.write();
        char_e.write();
        char_l.write();
        char_l.write();
        char_o.write();
    };

    let value = builder.u8('l' as u8);
    value.eq('l' as u8).if_true(output);

    let runner = builder.run(b"")?;
    println!("{runner:?}");

    Ok(())
}

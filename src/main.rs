//! Hi

#![deny(missing_docs)]

use crate::builder::allocator::core::AllocatingBuilder;

pub mod builder;
pub mod compiler;
pub mod runner;

fn main() -> Result<(), &'static str> {
    let builder = AllocatingBuilder::<30_000>::new();

    let mut a = builder.u8(4);
    let b = builder.u8(7);
    a *= b;

    let runner = builder.run(b"")?;
    println!("{runner:?}");

    Ok(())
}

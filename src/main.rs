#![doc = ""]
#![deny(missing_docs)]

use crate::builder::allocator::core::AllocatingBuilder;

pub mod builder;
pub mod compiler;
pub mod runner;

fn main() -> Result<(), &'static str> {
    let builder = AllocatingBuilder::<65536>::new();

    let a = builder.u8(32);
    let mut b = builder.u8(7);
    b *= a;

    println!("{}", builder.source());
    println!("{:#?}", builder.run(b"hello")?);

    Ok(())
}

/*!
A crate for working with [Lindenmayer systems](https://en.wikipedia.org/wiki/L-system).

# Background

An L-System consists of an alphabet of symbols that can be used to make strings,
a collection of production rules that expand each symbol into a larger string of symbols,
an initial axiom string from which to begin construction, and a mechanism for transforming
the generated strings into geometric structures.

# Algae example
Lindenmayer's original L-System for modelling the growth of Algae had
variables `A` and `B`, axiom `A`, and production rules `A -> AB`, `B -> A`.  Iterating
this system produces the following output:

0. `A`
1. `AB`
2. `ABA`
3. `ABAAB`

# Basic usage

Put the following in your `Cargo.toml`:

```toml
dcc-lsystem = "0.7"
```

## [`LSystemBuilder`]

An L-system is represented by an instance of [`LSystem`].  To create a barebones [`LSystem`],
the [`LSystemBuilder`] struct is useful.  The following example shows an implementation of
Lindenmayer's Algae system.

```rust
use dcc_lsystem::{LSystemBuilder, LSystemError};

fn main() -> Result<(), LSystemError> {
    let mut builder = LSystemBuilder::new();

    // Set up the two tokens we use for our system.
    let a = builder.token("A")?;
    let b = builder.token("B")?;

    // Set up our axiom (i.e. initial state)
    builder.axiom(vec![a])?;

    // Set the transformation rules
    builder.transformation_rule(a, vec![a,b])?; // A -> AB
    builder.transformation_rule(b, vec![a])?;   // B -> A

    // Build our LSystem, which should have initial state A
    let mut system = builder.finish()?;
    assert_eq!(system.render(), "A");

    // system.step() applies our production rules a single time
    system.step();
    assert_eq!(system.render(), "AB");

    system.step();
    assert_eq!(system.render(), "ABA");

    // system.step_by() applies our production rule a number of times
    system.step_by(5);
    assert_eq!(system.render(), "ABAABABAABAABABAABABAABAABABAABAAB");

    Ok(())
}
```
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
*/

extern crate self as dcc_lsystem;

pub use arena::{Arena, ArenaId};
pub use builder::LSystemBuilder;
pub use errors::LSystemError;
pub use system::LSystem;

pub mod arena;
pub mod builder;
pub mod errors;
pub mod system;
pub mod token;

#[cfg(test)]
mod tests;

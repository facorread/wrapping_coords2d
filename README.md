# wrapping_coords2d
Rust crate to translate between 1D indices and 2D coordinates with wrapping.

[![Crate](https://img.shields.io/crates/v/wrapping_coords2d.svg)](https://crates.io/crates/wrapping_coords2d)
[![Downloads](https://img.shields.io/crates/d/wrapping_coords2d.svg)](https://crates.io/crates/wrapping_coords2d)
[![Documentation](https://docs.rs/wrapping_coords2d/badge.svg)](https://docs.rs/wrapping_coords2d)
[![License](https://img.shields.io/crates/l/wrapping_coords2d.svg)](https://www.apache.org/licenses/LICENSE-2.0)

Use [`WrappingCoords2d`](https://docs.rs/wrapping_coords2d/latest/wrapping_coords2d/struct.WrappingCoords2d.html) to store data from a 2D grid into a 1D container such as `std::vec::Vec`. Both x and y coordinates wrap around the limits of the grid. `WrappingCoords2d` is not a container; it is just a tool to manipulate indices. For a 2D container, see [`array2d`](https://docs.rs/array2d/latest/array2d/). For coordinate translation without wrapping, see [`ameda`](https://docs.rs/ameda/latest/ameda).

`WrappingCoords2d` is useful to design cellular automata, agent-based models, and game worlds in 2D and 3D. You can use `WrappingCoords2d` as part of an [Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) software architecture for high-performing models and flexible games. See my [ABM project](https://github.com/facorread/rust-agent-based-models) for an example.

See more examples in the documentation for the [`wrapping_coords2d`](https://docs.rs/wrapping_coords2d/latest/wrapping_coords2d/) crate.

# FAQ

## Why not create iterators to the neighbors of a cell?

Indices make more sense than iterators in an ECS design. [It's generally more idiomatic] to use a `for` loop with indices than iterator chains. If a game world or a model landscape has several components, such as color, depth, and fertility, it makes sense to keep each component in its own vector. The simplest approach is to use indices on them. However, I am open to a pull request contributing code for iterators into neighbors.

[It's generally more idiomatic]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each

## Does this crate use the `unsafe` keyword?

No.

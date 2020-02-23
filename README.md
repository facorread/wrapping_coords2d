# wrapping_coords2d
Rust crate to translate 2D coordinates into a 1D index with wrapping.
[![Crate](https://img.shields.io/crates/v/wrapping_coords2d.svg)](https://crates.io/crates/wrapping_coords2d)
[![Downloads](https://img.shields.io/crates/d/wrapping_coords2d.svg)](https://crates.io/crates/wrapping_coords2d)
[![Documentation](https://docs.rs/wrapping_coords2d/badge.svg)](https://docs.rs/wrapping_coords2d)
![License](https://img.shields.io/crates/l/wrapping_coords2d.svg)

Use [`WrappingCoords2d`](https://docs.rs/wrapping_coords2d/latest/wrapping_coords2d/struct.WrappingCoords2d.html) to store data from a 2D grid into a 1D container such as `std::vec::Vec`. Both x and y coordinates wrap around the limits of the grid. `WrappingCoords2d` is not a container; it is just a tool to manipulate indices. For a 2D container, see [`array2d`](https://docs.rs/array2d/latest/array2d/). For coordinate translation without wrapping, see [`ameda`](https://docs.rs/ameda/latest/ameda).

`WrappingCoords2d` is useful to design cellular automata and agent-based models. You can use `WrappingCoords2d` as part of an [Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) software architecture for high-performing, flexible models. See my [ABM project](https://github.com/facorread/rust-agent-based-models) for an example.

See more examples in the documentation for the [`wrapping_coords2d`](https://docs.rs/wrapping_coords2d/latest/wrapping_coords2d/) crate.

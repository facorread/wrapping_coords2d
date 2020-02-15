# WrappingCoords2d
Rust crate to translate 2D coordinates into a 1D index with wrapping

Use WrappingCoords2d to store data from a 2D grid into a 1D container such as `std::vec::Vec`. Index operations wrap around in both dimensions. This is not a container; it is just a tool to manipulate indices. For a 2D container, see [`array2d](https://docs.rs/array2d/latest/array2d/. For coordinate translation without wrapping, see [`ameda`](https://docs.rs/ameda/latest/ameda/).

WrappingCoords2d is useful to design cellular automata and agent-based models. See my [ABM project](https://github.com/facorread/rust-agent-based-models).

Index operations wrap around in both dimensions. See the examples in the documentation for [`WrappingCoords2d`](https://docs.rs/WrappingCoords2d/latest/WrappingCoords2d/).

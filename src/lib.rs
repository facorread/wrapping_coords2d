/* WrappingCoords2d: Rust crate to translate between 1D indices and 2D coordinates with wrapping https://crates.io/crates/wrapping_coords2d

    Copyright 2020 Fabio A. Correa Duran facorread@gmail.com

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#![doc(html_root_url = "https://docs.rs/wrapping_coords2d/0.1.9")]

//! Rust crate to translate between 1D indices and 2D coordinates with wrapping https://crates.io/crates/wrapping_coords2d
//!
//! Use [`WrappingCoords2d`](https://docs.rs/wrapping_coords2d/latest/wrapping_coords2d/struct.WrappingCoords2d.html)
//! to store data from a 2D grid into a 1D container such as `std::vec::Vec`.
//! Both x and y coordinates wrap around the limits of the grid.
//! `WrappingCoords2d` is not a container; it is just a tool to manipulate indices.
//! For a 2D container, see [`array2d`](https://docs.rs/array2d/latest/array2d/).
//! For coordinate translation without wrapping, see [`ameda`](https://docs.rs/ameda/latest/ameda).
//!
//! `WrappingCoords2d` is useful to design cellular automata and agent-based models.
//! You can use `WrappingCoords2d` as part of an [Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system)
//! software architecture for high-performing, flexible models.
//! See my [ABM project](https://github.com/facorread/rust-agent-based-models) for an example.
//!
//! # Examples
//!
//! ```
//! use wrapping_coords2d::WrappingCoords2d;
//! let w2d = WrappingCoords2d::new(10, 10).unwrap();
//! // Here are some basic coordinates:
//! assert_eq!(w2d.coords(0), (0, 0));
//! assert_eq!(w2d.coords(1), (1, 0));
//! assert_eq!(w2d.coords(10), (0, 1));
//! assert_eq!(w2d.coords(11), (1, 1));
//! assert_eq!(w2d.coords(90), (0, 9));
//! assert_eq!(w2d.coords(91), (1, 9));
//! // Here are the cell at (5, 9) and its 8 neighbors, counterclockwise, starting from the right neighbor:
//! assert_eq!(w2d.index(5, 9), 95);
//! assert_eq!(w2d.index(6, 9), 96);
//! assert_eq!(w2d.index(6, 0), 6);
//! assert_eq!(w2d.index(5, 0), 5);
//! assert_eq!(w2d.index(4, 0), 4);
//! assert_eq!(w2d.index(4, 9), 94);
//! assert_eq!(w2d.index(4, 8), 84);
//! assert_eq!(w2d.index(5, 8), 85);
//! assert_eq!(w2d.index(6, 8), 86);
//! // Here are the cell at (0, 0) and its 8 neighbors, counterclockwise, starting from the right neighbor:
//! assert_eq!(w2d.index(0, 0), 0);
//! assert_eq!(w2d.index(1, 0), 1);
//! assert_eq!(w2d.index(1, 1), 11);
//! assert_eq!(w2d.index(0, 1), 10);
//! assert_eq!(w2d.index(-1, 1), 19);
//! assert_eq!(w2d.index(-1, 0), 9);
//! assert_eq!(w2d.index(-1, -1), 99);
//! assert_eq!(w2d.index(0, -1), 90);
//! assert_eq!(w2d.index(1, -1), 91);
//! // Here are the 8 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
//! assert_eq!(w2d.shift(95, 1, 0), 96);
//! assert_eq!(w2d.shift(95, 1, 1), 6);
//! assert_eq!(w2d.shift(95, 0, 1), 5);
//! assert_eq!(w2d.shift(95, -1, 1), 4);
//! assert_eq!(w2d.shift(95, -1, 0), 94);
//! assert_eq!(w2d.shift(95, -1, -1), 84);
//! assert_eq!(w2d.shift(95, 0, -1), 85);
//! assert_eq!(w2d.shift(95, 1, -1), 86);
//! // Here are the 8 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
//! assert_eq!(w2d.shift(0, 1, 0), 1);
//! assert_eq!(w2d.shift(0, 1, 1), 11);
//! assert_eq!(w2d.shift(0, 0, 1), 10);
//! assert_eq!(w2d.shift(0, -1, 1), 19);
//! assert_eq!(w2d.shift(0, -1, 0), 9);
//! assert_eq!(w2d.shift(0, -1, -1), 99);
//! assert_eq!(w2d.shift(0, 0, -1), 90);
//! assert_eq!(w2d.shift(0, 1, -1), 91);
//! ```

/// Represents errors in the construction of a 2D grid.
#[derive(Debug)]
pub enum ErrorKind {
    /// `width` or `height` less than 1.
    DimensionsLessThan1,
    /// The product of `width` and `height` exceeds `std::i32::MAX`.
    DimensionsTooLarge,
}

impl std::error::Error for ErrorKind {}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ErrorKind::DimensionsLessThan1 => write!(f, "width or height less than 1"),
            ErrorKind::DimensionsTooLarge => write!(f, "the product of width and height exceeds std::i32::MAX = {}", std::i32::MAX),
        }
    }
}

/// Represents a 2D grid with wrapping.
#[derive(Debug, PartialEq)]
pub struct WrappingCoords2d {
    /// Width of the grid; it has to be larger than 0.
    w32: i32,
    /// Height of the grid; it has to be larger than 0.
    h32: i32,
    /// Total number of cells in the grid; it has to be larger than 0 and smaller than std::i32::MAX.
    sz32: i32,
    /// Width of the grid.
    wu: usize,
    /// Total number of cells in the grid.
    szu: usize,
}

impl WrappingCoords2d {
    /// Constructs a new WrappingCoords2d object.
    ///
    /// # Errors
    ///
    /// Both `width` and `height` must be larger than 0. Also, their product must be smaller than `std::i32::MAX = 2147483647`.
    /// Generally speaking, [`i32` is the fastest] integer type, even on 64-bit systems. `i32` is sufficient for a wide range
    /// of agent-based models. You will need to modify the data type to accommodate larger landscapes.
    ///
    /// As an example, the largest square grid that a `WrappingCoords2d` object can accommodate has a size of `46340x46340` cells,
    /// or approximately the square root of `std::i32::MAX`. For a property that needs an `i32` representation,
    /// the program needs to allocate `std::i32::MAX * 4 = 8GiB` of RAM.
    ///
    /// [`i32` is the fastest]: https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types
    pub fn new(width: i32, height: i32) -> Result<WrappingCoords2d, ErrorKind> {
        if width > 0 && height > 0 {
            match width.checked_mul(height) {
                Some(s) => Ok(WrappingCoords2d {
                    w32: width,
                    h32: height,
                    sz32: s,
                    wu: width as usize,
                    szu: s as usize,
                }),
                None => Err(ErrorKind::DimensionsTooLarge),
            }
        } else {
            Err(ErrorKind::DimensionsLessThan1)
        }
    }
    /// Returns the width of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// assert_eq!(w2d.width(), 10);
    /// ```
    pub fn width(&self) -> i32 {
        self.w32
    }
    /// Returns the height of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// assert_eq!(w2d.height(), 10);
    /// ```
    pub fn height(&self) -> i32 {
        self.h32
    }
    /// Returns the total number of cells in the grid. Use this to initialize 1D containers.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// assert_eq!(w2d.size(), 100 as usize);
    /// ```
    pub fn size(&self) -> usize {
        self.szu
    }
    /// Returns the total number of cells in the grid as an `i32` number.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// assert_eq!(w2d.size32(), 100);
    /// ```
    pub fn size32(&self) -> i32 {
        self.sz32
    }
    /// Returns the Euclidean modulo, a non-negative number.
    /// This operation is also available in the [`DivRem`](https://crates.io/crates/divrem) crate.
    /// In contrast, the standard modulo operator can be negative; for example, `-11 % 10 = -1`.
    ///
    /// Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// assert_eq!(-11 % 10, -1);
    /// assert_eq!(WrappingCoords2d::modulo(-11, 10), 9);
    /// ```
    pub fn modulo(lhs: i32, rhs: i32) -> i32 {
        let mut res = lhs % rhs;
        if res < 0 {
            res += rhs;
        }
        res
    }
    /// Returns an index into the grid based on x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the cell at (5, 9) and its 8 neighbors, counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.index(5, 9), 95);
    /// assert_eq!(w2d.index(6, 9), 96);
    /// assert_eq!(w2d.index(6, 0), 6);
    /// assert_eq!(w2d.index(5, 0), 5);
    /// assert_eq!(w2d.index(4, 0), 4);
    /// assert_eq!(w2d.index(4, 9), 94);
    /// assert_eq!(w2d.index(4, 8), 84);
    /// assert_eq!(w2d.index(5, 8), 85);
    /// assert_eq!(w2d.index(6, 8), 86);
    /// // Here are the cell at (0, 0) and its 8 neighbors, counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.index(0, 0), 0);
    /// assert_eq!(w2d.index(1, 0), 1);
    /// assert_eq!(w2d.index(1, 1), 11);
    /// assert_eq!(w2d.index(0, 1), 10);
    /// assert_eq!(w2d.index(-1, 1), 19);
    /// assert_eq!(w2d.index(-1, 0), 9);
    /// assert_eq!(w2d.index(-1, -1), 99);
    /// assert_eq!(w2d.index(0, -1), 90);
    /// assert_eq!(w2d.index(1, -1), 91);
    /// ```
    pub fn index(&self, x: i32, y: i32) -> usize {
        let mx = WrappingCoords2d::modulo(x, self.w32);
        let myw = WrappingCoords2d::modulo(y * self.w32, self.sz32);
        (myw + mx) as usize
    }
    /// Returns `x` and `y` coordinates based on an `index` into the 1D container.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are some basic coordinates:
    /// assert_eq!(w2d.coords(0), (0, 0));
    /// assert_eq!(w2d.coords(1), (1, 0));
    /// assert_eq!(w2d.coords(10), (0, 1));
    /// assert_eq!(w2d.coords(11), (1, 1));
    /// assert_eq!(w2d.coords(90), (0, 9));
    /// assert_eq!(w2d.coords(91), (1, 9));
    pub fn coords(&self, index: usize) -> (i32, i32) {
        let idx32 = index as i32; // Always positive
        (idx32 % self.w32, idx32 / self.h32)
    }
    /// Returns a new index into the grid based on a starting index `start_index`, an x offset, and a y offset.
    /// `delta_x` and `delta_y` can be negative.
    ///
    /// # Safety
    ///
    /// This function does not check that `start_index` is a valid index. However, it returns a valid index in the range [0, size).
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 8 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.shift(95, 1, 0), 96);
    /// assert_eq!(w2d.shift(95, 1, 1), 6);
    /// assert_eq!(w2d.shift(95, 0, 1), 5);
    /// assert_eq!(w2d.shift(95, -1, 1), 4);
    /// assert_eq!(w2d.shift(95, -1, 0), 94);
    /// assert_eq!(w2d.shift(95, -1, -1), 84);
    /// assert_eq!(w2d.shift(95, 0, -1), 85);
    /// assert_eq!(w2d.shift(95, 1, -1), 86);
    /// // Here are the 8 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.shift(0, 1, 0), 1);
    /// assert_eq!(w2d.shift(0, 1, 1), 11);
    /// assert_eq!(w2d.shift(0, 0, 1), 10);
    /// assert_eq!(w2d.shift(0, -1, 1), 19);
    /// assert_eq!(w2d.shift(0, -1, 0), 9);
    /// assert_eq!(w2d.shift(0, -1, -1), 99);
    /// assert_eq!(w2d.shift(0, 0, -1), 90);
    /// assert_eq!(w2d.shift(0, 1, -1), 91);
    /// ```
    pub fn shift(&self, start_index: usize, delta_x: i32, delta_y: i32) -> usize {
        // Note: -11 % 10 = -1
        let index = start_index as i32;
        let x = index % self.w32; // Always positive
        let new_x = WrappingCoords2d::modulo(x + delta_x, self.w32); // Positive number
        let yw = index - x; // yw: The y coordinate times the width; always positive
        let new_yw = WrappingCoords2d::modulo(yw + delta_y * self.w32, self.sz32); // Positive number
        (new_yw + new_x) as usize
    }
    /// This function takes the cell given by `start_index` and returns a vector of the indices to its 4 neighbors,
    /// the so-called von Neumann neighborhood or 4-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Safety
    ///
    /// This function does not check that `start_index` is a valid index. However, it returns valid indices in the range [0, size).
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 4 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors4(95), vec![96, 5, 94, 85]);
    /// // Here are the 4 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors4(0), vec![1, 10, 9, 90]);
    /// ```
    pub fn neighbors4(&self, start_index: usize) -> std::vec::Vec<usize> {
        // Note: -11 % 10 = -1
        let idx = start_index as i32;
        let x = idx % self.w32; // Always positive
        let yw = idx - x; // yw: The y coordinate times the width; always positive
        let mut result32 = vec![x; 4];
        result32[0] = (x + 1) % self.w32 + yw; // Neighbor to the right; modulo is always positive
        result32[1] = (idx + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[2] = WrappingCoords2d::modulo(x - 1, self.w32) + yw; // Neighbor to the left
        result32[3] = WrappingCoords2d::modulo(idx - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32.into_iter().map(|index| index as usize).collect()
    }
    /// This function takes the cell given by `(start_x, start_y)` and returns a vector of the indices to its 4 neighbors,
    /// the so-called von Neumann neighborhood or 4-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 4 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors4xy(5, 9), vec![96, 5, 94, 85]);
    /// // Here are the 4 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors4xy(0, 0), vec![1, 10, 9, 90]);
    /// ```
    pub fn neighbors4xy(&self, start_x: i32, start_y: i32) -> std::vec::Vec<usize> {
        self.neighbors4(self.index(start_x, start_y))
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and the neighbors defined by `x_shifts` and `yw_shifts`.
    /// This function remains private because `neighbor_shifts` has value restrictions. These coordinates must have
    /// the form (x + w, wy + sz) which elliminates the need to convert between usize and i32.
    /// Despite the increased RAM bandwidth usage, this function keeps `usize` operations to a minimum. `usize` is necessary to prevent
    /// overflow in very large worlds and 32 bit environments.
    ///
    /// # Safety
    ///
    /// This function does not check that `x_shifts` and `yw_shifts` have the same length.
    fn for_each<F>(
        &self,
        mut f: F,
        mut x_shifts: std::vec::Vec<usize>,
        mut yw_shifts: std::vec::Vec<usize>,
    ) where
        F: FnMut(usize, &std::vec::Vec<usize>),
    {
        // Reusing`x_shifts` and `yw_shifts`
        let x_shifts0 = x_shifts.clone();
        let mut x = x_shifts.clone();
        let mut yw = yw_shifts.clone();
        let mut neighbors = vec![0; x_shifts.len()];
        let mut this_cell_index = 0;
        loop {
            for j in 0..neighbors.len() {
                x[j] = x_shifts[j] % self.wu;
                yw[j] = yw_shifts[j] % self.szu;
                neighbors[j] = yw[j] + x[j];
            }
            // Evaluation
            f(this_cell_index, &neighbors);
            // Next iteration
            this_cell_index += 1;
            if this_cell_index == self.szu {
                break;
            }
            // Locate the neighbors
            if this_cell_index % self.wu == 0 {
                x_shifts = x_shifts0.clone();
                for j in yw_shifts.iter_mut() {
                    *j += self.wu;
                }
            } else {
                for j in x_shifts.iter_mut() {
                    *j += 1;
                }
            }
        }
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and its 4 neighbors,
    /// the so-called von Neumann neighborhood or 4-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each4(|this_cell_index, neighbors| {
    ///     assert_eq!(neighbors[0], w2d.shift(this_cell_index, 1, 0));
    ///     assert_eq!(neighbors[1], w2d.shift(this_cell_index, 0, 1));
    ///     assert_eq!(neighbors[2], w2d.shift(this_cell_index, -1, 0));
    ///     assert_eq!(neighbors[3], w2d.shift(this_cell_index, 0, -1));
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, w2d.size());
    /// ```
    pub fn for_each4<F>(&self, f: F)
    where
        F: FnMut(usize, &std::vec::Vec<usize>),
    {
        let wp1 = self.wu + 1;
        let wm1 = self.wu - 1;
        let spw = self.szu + self.wu;
        let smw = self.szu - self.wu;
        self.for_each(
            f,
            vec![wp1, self.wu, wm1, self.wu],
            vec![self.szu, spw, self.szu, smw],
        )
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and one of its 4 neighbors,
    /// the so-called von Neumann neighborhood or 4-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each_pair4(|this_cell_index, neighbor_index| {
    ///     assert!(this_cell_index != neighbor_index);
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, 4 * w2d.size());
    /// ```
    pub fn for_each_pair4<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        self.for_each4(|this_cell_index, neighbors| {
            for &neighbor_index in neighbors {
                f(this_cell_index, neighbor_index);
            }
        });
    }
    /// This function takes the cell given by `start_index` and returns a vector of the indices to its 8 neighbors,
    /// the so-called Moore neighborhood or 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Safety
    ///
    /// This function does not check that `start_index` is a valid index. However, it returns valid indices in the range [0, size).
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 8 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors8(95), vec![96, 6, 5, 4, 94, 84, 85, 86]);
    /// // Here are the 8 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors8(0), vec![1, 11, 10, 19, 9, 99, 90, 91]);
    /// ```
    pub fn neighbors8(&self, start_index: usize) -> std::vec::Vec<usize> {
        // Note: -11 % 10 = -1
        let idx = start_index as i32;
        let x = idx % self.w32; // Always positive
        let yw = idx - x; // yw: The y coordinate times the width; always positive
        let idxr1 = (x + 1) % self.w32 + yw; // Index of the first neighbor, the one to the right; modulo is always positive
        let idxl1 = WrappingCoords2d::modulo(x - 1, self.w32) + yw; // Index of the fourth neighbor, the one to the left; modulo is always positive
        let mut result32 = vec![idxr1; 8];
        result32[1] = (idxr1 + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[2] = (idx + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[3] = (idxl1 + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[4] = idxl1;
        result32[5] = WrappingCoords2d::modulo(idxl1 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[6] = WrappingCoords2d::modulo(idx - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[7] = WrappingCoords2d::modulo(idxr1 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32.into_iter().map(|index| index as usize).collect()
    }
    /// This function takes the cell given by `(start_x, start_y)` and returns a vector of the indices to its 8 neighbors,
    /// the so-called Moore neighborhood or 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 8 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors8xy(5, 9), vec![96, 6, 5, 4, 94, 84, 85, 86]);
    /// // Here are the 8 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors8xy(0, 0), vec![1, 11, 10, 19, 9, 99, 90, 91]);
    /// ```
    pub fn neighbors8xy(&self, start_x: i32, start_y: i32) -> std::vec::Vec<usize> {
        self.neighbors8(self.index(start_x, start_y))
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and its 8 neighbors,
    /// the so-called Moore neighborhood or 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each8(|this_cell_index, neighbors| {
    ///     assert_eq!(neighbors[0], w2d.shift(this_cell_index, 1, 0));
    ///     assert_eq!(neighbors[1], w2d.shift(this_cell_index, 1, 1));
    ///     assert_eq!(neighbors[2], w2d.shift(this_cell_index, 0, 1));
    ///     assert_eq!(neighbors[3], w2d.shift(this_cell_index, -1, 1));
    ///     assert_eq!(neighbors[4], w2d.shift(this_cell_index, -1, 0));
    ///     assert_eq!(neighbors[5], w2d.shift(this_cell_index, -1, -1));
    ///     assert_eq!(neighbors[6], w2d.shift(this_cell_index, 0, -1));
    ///     assert_eq!(neighbors[7], w2d.shift(this_cell_index, 1, -1));
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, w2d.size());
    /// ```
    pub fn for_each8<F>(&self, f: F)
    where
        F: FnMut(usize, &std::vec::Vec<usize>),
    {
        let wp1 = self.wu + 1;
        let wm1 = self.wu - 1;
        let spw = self.szu + self.wu;
        let smw = self.szu - self.wu;
        self.for_each(
            f,
            vec![wp1, wp1, self.wu, wm1, wm1, wm1, self.wu, wp1],
            vec![self.szu, spw, spw, spw, self.szu, smw, smw, smw],
        )
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and one of its 8 neighbors,
    /// the so-called Moore neighborhood or 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the neighbor to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each_pair8(|this_cell_index, neighbor_index| {
    ///     assert!(this_cell_index != neighbor_index);
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, 8 * w2d.size());
    /// ```
    pub fn for_each_pair8<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        self.for_each8(|this_cell_index, neighbors| {
            for &neighbor_index in neighbors {
                f(this_cell_index, neighbor_index);
            }
        });
    }
    /// This function takes the cell given by `start_index` and returns a vector of the indices to its 16 second neighbors,
    /// which are adjacent to the cell's 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the second cell to the right.
    ///
    /// # Safety
    ///
    /// This function does not check that `start_index` is a valid index. However, it returns valid indices in the range [0, size).
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 16 second neighbors of the cell at (5, 9), counterclockwise, starting from the right:
    /// assert_eq!(w2d.neighbors16(95), vec![97, 7, 17, 16, 15, 14, 13, 3, 93, 83, 73, 74, 75, 76, 77, 87]);
    /// // Here are the 16 second neighbors of the cell at (0, 0), counterclockwise, starting from the right:
    /// assert_eq!(w2d.neighbors16(0), vec![2, 12, 22, 21, 20, 29, 28, 18, 8, 98, 88, 89, 80, 81, 82, 92]);
    /// ```
    pub fn neighbors16(&self, start_index: usize) -> std::vec::Vec<usize> {
        // Note: -11 % 10 = -1
        let idx = start_index as i32;
        let x = idx % self.w32; // Always positive
        let yw = idx - x; // yw: The y coordinate times the width; always positive
        let idxr2 = (x + 2) % self.w32 + yw; // Index of the first neighbor, the one to the right; modulo is always positive
        let idxr1 = (x + 1) % self.w32 + yw; // Index of the first neighbor, the one to the right; modulo is always positive
        let idxl1 = WrappingCoords2d::modulo(x - 1, self.w32) + yw; // Index of the fourth neighbor, the one to the left; modulo is always positive
        let idxl2 = WrappingCoords2d::modulo(x - 2, self.w32) + yw; // Index of the fourth neighbor, the one to the left; modulo is always positive
        let mut result32 = vec![idxr2; 16];
        result32[1] = (idxr2 + self.w32) % self.sz32;
        result32[2] = (idxr2 + 2 * self.w32) % self.sz32;
        result32[3] = (idxr1 + 2 * self.w32) % self.sz32;
        result32[4] = (idx + 2 * self.w32) % self.sz32;
        result32[5] = (idxl1 + 2 * self.w32) % self.sz32;
        result32[6] = (idxl2 + 2 * self.w32) % self.sz32;
        result32[7] = (idxl2 + self.w32) % self.sz32;
        result32[8] = idxl2;
        result32[9] = WrappingCoords2d::modulo(idxl2 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[10] = WrappingCoords2d::modulo(idxl2 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[11] = WrappingCoords2d::modulo(idxl1 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[12] = WrappingCoords2d::modulo(idx - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[13] = WrappingCoords2d::modulo(idxr1 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[14] = WrappingCoords2d::modulo(idxr2 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[15] = WrappingCoords2d::modulo(idxr2 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32.into_iter().map(|index| index as usize).collect()
    }
    /// This function takes the cell given by `(start_x, start_y)` and returns a vector of the indices to its 16 second neighbors,
    /// which are adjacent to the cell's 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the second cell to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 16 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors16xy(5, 9), vec![97, 7, 17, 16, 15, 14, 13, 3, 93, 83, 73, 74, 75, 76, 77, 87]);
    /// // Here are the 16 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors16xy(0, 0), vec![2, 12, 22, 21, 20, 29, 28, 18, 8, 98, 88, 89, 80, 81, 82, 92]);
    /// ```
    pub fn neighbors16xy(&self, start_x: i32, start_y: i32) -> std::vec::Vec<usize> {
        self.neighbors16(self.index(start_x, start_y))
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and its 16 second neighbors,
    /// which are adjacent to the cell's 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the second cell to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each16(|this_cell_index, neighbors| {
    ///     assert_eq!(neighbors[0], w2d.shift(this_cell_index, 2, 0));
    ///     assert_eq!(neighbors[1], w2d.shift(this_cell_index, 2, 1));
    ///     assert_eq!(neighbors[2], w2d.shift(this_cell_index, 2, 2));
    ///     assert_eq!(neighbors[3], w2d.shift(this_cell_index, 1, 2));
    ///     assert_eq!(neighbors[4], w2d.shift(this_cell_index, 0, 2));
    ///     assert_eq!(neighbors[5], w2d.shift(this_cell_index, -1, 2));
    ///     assert_eq!(neighbors[6], w2d.shift(this_cell_index, -2, 2));
    ///     assert_eq!(neighbors[7], w2d.shift(this_cell_index, -2, 1));
    ///     assert_eq!(neighbors[8], w2d.shift(this_cell_index, -2, 0));
    ///     assert_eq!(neighbors[9], w2d.shift(this_cell_index, -2, -1));
    ///     assert_eq!(neighbors[10], w2d.shift(this_cell_index, -2, -2));
    ///     assert_eq!(neighbors[11], w2d.shift(this_cell_index, -1, -2));
    ///     assert_eq!(neighbors[12], w2d.shift(this_cell_index, 0, -2));
    ///     assert_eq!(neighbors[13], w2d.shift(this_cell_index, 1, -2));
    ///     assert_eq!(neighbors[14], w2d.shift(this_cell_index, 2, -2));
    ///     assert_eq!(neighbors[15], w2d.shift(this_cell_index, 2, -1));
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, w2d.size());
    /// ```
    pub fn for_each16<F>(&self, f: F)
    where
        F: FnMut(usize, &std::vec::Vec<usize>),
    {
        let wp2 = self.wu + 2;
        let wp1 = self.wu + 1;
        let wm1 = self.wu - 1;
        let wm2 = self.wu - 2;
        let w2 = 2 * self.wu;
        let sp2 = self.szu + w2;
        let spw = self.szu + self.wu;
        let smw = self.szu - self.wu;
        let sm2 = self.szu - w2;
        self.for_each(
            f,
            vec![
                wp2, wp2, wp2, wp1, self.wu, wm1, wm2, wm2, wm2, wm2, wm2, wm1, self.wu, wp1, wp2,
                wp2,
            ],
            vec![
                self.szu, spw, sp2, sp2, sp2, sp2, sp2, spw, self.szu, smw, sm2, sm2, sm2, sm2,
                sm2, smw,
            ],
        )
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and one of its 16 second neighbors,
    /// which are adjacent to the cell's 8-neighborhood. The indices are ordered in 2D, counter-clockwise,
    /// starting from the second cell to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each_pair16(|this_cell_index, neighbor_index| {
    ///     assert!(this_cell_index != neighbor_index);
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, 16 * w2d.size());
    /// ```
    pub fn for_each_pair16<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        self.for_each16(|this_cell_index, neighbors| {
            for &neighbor_index in neighbors {
                f(this_cell_index, neighbor_index);
            }
        });
    }
    /// This function takes the cell given by `start_index` and returns a vector of the indices to its 24 nearest neighbors.
    /// The indices are ordered in 2D, counter-clockwise, starting with the cell to the right, going through the
    /// Moore neighborhood first, and then going through the second cell to the right, and ending with the second neighbors.
    ///
    /// # Safety
    ///
    /// This function does not check that `start_index` is a valid index. However, it returns valid indices in the range [0, size).
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 24 second neighbors of the cell at (5, 9), counterclockwise, starting from the right:
    /// assert_eq!(w2d.neighbors24(95), vec![96, 6, 5, 4, 94, 84, 85, 86, 97, 7, 17, 16, 15, 14, 13, 3, 93, 83, 73, 74, 75, 76, 77, 87]);
    /// // Here are the 24 second neighbors of the cell at (0, 0), counterclockwise, starting from the right:
    /// assert_eq!(w2d.neighbors24(0), vec![1, 11, 10, 19, 9, 99, 90, 91, 2, 12, 22, 21, 20, 29, 28, 18, 8, 98, 88, 89, 80, 81, 82, 92]);
    /// ```
    pub fn neighbors24(&self, start_index: usize) -> std::vec::Vec<usize> {
        // Note: -11 % 10 = -1
        let idx = start_index as i32;
        let x = idx % self.w32; // Always positive
        let yw = idx - x; // yw: The y coordinate times the width; always positive
        let idxr2 = (x + 2) % self.w32 + yw; // Index of the first neighbor, the one to the right; modulo is always positive
        let idxr1 = (x + 1) % self.w32 + yw; // Index of the first neighbor, the one to the right; modulo is always positive
        let idxl1 = WrappingCoords2d::modulo(x - 1, self.w32) + yw; // Index of the fourth neighbor, the one to the left; modulo is always positive
        let idxl2 = WrappingCoords2d::modulo(x - 2, self.w32) + yw; // Index of the fourth neighbor, the one to the left; modulo is always positive
        let mut result32 = vec![idxr1; 24];
        result32[1] = (idxr1 + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[2] = (idx + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[3] = (idxl1 + self.w32) % self.sz32; // Neighbor above; modulo is always positive
        result32[4] = idxl1;
        result32[5] = WrappingCoords2d::modulo(idxl1 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[6] = WrappingCoords2d::modulo(idx - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[7] = WrappingCoords2d::modulo(idxr1 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[8] = idxr2;
        result32[9] = (idxr2 + self.w32) % self.sz32;
        result32[10] = (idxr2 + 2 * self.w32) % self.sz32;
        result32[11] = (idxr1 + 2 * self.w32) % self.sz32;
        result32[12] = (idx + 2 * self.w32) % self.sz32;
        result32[13] = (idxl1 + 2 * self.w32) % self.sz32;
        result32[14] = (idxl2 + 2 * self.w32) % self.sz32;
        result32[15] = (idxl2 + self.w32) % self.sz32;
        result32[16] = idxl2;
        result32[17] = WrappingCoords2d::modulo(idxl2 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[18] = WrappingCoords2d::modulo(idxl2 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[19] = WrappingCoords2d::modulo(idxl1 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[20] = WrappingCoords2d::modulo(idx - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[21] = WrappingCoords2d::modulo(idxr1 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[22] = WrappingCoords2d::modulo(idxr2 - 2 * self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32[23] = WrappingCoords2d::modulo(idxr2 - self.w32, self.sz32); // Neighbor below; modulo is always positive
        result32.into_iter().map(|index| index as usize).collect()
    }
    /// This function takes the cell given by `(start_x, start_y)` and returns a vector of the indices to its 24 nearest neighbors.
    /// The indices are ordered in 2D, counter-clockwise, starting with the cell to the right, going through the
    /// Moore neighborhood first, and then going through the second cell to the right, and ending with the second neighbors.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// // Here are the 24 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors24xy(5, 9), vec![96, 6, 5, 4, 94, 84, 85, 86, 97, 7, 17, 16, 15, 14, 13, 3, 93, 83, 73, 74, 75, 76, 77, 87]);
    /// // Here are the 24 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
    /// assert_eq!(w2d.neighbors24xy(0, 0), vec![1, 11, 10, 19, 9, 99, 90, 91, 2, 12, 22, 21, 20, 29, 28, 18, 8, 98, 88, 89, 80, 81, 82, 92]);
    /// ```
    pub fn neighbors24xy(&self, start_x: i32, start_y: i32) -> std::vec::Vec<usize> {
        self.neighbors24(self.index(start_x, start_y))
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and its 24 nearest neighbors.
    /// The indices are ordered in 2D, counter-clockwise, starting with the cell to the right, going through the
    /// Moore neighborhood first, and then going through the second cell to the right, and ending with the second neighbors.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each24(|this_cell_index, neighbors| {
    ///     assert_eq!(neighbors[0], w2d.shift(this_cell_index, 1, 0));
    ///     assert_eq!(neighbors[1], w2d.shift(this_cell_index, 1, 1));
    ///     assert_eq!(neighbors[2], w2d.shift(this_cell_index, 0, 1));
    ///     assert_eq!(neighbors[3], w2d.shift(this_cell_index, -1, 1));
    ///     assert_eq!(neighbors[4], w2d.shift(this_cell_index, -1, 0));
    ///     assert_eq!(neighbors[5], w2d.shift(this_cell_index, -1, -1));
    ///     assert_eq!(neighbors[6], w2d.shift(this_cell_index, 0, -1));
    ///     assert_eq!(neighbors[7], w2d.shift(this_cell_index, 1, -1));
    ///     assert_eq!(neighbors[8], w2d.shift(this_cell_index, 2, 0));
    ///     assert_eq!(neighbors[9], w2d.shift(this_cell_index, 2, 1));
    ///     assert_eq!(neighbors[10], w2d.shift(this_cell_index, 2, 2));
    ///     assert_eq!(neighbors[11], w2d.shift(this_cell_index, 1, 2));
    ///     assert_eq!(neighbors[12], w2d.shift(this_cell_index, 0, 2));
    ///     assert_eq!(neighbors[13], w2d.shift(this_cell_index, -1, 2));
    ///     assert_eq!(neighbors[14], w2d.shift(this_cell_index, -2, 2));
    ///     assert_eq!(neighbors[15], w2d.shift(this_cell_index, -2, 1));
    ///     assert_eq!(neighbors[16], w2d.shift(this_cell_index, -2, 0));
    ///     assert_eq!(neighbors[17], w2d.shift(this_cell_index, -2, -1));
    ///     assert_eq!(neighbors[18], w2d.shift(this_cell_index, -2, -2));
    ///     assert_eq!(neighbors[19], w2d.shift(this_cell_index, -1, -2));
    ///     assert_eq!(neighbors[20], w2d.shift(this_cell_index, 0, -2));
    ///     assert_eq!(neighbors[21], w2d.shift(this_cell_index, 1, -2));
    ///     assert_eq!(neighbors[22], w2d.shift(this_cell_index, 2, -2));
    ///     assert_eq!(neighbors[23], w2d.shift(this_cell_index, 2, -1));
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, w2d.size());
    /// ```
    pub fn for_each24<F>(&self, f: F)
    where
        F: FnMut(usize, &std::vec::Vec<usize>),
    {
        let wp2 = self.wu + 2;
        let wp1 = self.wu + 1;
        let wm1 = self.wu - 1;
        let wm2 = self.wu - 2;
        let w2 = 2 * self.wu;
        let sp2 = self.szu + w2;
        let spw = self.szu + self.wu;
        let smw = self.szu - self.wu;
        let sm2 = self.szu - w2;
        self.for_each(
            f,
            vec![
                wp1, wp1, self.wu, wm1, wm1, wm1, self.wu, wp1, wp2, wp2, wp2, wp1, self.wu, wm1,
                wm2, wm2, wm2, wm2, wm2, wm1, self.wu, wp1, wp2, wp2,
            ],
            vec![
                self.szu, spw, spw, spw, self.szu, smw, smw, smw, self.szu, spw, sp2, sp2, sp2,
                sp2, sp2, spw, self.szu, smw, sm2, sm2, sm2, sm2, sm2, smw,
            ],
        )
    }
    /// Calls a closure `f` on each cell of the grid. Each call acts on the cell and one of its 24 nearest neighbors.
    /// The indices are ordered in 2D, counter-clockwise, starting with the cell to the right, going through the
    /// Moore neighborhood first, and then going through the second cell to the right, and ending with the second neighbors.
    ///
    /// # Examples
    ///
    /// ```
    /// use wrapping_coords2d::WrappingCoords2d;
    /// let w2d = WrappingCoords2d::new(10, 10).unwrap();
    /// let mut calls_counter = 0;
    /// w2d.for_each_pair24(|this_cell_index, neighbor_index| {
    ///     assert!(this_cell_index != neighbor_index);
    ///     calls_counter += 1;
    /// });
    /// assert_eq!(calls_counter, 24 * w2d.size());
    /// ```
    pub fn for_each_pair24<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        self.for_each24(|this_cell_index, neighbors| {
            for &neighbor_index in neighbors {
                f(this_cell_index, neighbor_index);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Basic examples - Copy over to the documentation section
        let w2d = WrappingCoords2d::new(10, 10).unwrap();
        // Here are some basic coordinates:
        assert_eq!(w2d.coords(0), (0, 0));
        assert_eq!(w2d.coords(1), (1, 0));
        assert_eq!(w2d.coords(10), (0, 1));
        assert_eq!(w2d.coords(11), (1, 1));
        assert_eq!(w2d.coords(90), (0, 9));
        assert_eq!(w2d.coords(91), (1, 9));
        // Here are the cell at (5, 9) and its 8 neighbors, counterclockwise, starting from the right neighbor:
        assert_eq!(w2d.index(5, 9), 95);
        assert_eq!(w2d.index(6, 9), 96);
        assert_eq!(w2d.index(6, 0), 6);
        assert_eq!(w2d.index(5, 0), 5);
        assert_eq!(w2d.index(4, 0), 4);
        assert_eq!(w2d.index(4, 9), 94);
        assert_eq!(w2d.index(4, 8), 84);
        assert_eq!(w2d.index(5, 8), 85);
        assert_eq!(w2d.index(6, 8), 86);
        // Here are the cell at (0, 0) and its 8 neighbors, counterclockwise, starting from the right neighbor:
        assert_eq!(w2d.index(0, 0), 0);
        assert_eq!(w2d.index(1, 0), 1);
        assert_eq!(w2d.index(1, 1), 11);
        assert_eq!(w2d.index(0, 1), 10);
        assert_eq!(w2d.index(-1, 1), 19);
        assert_eq!(w2d.index(-1, 0), 9);
        assert_eq!(w2d.index(-1, -1), 99);
        assert_eq!(w2d.index(0, -1), 90);
        assert_eq!(w2d.index(1, -1), 91);
        // Here are the 8 neighbors of the cell at (5, 9), counterclockwise, starting from the right neighbor:
        assert_eq!(w2d.shift(95, 1, 0), 96);
        assert_eq!(w2d.shift(95, 1, 1), 6);
        assert_eq!(w2d.shift(95, 0, 1), 5);
        assert_eq!(w2d.shift(95, -1, 1), 4);
        assert_eq!(w2d.shift(95, -1, 0), 94);
        assert_eq!(w2d.shift(95, -1, -1), 84);
        assert_eq!(w2d.shift(95, 0, -1), 85);
        assert_eq!(w2d.shift(95, 1, -1), 86);
        // Here are the 8 neighbors of the cell at (0, 0), counterclockwise, starting from the right neighbor:
        assert_eq!(w2d.shift(0, 1, 0), 1);
        assert_eq!(w2d.shift(0, 1, 1), 11);
        assert_eq!(w2d.shift(0, 0, 1), 10);
        assert_eq!(w2d.shift(0, -1, 1), 19);
        assert_eq!(w2d.shift(0, -1, 0), 9);
        assert_eq!(w2d.shift(0, -1, -1), 99);
        assert_eq!(w2d.shift(0, 0, -1), 90);
        assert_eq!(w2d.shift(0, 1, -1), 91);

        // End of basic examples
        // Easy-to-read tests; may repeat some basic examples
        assert_eq!(w2d.shift(0, 1, 0), 1);
        assert_eq!(w2d.shift(0, 11, 0), 1);
        assert_eq!(w2d.shift(0, -1, 0), 9);
        assert_eq!(w2d.shift(0, -11, 0), 9);
        assert_eq!(w2d.shift(10, 1, 0), 11);
        assert_eq!(w2d.shift(10, 11, 0), 11);
        assert_eq!(w2d.shift(10, -1, 0), 19);
        assert_eq!(w2d.shift(10, -11, 0), 19);
        assert_eq!(w2d.shift(0, 0, 1), 10);
        assert_eq!(w2d.shift(0, 0, 11), 10);
        assert_eq!(w2d.shift(0, 0, -1), 90);
        assert_eq!(w2d.shift(0, 0, -11), 90);
        assert_eq!(w2d.shift(10, 0, 1), 20);
        assert_eq!(w2d.shift(10, 0, 11), 20);
        assert_eq!(w2d.shift(10, 0, -1), 0);
        assert_eq!(w2d.shift(10, 0, -11), 0);

        // More tests
        let grids = vec![
            WrappingCoords2d::new(10, 10).unwrap(),
            WrappingCoords2d::new(20, 20).unwrap(),
            WrappingCoords2d::new(100, 100).unwrap(),
            WrappingCoords2d::new(21, 2).unwrap(),
            WrappingCoords2d::new(1, 1).unwrap(),
            WrappingCoords2d::new(10000, 10).unwrap(),
            WrappingCoords2d::new(10000, 10000).unwrap(),
            WrappingCoords2d::new(1, 10000000).unwrap(),
        ];
        for g in grids {
            assert_eq!(g.shift(0, 1, 0), 1 % g.wu);
            assert_eq!(g.shift(0, 1, 20), (20 * g.wu + (1 % g.wu)) % g.szu);
            assert_eq!(g.shift(0, 20, 20), (20 * g.wu + (20 % g.wu)) % g.szu);
            assert_eq!(
                g.shift(0, 200000, 200000),
                (200000 * g.wu + (200000 % g.wu)) % g.szu
            );
            let x1 = 10000;
            let y1 = 10000;
            let idx1 = g.index(x1, y1);
            assert_eq!(g.shift(idx1, 1, 0), g.index(x1 + 1, y1));
            assert_eq!(g.shift(idx1, 1, 20), g.index(x1 + 1, y1 + 20));
            assert_eq!(g.shift(idx1, 20, 20), g.index(x1 + 20, y1 + 20));
            assert_eq!(
                g.shift(idx1, 200000, 200000),
                g.index(x1 + 200000, y1 + 200000)
            );
            assert_eq!(g.shift(idx1, -1, 0), g.index(x1 - 1, y1));
            assert_eq!(g.shift(idx1, -1, 20), g.index(x1 - 1, y1 + 20));
            assert_eq!(g.shift(idx1, -20, 20), g.index(x1 - 20, y1 + 20));
            assert_eq!(
                g.shift(idx1, -200000, 200000),
                g.index(x1 - 200000, y1 + 200000)
            );
            assert_eq!(g.shift(idx1, -1, -1), g.index(x1 - 1, y1 - 1));
            assert_eq!(g.shift(idx1, -1, -20), g.index(x1 - 1, y1 - 20));
            assert_eq!(g.shift(idx1, -20, -20), g.index(x1 - 20, y1 - 20));
            assert_eq!(
                g.shift(idx1, -200000, -200000),
                g.index(x1 - 200000, y1 - 200000)
            );
            assert_eq!(g.shift(idx1, 0, -1), g.index(x1, y1 - 1));
            assert_eq!(g.shift(idx1, 1, -20), g.index(x1 + 1, y1 - 20));
            assert_eq!(g.shift(idx1, 20, -20), g.index(x1 + 20, y1 - 20));
            assert_eq!(
                g.shift(idx1, 200000, -200000),
                g.index(x1 + 200000, y1 - 200000)
            );
            assert_eq!(
                g.neighbors4(idx1),
                vec![
                    g.index(x1 + 1, y1),
                    g.index(x1, y1 + 1),
                    g.index(x1 - 1, y1),
                    g.index(x1, y1 - 1)
                ]
            );
            assert_eq!(
                g.neighbors8(idx1),
                vec![
                    g.index(x1 + 1, y1),
                    g.index(x1 + 1, y1 + 1),
                    g.index(x1, y1 + 1),
                    g.index(x1 - 1, y1 + 1),
                    g.index(x1 - 1, y1),
                    g.index(x1 - 1, y1 - 1),
                    g.index(x1, y1 - 1),
                    g.index(x1 + 1, y1 - 1)
                ]
            );
            assert_eq!(
                g.neighbors16(idx1),
                vec![
                    g.index(x1 + 2, y1),
                    g.index(x1 + 2, y1 + 1),
                    g.index(x1 + 2, y1 + 2),
                    g.index(x1 + 1, y1 + 2),
                    g.index(x1, y1 + 2),
                    g.index(x1 - 1, y1 + 2),
                    g.index(x1 - 2, y1 + 2),
                    g.index(x1 - 2, y1 + 1),
                    g.index(x1 - 2, y1),
                    g.index(x1 - 2, y1 - 1),
                    g.index(x1 - 2, y1 - 2),
                    g.index(x1 - 1, y1 - 2),
                    g.index(x1, y1 - 2),
                    g.index(x1 + 1, y1 - 2),
                    g.index(x1 + 2, y1 - 2),
                    g.index(x1 + 2, y1 - 1)
                ]
            );
            assert_eq!(
                g.neighbors24(idx1),
                vec![
                    g.index(x1 + 1, y1),
                    g.index(x1 + 1, y1 + 1),
                    g.index(x1, y1 + 1),
                    g.index(x1 - 1, y1 + 1),
                    g.index(x1 - 1, y1),
                    g.index(x1 - 1, y1 - 1),
                    g.index(x1, y1 - 1),
                    g.index(x1 + 1, y1 - 1),
                    g.index(x1 + 2, y1),
                    g.index(x1 + 2, y1 + 1),
                    g.index(x1 + 2, y1 + 2),
                    g.index(x1 + 1, y1 + 2),
                    g.index(x1, y1 + 2),
                    g.index(x1 - 1, y1 + 2),
                    g.index(x1 - 2, y1 + 2),
                    g.index(x1 - 2, y1 + 1),
                    g.index(x1 - 2, y1),
                    g.index(x1 - 2, y1 - 1),
                    g.index(x1 - 2, y1 - 2),
                    g.index(x1 - 1, y1 - 2),
                    g.index(x1, y1 - 2),
                    g.index(x1 + 1, y1 - 2),
                    g.index(x1 + 2, y1 - 2),
                    g.index(x1 + 2, y1 - 1)
                ]
            );
        }
    }
}

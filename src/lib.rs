/* WrappingCoords2d: Translate between 2D coordinates and 1D indices with wrapping

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

#![doc(html_root_url = "https://docs.rs/wrapping_coords2d/0.1.3")]

//! Translate between 1D indices and 2D coordinates with wrapping.
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
    /// Either or both indices are less than 1.
    IndicesLessThan1,
    /// The product of both indices is larger than `std::i32::MAX`.
    IndicesTooLarge,
}

/// Represents a 2D grid with wrapping.
#[derive(Debug, PartialEq)]
pub struct WrappingCoords2d {
    /// Width of the grid; it has to be larger than 0.
    w: i32,
    /// Height of the grid; it has to be larger than 0.
    h: i32,
    /// Total number of cells in the grid; it has to be larger than 0 and smaller than std::i32::MAX.
    sz: i32,
}

impl WrappingCoords2d {
    /// Constructs a new WrappingCoords2d representation.
    /// 
    /// # Errors
    /// 
    /// Both `width` and `height` must be larger than 0. Also, their product must be smaller than `std::i32::MAX`.
    pub fn new(width: i32, height: i32) -> Result<WrappingCoords2d, ErrorKind> {
        if width > 0 && height > 0 {
            match width.checked_mul(height) {
                Some(s) => Ok(WrappingCoords2d {
                    w: width,
                    h: height,
                    sz: s,
                }),
                None => Err(ErrorKind::IndicesTooLarge),
            }
        } else {
            Err(ErrorKind::IndicesLessThan1)
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
        self.w
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
        self.h
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
        self.sz as usize
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
        self.sz
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
        let mx = WrappingCoords2d::modulo(x, self.w);
        let myw = WrappingCoords2d::modulo(y * self.w, self.sz);
        (myw + mx) as usize
    }
    /// Returns x and y coordinates based on an index into the 1D container.
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
    pub fn coords(&self, idx: usize) -> (i32, i32) {
        let idx32 = idx as i32; // Always positive
        (idx32 % self.w, idx32 / self.h)
    }
    /// Returns a new index into the grid based on a starting index `start_idx`, an x offset, and a y offset.
    /// `delta_x` and `delta_y` can be negative.
    /// 
    /// # Safety
    /// 
    /// This function does not check that start_idx is a valid index. However, it returns a valid index in the range [0, size).
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
    pub fn shift(&self, start_idx: usize, delta_x: i32, delta_y: i32) -> usize {
        // Note: -11 % 10 = -1
        let idx = start_idx as i32;
        let x = idx % self.w; // Always positive
        let new_x = WrappingCoords2d::modulo(x + delta_x, self.w); // Positive number
        let yw = idx - x; // yw: The y coordinate times the width; always positive
        let new_yw = WrappingCoords2d::modulo(yw + delta_y * self.w, self.sz); // Positive number
        (new_yw + new_x) as usize
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
            assert_eq!(g.shift(0, 1, 0), 1 % g.w as usize);
            assert_eq!(g.shift(0, 1, 20), ((20 * g.w + (1 % g.w)) % g.sz) as usize);
            assert_eq!(
                g.shift(0, 20, 20),
                ((20 * g.w + (20 % g.w)) % g.sz) as usize
            );
            assert_eq!(
                g.shift(0, 200000, 200000),
                ((200000 * g.w + (200000 % g.w)) % g.sz) as usize
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
        }
    }
}

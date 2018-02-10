//! This module contains type defintions for two-dimensional grids
//! that are used in the distance transform methods.

/// A two-dimensional grid of an arbitrary type `T`
#[derive(Debug,Clone)]
pub struct GenericGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default+Clone> GenericGrid<T> {
    /// Constructs a new two-dimensional `GenericGrid<T>`
    /// of size `width`x`height` with default values of type `T`.
    pub fn new(width: usize, height: usize) -> GenericGrid<T> {
        GenericGrid { data: vec![T::default(); height*width], width, height }
    }

    /// Returns the width of the grid
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the height of the grid
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the element at position (x, y), or `None` if the position
    /// is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.get_unsafe(x, y))
    }

    /// Returns a pointer to the element at position (x, y), without doing
    /// bounds checking.
    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        &self.dara[y*self.width+x]
    }

    /// Sets the value of the grid at position (x,y)
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y*self.width+x] = value;
    }

    /// Converts the `GenericGrid<T>` to a `Vec<T>` with indexing
    /// `grid[x, y] == vec[y*grid.width + x]`
    pub fn to_vec(self) -> Vec<T> {
        self.data
    }
}

/// two-dimensional grid of `f64` values
pub type FloatGrid = GenericGrid<f64>;

/// two-dimensional grid of `bool` values
pub type BoolGrid  = GenericGrid<bool>;
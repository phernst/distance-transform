//! This module contains type definitions for two-dimensional grids
//! that are used in the distance transform methods.

use std::slice::{Iter,IterMut};

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
        Some(self.get_unchecked(x, y))
    }

    /// Returns a pointer to the element at position (x, y), without doing
    /// bounds checking.
    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        &self.data[y*self.width+x]
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

    /// Returns an iterator over the nodes of the grid.
    /// An item of the iterator contains the position
    /// and a reference to the node: `(x, y, value)`
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            data_it: self.data.iter(),
            width: self.width,
            pos: 0,
        }
    }

    /// Returns an iterator over the nodes of the grid.
    /// An item of the iterator contains the position
    /// and a mutable reference to the node: `(x, y, value)`
    pub fn iter_mut(&mut self) -> GridIterMut<T> {
        GridIterMut {
            data_it: self.data.iter_mut(),
            width: self.width,
            pos: 0,
        }
    }
}

/// Iterate over the nodes of a grid.
#[derive(Debug)]
pub struct GridIter<'a, T: 'a> {
    data_it: Iter<'a, T>,
    width: usize,
    pos: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.data_it.next() {
            Some(val) => {
                let res = (self.pos % self.width, self.pos/self.width, val);
                self.pos += 1;
                Some(res)
            },
            None => None,
        }
    }
}

/// Iterate mutably over the nodes of a grid.
#[derive(Debug)]
pub struct GridIterMut<'a, T: 'a> {
    data_it: IterMut<'a, T>,
    width: usize,
    pos: usize,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (usize, usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.data_it.next() {
            Some(val) => {
                let res = (self.pos % self.width, self.pos/self.width, val);
                self.pos += 1;
                Some(res)
            },
            None => None,
        }
    }
}

/// two-dimensional grid of `f64` values
pub type FloatGrid = GenericGrid<f64>;

/// two-dimensional grid of `bool` values
pub type BoolGrid  = GenericGrid<bool>;
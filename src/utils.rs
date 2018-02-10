//! This module contains utility methods for working with images.

extern crate image;

use std::io::Write;
use std::f64;

use super::grid::*;

/// Performs a MinMax scaling
///
/// Maps the values of `input` to the interval `[range.0, range.1]`
pub fn min_max_scaling(input: &FloatGrid, range: &(f64, f64)) -> FloatGrid {
    let (min_value, max_value) = {
        let mut curmin = input.get(0, 0).unwrap();
        let mut curmax = input.get(0, 0).unwrap();
        for y in 0..input.height() {
            for x in 0..input.width() {
                if let Some(v) = input.get(x, y) {
                    if v < curmin {
                        curmin = v;
                    }
                    if v > curmax {
                        curmax = v;
                    }
                }
            }
        }
        (curmin, curmax)
    };

    if min_value == max_value {
        return FloatGrid::new(input.width(), input.height());
    }

    let mut result = FloatGrid::new(input.width(), input.height());
    for y in 0..input.height() {
        for x in 0..input.width() {
            let mut newval = (input.get(x, y).unwrap() - min_value)/(max_value - min_value);
            newval = newval*(range.1-range.0) - range.0;
            result.set(x, y, newval);
        }
    }
    result
}

/// Saves a `FloatGrid` to an 8bit image file
///
/// # Safety
/// Assumes that the values of the grid are in the interval [0,255].
pub fn save_float_grid<W: Write>(float_grid: &FloatGrid, w: &mut W, format: image::ImageFormat) -> image::ImageResult<()> {
    let mut imgbuf = image::ImageBuffer::new(float_grid.width() as u32, float_grid.height() as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([*float_grid.get(x as usize, y as usize).unwrap() as u8]);
    }
    image::ImageLuma8(imgbuf).save(w, format)
}

/// Takes the square root of every value in the grid.
///
/// Use this function after calculating the Squared Euclidean distance
/// transform to get the Euclidean distance transform.
///
/// # Safety
/// Assumes that all values of the grid are non-negative.
pub fn sqrt_img(mut im: FloatGrid) -> FloatGrid {
    for y in 0..im.height() {
        for x in 0..im.width() {
            let newval = im.get(x, y).unwrap().sqrt();
            im.set(x, y, newval);
        }
    }
    im
}
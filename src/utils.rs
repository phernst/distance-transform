//! This module contains utility methods for working with images.

extern crate image;

use std::io::Write;
use std::f64;

use super::grid::*;

/// Performs a MinMax scaling
///
/// Maps the values of `input` to the interval `[range.0, range.1]`
pub fn min_max_scaling(input: &FloatGrid, range: &(f64, f64)) -> FloatGrid {
    let (min_value, max_value) = input.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |acc, (_,_,&val)| {
        (acc.0.min(val), acc.1.max(val))
    });

    if min_value == max_value {
        return FloatGrid::new(input.width(), input.height());
    }

    let mut result = FloatGrid::new(input.width(), input.height());
    for (x, y, &val) in input.iter() {
        let mut newval = (val - min_value)/(max_value - min_value);
        newval = newval*(range.1-range.0) - range.0;
        result.set(x, y, newval);
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
    for (_, _, val) in im.iter_mut() {
        let newval = val.sqrt();
        *val = newval;
    }
    im
}
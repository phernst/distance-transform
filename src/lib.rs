//! This crate provides an implementation of a distance transform of binary 
//! grids using Squared Euclidean distances. It is a port of the C++
//! implementation of [Distance Transforms of Sampled Functions][1] by
//! P. Felzenszwalb and D. Huttenlocher.
//! 
//! [1]: http://cs.brown.edu/~pff/dt/
//! 
//! # Examples
//! To use the functions inside module `distance_transform::utils` you need to
//! compile this crate with the feature image-utils.
//!
//! ```
//! extern crate distance_transform;
//! extern crate image;
//! 
//! use distance_transform::*;
//! use distance_transform::utils;
//! use std::fs::File;
//! 
//! fn main() {
//!     // create a 128x128 binary image
//!     let imgwidth = 128usize;
//!     let imgheight = 128usize;
//!     let mut bimg = BoolGrid::new(imgwidth, imgheight);
//! 
//!     // draw a circle
//!     for y in 0..bimg.height() {
//!         for x in 0..bimg.width() {
//!             let pos = (x as isize - 64)*(x as isize - 64)
//!                     + (y as isize - 64)*(y as isize - 64);
//!             bimg.set(x, y, pos < 32*32 && pos > 31*31);
//!         }
//!     }
//! 
//!     // do the distance transformation and
//!     // take the square root since squared distances are calculated
//!     let fmm = utils::sqrt_img(dt2d(&bimg));
//! 
//!     // scale values to range [0, 255] and save as image
//!     let fmm_img = utils::min_max_scaling(&fmm, &(0., 255.));
//!     let ref mut fmm_out = File::create("fmm_out.png").unwrap();
//!     utils::save_float_grid(&fmm_img, fmm_out, image::PNG).unwrap();
//! }
//! ```

pub mod grid;
pub use grid::*;

#[cfg(feature = "image-utils")]
pub mod utils;
#[cfg(feature = "image-utils")]
pub use utils::*;

pub mod dt;
pub use dt::*;
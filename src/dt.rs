//! This module contains the functions for calculating the distance transform.

pub use super::grid::*;

const INF: f64 = 100000002004087734272.;

/* dt of 1d function using squared distance */
fn dt1d_float(f: &Vec<f64>) -> Vec<f64> {
    let mut d = vec![0f64; f.len()];
    let mut v = vec![0i32; f.len()];
    let mut z = vec![0f64; f.len() + 1];
    let mut k = 0i32;
    v[0] = 0;
    z[0] = -INF;
    z[1] = INF;

    for q in 1..f.len() {
        let mut s = ((f[q]+(q*q) as f64)-(f[v[k as usize] as usize]+(v[k as usize]*v[k as usize]) as f64))/(2*q as i32-2*v[k as usize]) as f64;
        while s <= z[k as usize] {
            k -= 1;
            s = ((f[q]+(q*q) as f64)-(f[v[k as usize] as usize]+(v[k as usize]*v[k as usize]) as f64))/(2*q as i32-2*v[k as usize]) as f64;
        }
        k += 1;
        v[k as usize] = q as i32;
        z[k as usize] = s;
        z[k as usize + 1] = INF;
    }

    k = 0;
    for q in 0..f.len() {
        while z[k as usize + 1] <= q as f64 {
            k += 1;
        }
        d[q] = ((q as i32-v[k as usize])*(q as i32-v[k as usize])) as f64 + f[v[k as usize] as usize];
    }

    d
}

/* dt of 2d function using squared distance */
fn dt2d_float(im: &FloatGrid) -> FloatGrid {
    let mut im = im.clone();
    let width = im.width();
    let height = im.height();
    let mut f = vec![0f64; if width > height { width } else { height }];

    // transform along columns
    for x in 0..width {
        for y in 0..height {
            f[y] = *im.get(x, y).unwrap();
        }
        let d = dt1d_float(&f);
        for y in 0..height {
            im.set(x, y, d[y]);
        }
    }

    // transform along rows
    for y in 0..height {
        for x in 0..width {
            f[x] = *im.get(x, y).unwrap();
        }
        let d = dt1d_float(&f);
        for x in 0..width {
            im.set(x, y, d[x]);
        }
    }

    im
}

/// distance transform of boolean vector using Squared Euclidean distance
pub fn dt1d(im: &Vec<bool>) -> Vec<f64> {
    let mut out = Vec::with_capacity(im.len());
    for x in 0..im.len() {
        out[x] = if im[x] { 0. } else { INF };
    }

    dt1d_float(&out)
}

/// distance transform of binary image using Squared Euclidean distance
pub fn dt2d(im: &BoolGrid) -> FloatGrid {
    let width = im.width();
    let height = im.height();

    let mut out = FloatGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            out.set(x, y, if *im.get(x, y).unwrap() { 0. } else { INF });
        }
    }

    dt2d_float(&out)
}

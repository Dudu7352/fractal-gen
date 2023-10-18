use num::Complex;

use crate::{options::{FractalOptions, Range, FractalType}, functions::value_calc::get_finite_u8};

pub fn calculate_part(
    id: usize,
    options: FractalOptions,
    range: Range
) -> Vec<u8> {
    let mut v = Vec::with_capacity((range.end - range.start) * options.resolution);
    for y in range.start..range.end {
        let cy = options.get_cy(y as f64);

        for x in 0..options.resolution {
            let cx = options.get_cx(x as f64);

            let (z, cst) = match options.fractal {
                FractalType::Mandelbrot => (Complex::new(0f64, 0f64), Complex::new(cx, cy)),
                FractalType::Julia(constant) => (Complex::new(cx, cy), constant)
            };

            v.push(get_finite_u8(z, cst));
        }
    }
    println!("Task on Thread {id} Complete");
    v
}
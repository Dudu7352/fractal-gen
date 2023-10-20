use num::Complex;

use crate::options::{FractalOptions, Range};

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
            
            v.push(options.fractal.get_val(Complex::new(cx, cy)));
        }
    }
    println!("Task on Thread {id} Complete");
    v
}

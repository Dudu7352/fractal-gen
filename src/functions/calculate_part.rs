use num::Complex;

use crate::options::{FractalOptions, Range};

pub fn calculate_part(
    id: usize,
    options: FractalOptions,
    range: Range
) -> Vec<u8> {
    let mut v = Vec::with_capacity((range.end - range.start) * options.resolution * 3);
    for y in range.start..range.end {
        let cy = options.get_cy(y as f64);

        for x in 0..options.resolution {
            let cx = options.get_cx(x as f64);
            
            v.push(255u8 - scale_to_u8(options.resolution, x));
            v.push(options.fractal.get_val(Complex::new(cx, cy)));
            v.push(255u8 - scale_to_u8(options.resolution, y));
        }
    }
    println!("Task on Thread {id} Complete");
    v
}

fn scale_to_u8( max: usize, num: usize) -> u8 {
    return ((num as f64)/(max as f64) * 255f64) as u8;
}

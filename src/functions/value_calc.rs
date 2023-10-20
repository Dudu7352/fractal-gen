use num::Complex;

pub fn get_finite_u8(mut z: Complex<f64>, c: Complex<f64>, pow: f64) -> u8 {
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
        z = z.powf(pow) + c;
        i += 1;
    }
    i
}

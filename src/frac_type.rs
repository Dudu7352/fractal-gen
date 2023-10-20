use num::Complex;
use crate::functions::value_calc::get_finite_u8;

#[derive(Clone)]
pub enum FracType {
    Mandelbrot,
    Julia { c: Complex<f64> },
    Mbrot { pow: f64 }
}

impl FracType {
    pub fn get_val(&self, position: Complex<f64>) -> u8 {
       match self {
           Self::Mandelbrot => get_finite_u8(Complex::new(0.0, 0.0), position, 2.0),
           Self::Julia{c} => get_finite_u8(position, *c, 2.0),
           Self::Mbrot { pow } => get_finite_u8(Complex::new(0.0, 0.0), position, *pow)
       } 
    }
}

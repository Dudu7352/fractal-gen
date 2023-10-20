use num::Complex;
use crate::frac_type::FracType;

#[derive(Clone)]
pub struct FractalOptions {
    pub scale: f64,
    pub resolution: usize,
    pub center: usize,
    pub offset: Complex<f64>,
    pub fractal: FracType,
}

impl FractalOptions {
    pub fn new(
        scale: f64,
        resolution: usize,
        center: usize,
        offset: Complex<f64>,
        fractal: FracType,
    ) -> Self {
        Self {
            scale,
            resolution,
            center,
            offset,
            fractal,
        }
    }

    pub fn get_cx(&self, x: f64) -> f64 {
        (x - self.center as f64) / self.scale + self.offset.re
    }

    pub fn get_cy(&self, y: f64) -> f64 {
        (y - self.center as f64) / self.scale + self.offset.im
    }
}

pub struct Range {
    pub start: usize,
    pub end: usize,
}

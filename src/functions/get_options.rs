use std::io::{stdin, stdout};
use num::Complex;
use crate::FractalOptions;
use crate::frac_type::FracType;
use super::input::input;

pub fn get_options() -> Result<(FractalOptions, usize), String> {
   let s_in = stdin();
   let s_out = stdout();

    let zoom: f64 = input(&s_out, &s_in, "Zoom: ");
    if zoom == 0f64 {
        return Err(String::from("Zoom cannot be equal to zero!"));
    }
    let frac: FracType = match input::<String>(&s_out, &s_in, "Fractal [mandelbrot/julia/mbrot]: ").as_str() {
        "mandelbrot" => FracType::Mandelbrot,
        "julia" => {
            FracType::Julia{
                c: input(&s_out, &s_in, "Input julia constant: ")
            }
        },
        "mbrot" => FracType::Mbrot {
            pow: input(&s_out, &s_in, "Input power: ")
        },
        _ => return Err(String::from("This type of fractal is not supported!"))
    };

    let offset_x: f64 = input(&s_out, &s_in, "X offset: ");
    let offset_y: f64 = input(&s_out, &s_in, "Y offset: ");
    let size: usize = input(&s_out, &s_in, "Image width and height: ");
    let threads: usize = input(&s_out, &s_in, "Number of worker threads: ");
    
    Ok((FractalOptions::new(
        zoom as f64 * size as f64,
        size,
        size/2,
        Complex::new(offset_x, offset_y),
        frac
    ), threads))
} 

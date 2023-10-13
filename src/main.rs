pub mod options;
pub mod functions;

use std::time::Instant;
use std::{
    io::{stdin, stdout},
    thread,
};

use image::ImageBuffer;
use num::Complex;
use options::{FractalOptions, Range};

use crate::functions::calculate_part::calculate_part;
use crate::functions::input::input;
use crate::options::FractalType;

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let zoom: f64 = input(&stdout, &stdin, "Zoom: ");
    if zoom == 0f64 {
        println!("Zoom cannot be equal to zero!");
        return;
    }
    let frac: FractalType = match input::<String>(&stdout, &stdin, "Fractal [mandelbrot/julia]").as_str() {
        "mandelbrot" => FractalType::Mandelbrot,
        "julia" => {
            FractalType::Julia(
                input(&stdout, &stdin, "Input julia constant: ")
            )
        },
        _ => panic!("This type of fractal is not supported!")
    };

    let offset_x: f64 = input(&stdout, &stdin, "X offset: ");
    let offset_y: f64 = input(&stdout, &stdin, "Y offset: ");
    let size: usize = input(&stdout, &stdin, "Image width and height: ");
    let threads: usize = input(&stdout, &stdin, "Number of worker threads: ");
    
    let options = FractalOptions::new(
        zoom as f64 * size as f64,
        size,
        size/2,
        Complex::new(offset_x, offset_y),
        frac
    );

    let timer = Instant::now();
    let mut pixels = Vec::with_capacity(size * size);
    let mut th = Vec::with_capacity(threads);

    for i in 0..threads {
        let end = if i == threads - 1 {
            size
        } else {
            (size / threads) * (i + 1)
        };
        let o = options.clone();
        th.push(thread::spawn(move || 
            calculate_part(
                i, 
                o, 
                Range {
                    start: (size / threads) * i,
                    end
                })
        ))
    }

    for t in th {
        pixels.append(&mut t.join().unwrap());
    }

    let img: ImageBuffer<image::Luma<u8>, Vec<u8>> =
        ImageBuffer::from_raw(size as u32, size as u32, pixels).unwrap();

    let _ = img.save("frac.png");
    println!(
        "Task completed in {} seconds",
        timer.elapsed().as_millis() as f64 / 1000f64
    )
}

pub mod options;

use std::fmt::Debug;
use std::time::Instant;
use std::{
    io::{stdin, stdout, Stdin, Stdout, Write},
    str::FromStr,
    thread,
};

use image::ImageBuffer;
use num::Complex;
use options::{FractalOptions, Range};

use crate::options::FractalType;

fn input<'a, T: FromStr>(mut stdout: &Stdout, stdin: &Stdin, prompt: &'a str) -> T
where 
    <T as FromStr>::Err: Debug,
{
    let mut buf = String::new();
    print!("{}", prompt);
    let _ = stdout.flush().unwrap();
    let _ = stdin.read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
}

fn get_finite_u8(mut z: Complex<f64>, c: Complex<f64>) -> u8 {
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    i
}

fn calculate_part(
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

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let zoom: f64 = input(&stdout, &stdin, "Zoom: ");
    if zoom == 0f64 {
        println!("Zoom cannot be equal to zero!");
        return;
    }
    let size: usize = input(&stdout, &stdin, "Image width and height: ");
    let offset_x: f64 = input(&stdout, &stdin, "X offset: ");
    let offset_y: f64 = input(&stdout, &stdin, "Y offset: ");
    let scale: f64 = zoom as f64 * size as f64;
    let threads: usize = input(&stdout, &stdin, "Number of worker threads: ");
    let frac: FractalType = match input::<String>(&stdout, &stdin, "Fractal [mandelbrot/julia]").as_str() {
        "mandelbrot" => FractalType::Mandelbrot,
        "julia" => {
            FractalType::Julia(
                input(&stdout, &stdin, "Input julia constant: ")
            )
        },
        _ => panic!("This type of fractal is not supported!")
    };

    let options = FractalOptions::new(
        scale,
        size,
        size/2,

    )

    let timer = Instant::now();
    let mut pixels = Vec::with_capacity(size * size);
    let mut th = Vec::with_capacity(threads);

    for i in 0..threads {
        let end = if i == threads - 1 {
            size
        } else {
            (size / threads) * (i + 1)
        };
        let c = frac.clone();
        th.push(thread::spawn(move || 
            calculate_part(i, size, size / 2, (size / threads) * i, end, scale, offset_x, offset_y, c)
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

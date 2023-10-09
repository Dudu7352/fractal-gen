use std::fmt::Debug;
use std::time::Instant;
use std::{
    io::{stdin, stdout, Stdin, Stdout, Write},
    str::FromStr,
    thread,
};

use image::ImageBuffer;
use num::Complex;

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

fn mbrot(mut z: Complex<f64>, c: Complex<f64>) -> u8 {
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    i
}

fn calculate_part(
    id: usize,
    size_x: usize,
    center: usize,
    start_y: usize,
    end_y: usize,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
) -> Vec<u8> {
    let mut v = Vec::with_capacity((end_y - start_y) * size_x);
    for y in start_y..end_y {
        let cy = (y as f64 - center as f64) / scale - offset_y;

        for x in 0..size_x {
            let cx = (x as f64 - center as f64 * 1.5) / scale - offset_x;

            let z = Complex::new(0f64, 0f64);
            let cst = Complex::new(cx, cy);

            v.push(mbrot(z, cst));
        }
    }
    println!("Task on Thread {id} Complete");
    v
}

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let zoom: f64 = input(&stdout, &stdin, "Zoom: ");
    let size: usize = input(&stdout, &stdin, "Image width and height: ");
    let offset_x: f64 = input(&stdout, &stdin, "X offset: ");
    let offset_y: f64 = input(&stdout, &stdin, "Y offset: ");
    let scale: f64 = zoom as f64 * size as f64;
    let threads: usize = input(&stdout, &stdin, "Number of worker threads: ");

    let timer = Instant::now();
    let mut pixels = Vec::with_capacity(size * size);
    let mut th = Vec::with_capacity(threads);

    for i in 0..threads {
        let end = if i == threads - 1 {
            size
        } else {
            (size / threads) * (i + 1)
        };
        th.push(thread::spawn(move || 
            calculate_part(i, size, size / 2, (size / threads) * i, end, scale, offset_x, offset_y)
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

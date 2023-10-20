pub mod frac_type;
pub mod options;
pub mod functions;

use std::time::Instant;
use std:: thread;

use image::ImageBuffer;
use functions::get_options::get_options;
use options::{FractalOptions, Range};

use crate::functions::calculate_part::calculate_part;

fn main() {
    let (options, threads) = match get_options() {
        Ok((options, threads)) => (options, threads),
        Err(err) => {
            eprintln!("Program encountered error:\n{}", err);
            return ();
        }
    };

    let timer = Instant::now();
    let mut pixels = Vec::with_capacity(options.resolution * options.resolution);
    let mut th = Vec::with_capacity(threads);

    for i in 0..threads {
        let end = if i == threads - 1 {
            options.resolution
        } else {
            (options.resolution / threads) * (i + 1)
        };
        let o = options.clone();
        th.push(thread::spawn(move || 
            calculate_part(
                i, 
                o, 
                Range {
                    start: (options.resolution / threads) * i,
                    end
                })
        ))
    }

    for t in th {
        pixels.append(&mut t.join().unwrap());
    }

    let img: ImageBuffer<image::Luma<u8>, Vec<u8>> =
        ImageBuffer::from_raw(options.resolution as u32, options.resolution as u32, pixels).unwrap();

    let _ = img.save("frac.png");
    println!(
        "Task completed in {} seconds",
        timer.elapsed().as_millis() as f64 / 1000f64
    )
}

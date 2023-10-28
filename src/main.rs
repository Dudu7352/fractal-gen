pub mod get_options;
pub mod input;

use std::time::Instant;

use frac_lib::{frac_generator::FracGenerator, options::gen_method::GenMethod};


#[tokio::main]
async fn main() {
    let (options, threads) = match get_options::get_options() {
        Ok((options, threads)) => (options, threads),
        Err(err) => {
            eprintln!("Program encountered error:\n{}", err);
            return ();
        }
    };

    let frac_generator  = FracGenerator::new(options, GenMethod::MultithreadAsync { threads });

    let timer = Instant::now();
    let img = frac_generator.generate_image().await.unwrap();

    let _ = img.save("frac.png");
    println!(
        "Task completed in {} seconds",
        timer.elapsed().as_millis() as f64 / 1000f64
    )
}

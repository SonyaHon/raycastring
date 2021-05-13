use std::fs::File;
use std::io::Write;
use crate::vector_3::Vector3;

use rand::Rng;

pub trait GenImageCallback {
    fn get_color(&self, u: f64, v: f64) -> Vector3;
}

pub fn gen_image<T: GenImageCallback>(width: i32, height: i32, filename: &str, executor: T) {
    println!("Rendering image {}x{} to {}", width, height, filename);

    let mut file = File::create(filename).expect(format!("Could not create file {}", filename).as_str());
   let mut final_string = format!("P3\n{} {}\n255\n", width, height);

    let mut rng = rand::thread_rng();


    for j in (0..height).rev() {
        for i in 0..width {
            let mut color = Vector3::zero();
            let samples_per_pixel = 20;

            for s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;

                let parsed_color = executor.get_color(u, v);
                color = color + parsed_color;
            }

            final_string += color.to_color_sampled_with_newline(samples_per_pixel).as_str();
        }

        let percents = (((height - j) as f64 / height as f64) * 100.0).floor();
        println!("Progress: {}%", percents);
    }

    file.write_all(final_string.as_bytes()).expect("Error while writing to the file");
}

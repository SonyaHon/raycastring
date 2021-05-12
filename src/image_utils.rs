use std::fs::File;
use std::io::Write;
use crate::vector_3::Vector3;

pub trait GenImageCallback {
    fn get_color(&self, u: f64, v: f64) -> Vector3;
}

pub fn gen_image<T: GenImageCallback>(width: i32, height: i32, filename: &str, executor: T) {
    println!("Rendering image {}x{} to {}", width, height, filename);

    let mut file = File::create(filename).expect(format!("Could not create file {}", filename).as_str());
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes()).expect("Error writing to file");

    for j in (0..height).rev() {
        for i in 0..width {

            let u = (i as f64) / (width - 1) as f64;
            let v = (j as f64) / (height - 1) as f64;

            let color = executor.get_color(u, v);

            file.write_all(color.to_color_str_with_newline().as_bytes()).expect("Error writing to file");
        }

        let percents = (((height - j) as f64 / height as f64) * 100.0).floor();
        println!("Progress: {}%", percents);
    }
}

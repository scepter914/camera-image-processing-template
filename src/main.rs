extern crate image;
extern crate imageproc;
extern crate rscam;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

pub struct Benchmark {
    start_time: std::time::Instant,
}

impl Benchmark {
    pub fn set_start_time() -> Benchmark {
        let now_time = std::time::Instant::now();
        Benchmark {
            start_time: now_time,
        }
    }

    pub fn print_bench_time(&self) {
        let end = self.start_time.elapsed();
        println!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }
}

fn rgb_to_gray(rgb_image: &image::RgbImage) -> image::GrayImage {
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut gray_image = image::GrayImage::new(width, height);
    // gray scale
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            let gray_pixel = [((pixel[0] as f32 * 0.2126) as u32
                + (pixel[1] as f32 * 0.7152) as u32
                + (pixel[2] as f32 * 0.0722) as u32) as u8; 1];
            gray_image.put_pixel(i, j, image::Luma(gray_pixel));
        }
    }
    gray_image
}

fn main() {
    let device = "/dev/video0";
    let mut camera = rscam::new(device).unwrap();
    // let width = 1920;
    // let height = 1080;
    // let fps = 50;
    let width = 640;
    let height = 360;
    let fps = 330;
    camera
        .start(&rscam::Config {
            interval: (1, fps),
            resolution: (width, height),
            format: b"RGB3",
            ..Default::default()
        })
        .unwrap();

    // loop (and dispose beginning frame for benchmark)
    let mut counter = 0;
    loop {
        let frame = camera.capture().unwrap();
        let rgb_image = image::RgbImage::from_vec(width, height, (&frame[..]).to_vec()).unwrap();
        let gray_image = rgb_to_gray(&rgb_image);

        let otsu_level = imageproc::contrast::otsu_level(&gray_image);
        let binarized_image = imageproc::contrast::threshold(&gray_image, otsu_level);

        // save images and break loop
        if counter > 300 {
            gray_image.save("data/gray_image.png").unwrap();
            binarized_image.save("data/binarized_image.png").unwrap();
            break;
        }
        counter += 1;
    }
    benchmark(device, camera, width, height, fps);
}

fn save_file_by_ppm(image: image::RgbImage, path: &str) -> std::result::Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(format!("P6\n{} {}\n255\n", image.width(), image.height()).as_bytes())?;
    file.write_all(&image.to_vec())?;
    Ok(())
}

fn benchmark(device: &str, camera: rscam::Camera, width: u32, height: u32, fps: u32) {
    // Benchmark
    println!("Camera {}: {} * {}, {} FPS ", device, width, height, fps);

    println!("capture");
    let bench = Benchmark::set_start_time();
    let frame = camera.capture().unwrap();
    bench.print_bench_time();

    println!("from_raw");
    let bench = Benchmark::set_start_time();
    let rgb_image_raw = image::RgbImage::from_raw(width, height, (&frame[..]).to_vec()).unwrap();
    bench.print_bench_time();

    println!("save ppm by directly");
    let bench = Benchmark::set_start_time();
    save_file_by_ppm(rgb_image_raw, "data/from_raw.ppm");
    bench.print_bench_time();

    println!("from_vec");
    let bench = Benchmark::set_start_time();
    let rgb_image_vec = image::RgbImage::from_vec(width, height, (&frame[..]).to_vec()).unwrap();
    bench.print_bench_time();

    println!("save png by image");
    let bench = Benchmark::set_start_time();
    rgb_image_vec.save("data/from_vec.png").unwrap();
    bench.print_bench_time();

    println!("rgb to gray");
    let bench = Benchmark::set_start_time();
    let gray_image = rgb_to_gray(&rgb_image_vec);
    bench.print_bench_time();

    println!("otsu binarize");
    let bench = Benchmark::set_start_time();
    let otsu_level = imageproc::contrast::otsu_level(&gray_image);
    let binarized_image = imageproc::contrast::threshold(&gray_image, otsu_level);
    bench.print_bench_time();
}

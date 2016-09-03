extern crate image;
extern crate rand;
extern crate img_hash;
mod objects;
use img_hash::{ImageHash, HashImage, HashType};
use objects::{Hitable, random_color, Point, Circle, Triangle};
use image::Rgba;
use std::cmp;
use std::fs::File;
use std::path::Path;
use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

fn random_objects(x: u32, y: u32, count: u32) -> Vec<Box<Hitable>> {
    let mut vec: Vec<Box<Hitable>> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vec.push(Box::new(Circle::random(x, y)));
    }
    vec
}

fn hash_fitness(source: &ImageHash, generated: &image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> f32 {

    let imgx = generated.width();
    let generated_hash = ImageHash::hash(generated, imgx, HashType::Gradient);
    source.dist_ratio(&generated_hash)
}

// https://rogeralsing.com/2008/12/09/genetic-programming-mona-lisa-faq/
fn fitness(source: &image::ImageBuffer<Rgba<u8>, Vec<u8>>,
           generated: &image::ImageBuffer<Rgba<u8>, Vec<u8>>)
           -> f32 {
    let mut fitness = 0.0;
    for (x, y, spixel) in source.enumerate_pixels() {
        let gpixel = generated.get_pixel(x, y);
        let mut local = 0.0;
        for i in 0..3 {
            let s = spixel.data[i] as isize;
            let g = gpixel.data[i] as isize;
            local += (s - g).pow(2) as f32;
        }
        fitness += local;
    }
    fitness
}

fn combine_channel(top: u8, bottom: u8, transparency: u8) -> u8 {
    let transp = transparency as f32 / 255.0;
    let t = top as f32;
    let b = bottom as f32;

    ((transp * t + (1.0 - transp) * b) * 255.0) as u8
}

fn sum_pixel_values(top: &Rgba<u8>, bottom: &Rgba<u8>) -> Rgba<u8> {
    Rgba {
        data: [combine_channel(top[0], bottom[0], top[3]),
               combine_channel(top[1], bottom[1], top[3]),
               combine_channel(top[2], bottom[2], top[3]),
               255],
    }
}

fn main() {
    let old_style = false;
    let file = format!("base.png");
    let reference = image::open(&Path::new(&file)).unwrap().to_rgba();
    let imgx = reference.width();
    let imgy = reference.height();
    let reference_hash = ImageHash::hash(&reference, imgx, HashType::Gradient);
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut list = vec![(std::f32::MAX, imgbuf.clone())];
    let runs = 1_000_001;
    for i in 0..runs {
        let mut rng = rand::thread_rng();
        let object_count = Range::new(2, 5);
        let circles = random_objects(imgx, imgy, object_count.ind_sample(&mut rng));
        let mut current_buf = list[0].1.clone();
        for (x, y, pixel) in current_buf.enumerate_pixels_mut() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if let Some(hit) = circles.iter().find(|circle| circle.hit(&point)) {
                if old_style {
                    *pixel = hit.color().clone();
                } else {
                    *pixel = sum_pixel_values(&hit.color(), &pixel);
                }
            }

        }

        // let value = fitness(&reference, &current_buf);
        let value = hash_fitness(&reference_hash, &current_buf);

        if value < list[0].0 {
            println!("{:?}", value);
            list = vec![(value, current_buf.clone())];
        }

        if i % 10_000 == 0 {
            println!("Iteration #{:?}", i);
        }

        if i % 100_000 == 0 || i == runs - 1 {
            let name = format!("results/run_2_{}.png", i);
            let ref mut fout = File::create(&Path::new(&name)).unwrap();
            let _ = image::ImageRgba8(current_buf).save(fout, image::PNG);
        }
    }
}

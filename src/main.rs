extern crate image;
extern crate rand;
extern crate docopt;
mod objects;
use docopt::Docopt;
use std::env;
use objects::{Hitable, random_color, Point, Circle, Triangle};
use image::Rgba;
use std::cmp;
use std::fs::File;
use std::path::Path;
use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};
const USAGE: &'static str = "
Image gen

Usage:
  image_gen --help
  image_gen --base=<file> [--out=<file>  --n=<number> --blend]

Options:
  -h --help
  --base=<file>
  --out=<file>
  --n=<number>
  --blend
";

fn random_objects(x: u32, y: u32, count: u32) -> Vec<Box<Hitable>> {
    let mut vec: Vec<Box<Hitable>> = Vec::with_capacity(count as usize);
    for i in 0..count {
        if i % 4 == 0 {
            vec.push(Box::new(Triangle::random(x, y)));
        } else {
            vec.push(Box::new(Circle::random(x, y)));
        }
    }
    vec
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
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| {
            println!("{:?}", e);
            e.exit()
        });
    let old_style = !args.get_bool("--blend");
    let file = format!("{}", args.get_str("--base"));
    let final_file = if args.get_str("--out") == "" {
        None
    } else {
        Some(args.get_str("--out"))
    };
    let reference = image::open(&Path::new(&file)).unwrap().to_rgba();
    let imgx = reference.width();
    let imgy = reference.height();
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut list = vec![(std::f32::MAX, imgbuf.clone())];
    let runs = if args.get_str("--n") != "" {
        args.get_str("--n").parse::<i32>().unwrap()
    } else {
        100_000
    };
    for i in 0..runs {
        let mut rng = rand::thread_rng();
        let object_count = Range::new(2, 6);
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

        let value = fitness(&reference, &current_buf);

        if value < list[0].0 {
            // println!("{:?}", value);
            list = vec![(value, current_buf.clone())];
        }

        if i % 10_000 == 0 {
            println!("Iteration #{:?}", i);
        }

        if final_file.is_none() && i % 100_000 == 0 || i == runs - 1 {
            let name = format!("results/run_{}.png", i);
            let ref mut fout = File::create(&Path::new(&name)).unwrap();
            let _ = image::ImageRgba8(current_buf).save(fout, image::PNG);
        }
    }
    if let Some(file) = final_file {
        let name = format!("{}.png", file);
        let ref mut fout = File::create(&Path::new(&name)).unwrap();
        let _ = image::ImageRgba8(list[0].1.clone()).save(fout, image::PNG);
    }
}

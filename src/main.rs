extern crate image;
extern crate rand;
mod objects;
use objects::{Hit, random_color, Color, Point, Circle, Triangle};
use image::Rgba;
use std::cmp;
use std::fs::File;
use std::path::Path;
use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

fn random_objects(x: u32, y: u32, count: u32) -> Vec<Circle>{
let mut vec = Vec::with_capacity(count as usize);
    for _ in 0..count{
        vec.push(Circle::random(x, y));
    }
    vec
}

//https://rogeralsing.com/2008/12/09/genetic-programming-mona-lisa-faq/
fn fitness(source: &image::ImageBuffer<Rgba<u8>, Vec<u8> >,generated: &image::ImageBuffer<Rgba<u8>, Vec<u8>> ) -> f32{
    let mut fitness = 0.0;
    for (x, y, spixel) in source.enumerate_pixels(){
        let gpixel = generated.get_pixel(x,y);
        let mut local = 0.0;
        for i in 0..3{
            let s = spixel.data[i] as isize;
            let g = gpixel.data[i] as isize;
            local += (s - g).pow(2) as f32;
        }
        fitness += local;
    }
    fitness
}

fn main() {
    let file =format!("base.png");
    let reference = image::open(&Path::new(&file)).unwrap().to_rgba();
    let imgx = reference.width();
    let imgy = reference.height();
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut list = vec![(std::f32::MAX ,imgbuf.clone())];
    let runs = 10;
    for i in 0..runs {
        let mut rng = rand::thread_rng();
        let object_count = Range::new(2,5);
        let circles = random_objects(imgx, imgy, object_count.ind_sample(&mut rng));
        let mut current_buf = list[0].1.clone();
        for (x, y, pixel) in current_buf.enumerate_pixels_mut() {
            let point = Point{x: x as i32, y: y as i32};
            if let Some(hit) = circles.iter().find({|circle| circle.hit(&point)}){
                *pixel = hit.color().clone();
            }

        }
        let value = fitness(&reference, &current_buf);
        if value < list[0].0 {
            println!("{:?}", value);
            list = vec![(value, current_buf.clone())];
        }

        if i % 10_000 == 0 || i == runs-1 {
            let name = format!("results/run_{}.png",i);
            let ref mut fout = File::create(&Path::new(&name)).unwrap();
            let _ = image::ImageRgba8(current_buf).save(fout, image::PNG);
        }
    }
}

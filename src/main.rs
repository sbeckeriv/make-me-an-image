extern crate image;
extern crate rand;
mod objects;
use objects::{Hit, Point, Circle};
use image::Rgba;
use rand::distributions::{IndependentSample, Range};
use std::cmp;
use std::fs::File;
use std::path::Path;
use rand::{SeedableRng, StdRng};

fn random_objects(x: u32, y: u32, count: u32) -> Vec<Circle>{
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut vec = Vec::with_capacity(count as usize);
    let radius_max =  cmp::max(x/4, y/4) as f32;
    let radius_between = Range::new(2.0, radius_max);
    let x_between = Range::new(0, x as i32);
    let y_between = Range::new(0, y as i32);
    let color_between = Range::new(0, 255);
    for _ in 0..count{
        vec.push(Circle{
            center: Point{
                x: x_between.ind_sample(&mut rng),
                y: y_between.ind_sample(&mut rng)
            },
            radius: radius_between.ind_sample(&mut rng),
            color: image::Rgba([
                               color_between.ind_sample(&mut rng),
                               color_between.ind_sample(&mut rng),
                               color_between.ind_sample(&mut rng),
                               color_between.ind_sample(&mut rng)
            ])
        });
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
    let circles = random_objects(imgx, imgy, 14);
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut list = vec![imgbuf.clone()];
    let runs = 4;
    for i in (0..runs){
        let mut current_buf = imgbuf.clone();
        for (x, y, pixel) in current_buf.enumerate_pixels_mut() {
            let point = Point{x: x as i32, y: y as i32};
            if let Some(hit) = circles.iter().find({|circle| circle.hit(&point)}){
                *pixel = hit.color().clone();
            }

        }
        let value = fitness(&reference, &current_buf);
        println!("{:?}", value);
        let name = format!("run_{}.png",i);
        let ref mut fout = File::create(&Path::new(&name)).unwrap();
        let _ = image::ImageRgba8(current_buf).save(fout, image::PNG);
    }
}

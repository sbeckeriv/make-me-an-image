extern crate image;
extern crate rand;
use image::Rgba;
use rand::distributions::{IndependentSample, Range};
use std::cmp;
use std::fs::File;
use std::path::Path;
use rand::{Rng, SeedableRng, StdRng};

struct Point{
    x: i32,
    y: i32
}

struct Circle{
    center: Point,
    radius: f32,
    color: Rgba<u8>

}

impl Circle{
    fn hit(&self, pixel: &Point) -> bool{
        let Point{x:cx,y:cy} = self.center;
        let Point{x,y} = *pixel;
        let mid = ((cx-x).pow(2)+(cy-y).pow(2)) as f32;
        let r = (mid).sqrt();
        r <= self.radius
    }
}

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
fn main() {

    let imgx = 800;
    let imgy = 800;
    let circles = random_objects(imgx, imgy, 14);
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let point = Point{x: x as i32, y: y as i32};
        if let Some(hit) = circles.iter().find({|circle| circle.hit(&point)}){
            *pixel = hit.color.clone();
        }

    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG);
}

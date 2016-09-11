extern crate image;
extern crate rand;
extern crate docopt;

mod objects;

use std::sync::Arc;
use docopt::Docopt;
use objects::{Hitable, Point, Circle, Triangle, Rectangle};
use image::Rgba;
use std::fs::File;
use std::path::Path;
use rand::distributions::{IndependentSample, Range};
const USAGE: &'static str = "
Image gen

Usage:
  image_gen --help
  image_gen --base=<file> [--out=<file>  --n=<number> --blend --peek]

Options:
  -h --help
  --base=<file>
  --out=<file>
  --n=<number>
  --blend
  --peek
";

fn random_objects(x: u32, y: u32, count: u32) -> Vec<Arc<Hitable>> {
    let mut vec: Vec<Arc<Hitable>> = Vec::with_capacity(count as usize);
    for i in 0..count {
        if i % 10 == 0 {
            vec.push(Arc::new(Triangle::random(x, y)));
        } else if i % 5 == 0 {
            vec.push(Arc::new(Circle::random(x, y)));
        } else {
            vec.push(Arc::new(Rectangle::random(x, y)));
        }
    }
    vec
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

fn make_me_an_image(args: &docopt::ArgvMap,
                    reference: &image::ImageBuffer<Rgba<u8>, Vec<u8>>)
                    -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    let final_file = if args.get_str("--out") == "" {
        None
    } else {
        Some(args.get_str("--out"))
    };
    let old_style = !args.get_bool("--blend");
    let peek = args.get_bool("--peek");
    let imgx = reference.width();
    let imgy = reference.height();
    let mut list = vec![image::ImageBuffer::new(imgx, imgy)];

    let runs = if args.get_str("--n") != "" {
        args.get_str("--n").parse::<i32>().unwrap()
    } else {
        100_000
    };

    let peek_size = runs / 30;
    let mut rng = rand::thread_rng();
    let object_count = Range::new(10, 20);
    for i in 0..runs {
        let mut current_buf = list[0].clone();
        let objects = random_objects(imgx, imgy, object_count.ind_sample(&mut rng));

        let good_objects: Vec<Arc<Hitable>> = objects.into_iter()
            .filter(|o| o.fitness(&reference, &current_buf) > 0)
            .collect();
        for circle in good_objects {
            let points: (Point, Point) = circle.pixel_box();
            let Point { x: min_x, y: min_y } = points.0;
            let Point { x: max_x, y: max_y } = points.1;
            for x in min_x..max_x {
                for y in min_y..max_y {
                    if x > 0 && y > 0 && x < imgx && y < imgy {
                        let point = Point { x: x, y: y };
                        if circle.hit(&point) {
                            if old_style {
                                current_buf.put_pixel(x, y, circle.color().clone());
                            } else {
                                let pixel = current_buf.get_pixel(x, y).clone();
                                current_buf.put_pixel(x, y, sum_pixel_values(&circle.color(), &pixel));
                            }
                        }
                    }
                }
            }
        }

        if (final_file.is_none() || peek) && ((i % peek_size) == 0 || i == runs - 1) {
            let name = format!("results/run_{}.png", i);
            let ref mut fout = Path::new(&name);
            let _ = current_buf.save(fout);
        }

        list = vec![current_buf];

        if i % 10_000 == 0 {
            println!("Iteration #{:?}", i);
        }
    }
    list[0].clone()
}
fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| {
            println!("{:?}", e);
            e.exit()
        });
    let file = format!("{}", args.get_str("--base"));
    let reference = image::open(&Path::new(&file)).unwrap().to_rgba();
    let new_image = make_me_an_image(&args, &reference);

    let final_file = if args.get_str("--out") == "" {
        None
    } else {
        Some(args.get_str("--out"))
    };

    if let Some(file) = final_file {
        let name = format!("{}.png", file);
        let ref mut fout = File::create(&Path::new(&name)).unwrap();
        let _ = image::ImageRgba8(new_image).save(fout, image::PNG);
    }
}

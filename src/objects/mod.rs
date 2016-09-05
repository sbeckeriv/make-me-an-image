use image;
use image::Rgba;
use rand;
use std::cmp;
use rand::distributions::{IndependentSample, Range};
pub fn random_color() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    let color_between = Range::new(0, 255);
    let alpha_between = Range::new(0, 255);
    Rgba([color_between.ind_sample(&mut rng),
          color_between.ind_sample(&mut rng),
          color_between.ind_sample(&mut rng),
          alpha_between.ind_sample(&mut rng)])
}
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub trait Hitable {
    fn debug(&self) -> ();
    fn hit(&self, pixel: &Point) -> bool;
    fn color(&self) -> &Rgba<u8>;
    fn fitness(&self, source: &image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> isize;
    fn pixel_box(&self) -> (Point, Point);
    fn color_fitness(&self, generated_color: &Rgba<u8>, source_color: &Rgba<u8>) -> isize {
        let mut fitness = 0;
        let s = source_color.data[0] as isize;
        let g = generated_color.data[0] as isize;
        fitness += (s - g) * (s - g);
        let s = source_color.data[1] as isize;
        let g = generated_color.data[1] as isize;
        fitness += (s - g) * (s - g);
        let s = source_color.data[2] as isize;
        let g = generated_color.data[2] as isize;
        fitness += (s - g) * (s - g);
        fitness
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
    pub color: Rgba<u8>,
}

impl Circle {
    pub fn random(x: u32, y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let radius_between = Range::new(2.0, 4.0);
        let x_between = Range::new(0, x);
        let y_between = Range::new(0, y);

        Circle {
            center: Point {
                x: x_between.ind_sample(&mut rng),
                y: y_between.ind_sample(&mut rng),
            },
            radius: radius_between.ind_sample(&mut rng),
            color: random_color(),
        }
    }
}

impl Hitable for Circle {
    fn debug(&self) {
        println!("{:?}", self);
    }
    fn hit(&self, pixel: &Point) -> bool {
        let Point { x: cx, y: cy } = self.center;
        let Point { x, y } = *pixel;
        let mid = ((cx - x) * (cx - x) + (cy - y) * (cy - y)) as f32;
        let r = (mid).sqrt();
        r <= self.radius
    }

    fn color(&self) -> &Rgba<u8> {
        &self.color
    }

    fn pixel_box(&self) -> (Point, Point) {
        let radius_int = (self.radius + 0.5) as u32;

        let max_x = self.center.x + radius_int;
        let min_x = if self.center.x > radius_int {
            self.center.x - radius_int
        } else {
            0
        };

        let max_y = self.center.y + radius_int;
        let min_y = if self.center.y > radius_int {
            self.center.y - radius_int
        } else {
            0
        };
        (Point {
            x: min_x,
            y: min_y,
        },
         Point {
            x: max_x,
            y: max_y,
        })
    }

    fn fitness(&self, source: &image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> isize {
        let imgx = source.width();
        let imgy = source.height();
        let mut fitness = 0;
        let (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y }) = self.pixel_box();
        for x in min_x..max_x {
            for y in min_y..max_y {
                if x > 0 && y > 0 && x < imgx && y < imgy {
                    let point = Point { x: x, y: y };
                    if self.hit(&point) {
                        fitness += self.color_fitness(&self.color(), source.get_pixel(x, y));
                    }
                }
            }
        }
        fitness
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub color: Rgba<u8>,
}
impl Triangle {
    pub fn random(x: u32, y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let range = Range::new(3, 10);
        let direction = Range::new(0, 4);

        let x_between = Range::new(0, x);
        let y_between = Range::new(0, y);
        let center = Point {
            x: x_between.ind_sample(&mut rng),
            y: y_between.ind_sample(&mut rng),
        };
        let distance = range.ind_sample(&mut rng) as u32;
        let the_direction = direction.ind_sample(&mut rng);
        let (a, b, c) = match the_direction {
            0 => {
                (Point {
                    x: center.x + distance,
                    y: center.y,
                },
                 Point {
                    x: cmp::min(x, center.x - distance),
                    y: center.y,
                },
                 Point {
                    x: center.x,
                    y: center.y + distance,
                })
            }

            1 => {
                (Point {
                    x: center.x + distance,
                    y: center.y,
                },
                 Point {
                    x: cmp::min(x, center.x - distance),
                    y: center.y,
                },
                 Point {
                    x: center.x,
                    y: cmp::min(y, center.y - distance),
                })
            }

            2 => {
                (Point {
                    x: center.x,
                    y: center.y + distance,
                },
                 Point {
                    x: center.x,
                    y: cmp::min(y, center.y - distance),
                },
                 Point {
                    x: center.x + distance,
                    y: center.y,
                })
            }
            _ => {
                (Point {
                    x: center.x,
                    y: center.y + distance,
                },
                 Point {
                    x: center.x,
                    y: cmp::min(y, center.y - distance),
                },
                 Point {
                    x: cmp::min(x, center.x - distance),
                    y: center.y,
                })
            }
        };

        Triangle {
            a: a,
            b: b,
            c: c,
            color: random_color(),
        }
    }
}

impl Hitable for Triangle {
    fn debug(&self) {
        println!("{:?}", self);
    }
    fn color(&self) -> &Rgba<u8> {
        &self.color
    }
    // http://totologic.blogspot.fr/2014/01/accurate-point-in-triangle-test.html
    fn hit(&self, pixel: &Point) -> bool {
        let Point { x, y } = *pixel;
        let Point { x: x1, y: y1 } = self.a;
        let Point { x: x2, y: y2 } = self.b;
        let Point { x: x3, y: y3 } = self.c;
        let denominator = ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
        let a = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) as f32 / denominator;
        let b = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) as f32 / denominator;
        let c = 1.0 - a - b;

        0.0 <= a && a <= 1.0 && 0.0 <= b && b <= 1.0 && 0.0 <= c && c <= 1.0
    }

    fn pixel_box(&self) -> (Point, Point) {
        let min_x = cmp::min(self.a.x, cmp::min(self.b.x, self.c.x));
        let min_y = cmp::min(self.a.y, cmp::min(self.b.y, self.c.y));
        let max_x = cmp::max(self.a.x, cmp::max(self.b.x, self.c.x));
        let max_y = cmp::max(self.a.y, cmp::max(self.b.y, self.c.y));
        (Point {
            x: min_x,
            y: min_y,
        },
         Point {
            x: max_x,
            y: max_y,
        })
    }
    fn fitness(&self, source: &image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> isize {
        let imgx = source.width();
        let imgy = source.height();
        let mut fitness = 0;
        let (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y }) = self.pixel_box();
        for x in min_x..max_x {
            for y in min_y..max_y {
                if x > 0 && y > 0 && x < imgx && y < imgy {
                    let point = Point { x: x, y: y };
                    if self.hit(&point) {
                        let px = x as u32;
                        let py = y as u32;
                        fitness += self.color_fitness(&self.color(), source.get_pixel(px, py));

                    }
                }
            }
        }
        fitness
    }
}

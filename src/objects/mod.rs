use image::Rgba;
use rand;
use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};
use std::cmp;

pub fn random_color() -> Rgba<u8>{

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = rand::thread_rng();
    //let mut rng: StdRng = SeedableRng::from_seed(seed);
    let color_between = Range::new(0, 255);
    let alpha_between = Range::new(0, 255);
    Rgba([
         color_between.ind_sample(&mut rng),
         color_between.ind_sample(&mut rng),
         color_between.ind_sample(&mut rng),
         alpha_between.ind_sample(&mut rng)
    ])

}
pub struct Point{
    pub x: i32,
    pub y: i32
}

pub trait Hitable {
    fn hit(&self, pixel: &Point) -> bool;
    fn color(&self) -> &Rgba<u8>;
}

pub struct Circle{
    pub center: Point,
    pub radius: f32,
    pub color: Rgba<u8>
}
impl Circle{
    pub fn random(x: u32, y:u32) -> Self{
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng = rand::thread_rng();
        //let mut rng: StdRng = SeedableRng::from_seed(seed);
        let radius_max =  cmp::min(x/4, y/4) as f32;
        let radius_between = Range::new(2.0, 4.0);
        let x_between = Range::new(0, x as i32);
        let y_between = Range::new(0, y as i32);

        Circle{
            center: Point{
                x: x_between.ind_sample(&mut rng),
                y: y_between.ind_sample(&mut rng)
            },
            radius: radius_between.ind_sample(&mut rng),
            color: random_color()
        }
    }
}

impl Hitable for Circle{
    fn hit(&self, pixel: &Point) -> bool{
        let Point{x:cx,y:cy} = self.center;
        let Point{x,y} = *pixel;
        let mid = ((cx-x).pow(2)+(cy-y).pow(2)) as f32;
        let r = (mid).sqrt();
        r <= self.radius
    }

    fn color(&self) -> &Rgba<u8>{
        &self.color
    }
}

pub struct Triangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub color: Rgba<u8>
}
impl Triangle{
    pub fn random(x: u32, y: u32) -> Self{

        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng = rand::thread_rng();
        //let mut rng: StdRng = SeedableRng::from_seed(seed);
        let range = Range::new(5.0, 10.0);
        let direction = Range::new(0, 4);

        let x_between = Range::new(0, x as i32);
        let y_between = Range::new(0, y as i32);

        let center = Point{
            x: x_between.ind_sample(&mut rng),
            y: y_between.ind_sample(&mut rng)
        };
        let distance = range.ind_sample(&mut rng) as i32;
        let (a,b,c) = match direction.ind_sample(&mut rng){
            0 =>{
                (
                    Point{
                        x: center.x + distance,
                        y: center.y
                    },
                    Point{
                        x: center.x - distance,
                        y: center.y
                    },
                    Point{
                        x: center.x ,
                        y: center.y + distance
                    },
                    )
            },

            1 =>{
                (
                    Point{
                        x: center.x + distance,
                        y: center.y
                    },
                    Point{
                        x: center.x - distance,
                        y: center.y
                    },
                    Point{
                        x: center.x ,
                        y: center.y - distance
                    },
                    )
            },

            2 =>{
                (
                    Point{
                        x: center.x,
                        y: center.y + distance
                    },
                    Point{
                        x: center.x ,
                        y: center.y - distance
                    },
                    Point{
                        x: center.x + distance,
                        y: center.y
                    },
                    )
            },
            _ =>{
                (
                    Point{
                        x: center.x,
                        y: center.y + distance
                    },
                    Point{
                        x: center.x ,
                        y: center.y - distance
                    },
                    Point{
                        x: center.x - distance,
                        y: center.y
                    },
                    )
            },
        };
        Triangle{a: a, b: b, c:c, color: random_color()}
    }
}

impl Hitable for Triangle{
    fn color(&self) -> &Rgba<u8>{
        &self.color
    }
    //http://totologic.blogspot.fr/2014/01/accurate-point-in-triangle-test.html
    fn hit(&self, pixel: &Point) -> bool{
        let Point{x,y} = *pixel;
        let Point{x: x1, y: y1} = self.a;
        let Point{x: x2, y: y2} = self.b;
        let Point{x: x3, y: y3} = self.c;
        let denominator = ((y2 - y3)*(x1 - x3) + (x3 - x2)*(y1 - y3)) as f32;
        let a = ((y2 - y3)*(x - x3) + (x3 - x2)*(y - y3)) as f32 / denominator;
        let b= ((y3 - y1)*(x - x3) + (x1 - x3)*(y - y3)) as f32 / denominator ;
        let c = 1.0 - a - b;

        0.0 <= a && a <= 1.0 && 0.0 <= b && b <= 1.0 && 0.0 <= c && c <= 1.0
    }
}

use image::Rgba;

pub struct Point{
   pub x: i32,
   pub y: i32
}

pub trait Hit {
    fn hit(&self, pixel: &Point) -> bool;
}

pub struct Circle{
    pub center: Point,
    pub radius: f32,
    pub color: Rgba<u8>

}

impl Circle{
    pub fn color(&self) -> &Rgba<u8>{
        &self.color
    }
}

impl Hit for Circle{
    fn hit(&self, pixel: &Point) -> bool{
        let Point{x:cx,y:cy} = self.center;
        let Point{x,y} = *pixel;
        let mid = ((cx-x).pow(2)+(cy-y).pow(2)) as f32;
        let r = (mid).sqrt();
        r <= self.radius
    }
}

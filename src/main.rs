extern crate image;
use std::fs::File;
use std::path::Path;

struct Point{
    x: i32,
    y: i32
}

struct Circle{
    center: Point,
    radius: f32
}

impl Circle{
    fn hit(&self, pixel: Point) -> bool{
        let Point{x:cx,y:cy} = self.center;
        let Point{x,y} = pixel;
        let mid = ((cx-x).pow(2)+(cy-y).pow(2)) as f32;
        let r = (mid).sqrt();
        r <= self.radius
    }
}
fn main() {

    let imgx = 800;
    let imgy = 800;
    let circle = Circle{center: Point{x:400, y:400}, radius: 20.0};
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if circle.hit(Point{x: x as i32, y: y as i32}){
            *pixel = image::Rgba([255, 0, 0, 255]);
        }

    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PNG);
}

use crate::vec3::Vec3;
use std::io::Write;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }
}
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, t: f64) -> Color {
        Color::new(self.0.e[0] * t, self.0.e[1] * t, self.0.e[2] * t)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(
            self.0.e[0] + other.0.e[0],
            self.0.e[1] + other.0.e[1],
            self.0.e[2] + other.0.e[2],
        )
    }
}

pub fn write_color<W: Write>(mut out: W, pixel_color: Color) {
    // Write the translated [0,255] value of each color component.
    write!(
        out,
        "{} {} {} \n",
        (255.999 * pixel_color.0.x()) as i32,
        (255.999 * pixel_color.0.y()) as i32,
        (255.999 * pixel_color.0.z()) as i32,
    )
    .unwrap();
}

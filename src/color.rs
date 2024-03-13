use crate::vec3::Vec3;
use std::io::Write;
use std::ops::{Add, Mul, AddAssign};
use crate::interval::*;

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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
    
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    f64::sqrt(linear_component)
}

pub fn write_color<W: Write>(mut out: W, pixel_color: Color, samples_per_pixel: i32) {

    let mut r: f64 = pixel_color.0.x();
    let mut g: f64 = pixel_color.0.y();
    let mut b: f64 = pixel_color.0.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    
    let intensity: Interval = Interval::new(0.000, 0.999);
    // Write the translated [0,255] value of each color component.
    write!(
        out,
        "{} {} {} \n",
        (255.999 * intensity.clamp(r)) as i32,
        (255.999 * intensity.clamp(g)) as i32,
        (255.999 * intensity.clamp(b)) as i32,
    )
    .unwrap();
}

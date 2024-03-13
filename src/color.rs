use crate::{interval::*, vec3::*};
use std::io::Write;
use std::ops::{Add, AddAssign, Mul};

/// Represents a color in RGB space.
#[derive(Debug, Copy, Clone)]
pub struct Color(pub Vec3);

impl Color {
    /// Constructs a new color with all components set to zero.
    pub fn zero() -> Self {
        Color(Vec3::new(0.0, 0.0, 0.0))
    }

    /// Constructs a new color with the specified RGB components.
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    /// Converts a linear color component to gamma space.
    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        f64::sqrt(linear_component)
    }

    /// Writes a color to a stream in PPM format.
    pub fn write_color<W: Write>(mut out: W, pixel_color: Color, samples_per_pixel: i32) {
        let mut r: f64 = pixel_color.0.x();
        let mut g: f64 = pixel_color.0.y();
        let mut b: f64 = pixel_color.0.z();

        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        r = Color::linear_to_gamma(r);
        g = Color::linear_to_gamma(g);
        b = Color::linear_to_gamma(b);

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
}

// Implementations of mathematical operations using Rust traits

impl Mul<f64> for Color {
    type Output = Color;

    /// Multiplies the color by a scalar value.
    fn mul(self, t: f64) -> Color {
        Color::new(self.0.e[0] * t, self.0.e[1] * t, self.0.e[2] * t)
    }
}

impl Mul for Color {
    type Output = Color;

    /// Multiplies two colors component-wise.
    fn mul(self, other: Self) -> Self {
        Color::new(
            self.0.x() * other.0.x(),
            self.0.y() * other.0.y(),
            self.0.z() * other.0.z(),
        )
    }
}

impl Add<Color> for Color {
    type Output = Color;

    /// Adds two colors component-wise.
    fn add(self, other: Color) -> Color {
        Color::new(
            self.0.e[0] + other.0.e[0],
            self.0.e[1] + other.0.e[1],
            self.0.e[2] + other.0.e[2],
        )
    }
}

impl AddAssign for Color {
    /// Adds another color to this color.
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

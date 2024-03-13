use crate::{color::Color, rtweekend::*};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

/// A 3D vector representation.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    /// Returns a new Vec3 with all elements set to zero.
    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    /// Returns a new Vec3 with the specified elements.
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    /// Converts the Vec3 into a Color.
    pub fn into_color(self) -> Color {
        Color::new(self.e[0], self.e[1], self.e[2])
    }

    /// Returns the x-coordinate of the Vec3.
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    /// Returns the y-coordinate of the Vec3.
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    /// Returns the z-coordinate of the Vec3.
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// Returns the length (magnitude) of the Vec3.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the squared length of the Vec3 (more efficient for some calculations).
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Returns true if the Vec3 is near zero, false otherwise.
    pub fn near_zero(self) -> bool {
        let s: f64 = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

    /// Returns a random Vec3 with each element in the range [0, 1).
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    /// Returns a random Vec3 with each element in the range [min, max).
    pub fn random_r(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    /// Returns the dot product of two Vec3.
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    /// Returns the cross product of two Vec3.
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    /// Returns the unit vector of the Vec3.
    pub fn unit_vector(mut v: Vec3) -> Vec3 {
        v /= v.length();
        v
    }

    /// Returns a random Vec3 in the unit disk.
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns a random Vec3 in the unit sphere.
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_r(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns a random unit vector.
    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    /// Returns the reflection of the Vec3.
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - (n * (Vec3::dot(&v, &n) * 2.0))
    }

    /// Returns the refraction of the Vec3.
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&(-*uv), n), 1.0);
        let r_out_perp = (*uv + (*n * cos_theta)) * etai_over_etat;
        let r_out_parallel = *n * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }
}

// Alias for Vec3 representing a 3D point
pub type Point3 = Vec3;

// Implementations of mathematical operations using Rust traits

impl Neg for Vec3 {
    type Output = Vec3;
    /// Negates the Vec3.
    fn neg(self) -> Self {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    /// Adds two Vec3 together.
    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    /// Adds another Vec3 to the current Vec3.
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    /// Subtracts another Vec3 from the current Vec3.
    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    /// Multiplies the Vec3 by a scalar value.
    fn mul(self, t: f64) -> Self {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    /// Multiplies two Vec3 component-wise.
    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl MulAssign<f64> for Vec3 {
    /// Multiplies the Vec3 by a scalar value and assigns the result to the current Vec3.
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    /// Divides the Vec3 by a scalar value.
    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

impl DivAssign<f64> for Vec3 {
    /// Divides the Vec3 by a scalar value.
    fn div_assign(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

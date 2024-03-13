use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};
use crate::{color::*, rtweekend::{random_double, random_double_range}};
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

impl Vec3 {
    // Default constructor (all elements 0.0)
    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    // Constructor with specific elements
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn into_color(self) -> Color {
        Color::new(self.e[0], self.e[1], self.e[2])
    }

    // Accessor methods for individual elements
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    // Negation operator (returns a new vector with negated elements)
    pub fn neg(self) -> Self {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }

    // Indexing operator (immutable access)
    pub fn index(&self, i: usize) -> f64 {
        self.e[i]
    }

    // Mutable indexing operator
    pub fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }

    // Addition assignment operator
    pub fn add_assign(&mut self, v: &Vec3) {
        self.e[0] += v.e[0];
        self.e[1] += v.e[1];
        self.e[2] += v.e[2];
    }

    // Multiplication assignment operator (scalar multiplication)
    pub fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }

    // Division assignment operator (division by scalar)
    pub fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }

    // Length (magnitude) of the vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // Squared length of the vector (more efficient for some calculations)
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    
    pub fn random_r(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_double_range(min, max), random_double_range(min,max), random_double_range(min,max))
    }

    // Dot product of two vectors
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    // Cross product of two vectors
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    // Unit vector (vector with a magnitude of 1)
    pub fn unit_vector(mut v: Vec3) -> Vec3 {
        v /= v.length();
        v
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_r(-1.0,1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn randon_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere: Vec3 = Vec3::random_unit_vector();
        if (Vec3::dot(&on_unit_sphere, normal)) > 0.0 {
            return on_unit_sphere;
        }
        Vec3::neg(on_unit_sphere)
    }
}

// Alias for Vec3 representing a 3D point
pub type Point3 = Vec3;

// Implementations of mathematical operations using Rust traits

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

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

    fn mul(self, t: f64) -> Self {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

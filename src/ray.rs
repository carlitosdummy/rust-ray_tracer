use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    // The origin point of the ray.
    orig: Point3,
    // The direction vector of the ray.
    dir: Vec3,
}

impl Ray {
    /// Creates a new `Ray` with default settings.
    pub fn default() -> Self {
        Ray {
            orig: Vec3::zero(),
            dir: Vec3::zero(),
        }
    }
    /// Constructs a new Ray with the specified components.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    // Returns the origin of the ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    // Returns the direction of the ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Computes the point along the ray at a given parameter t.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

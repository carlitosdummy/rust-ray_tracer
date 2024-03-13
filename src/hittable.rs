use crate::{interval::*, material::*, ray::*, vec3::*};
use std::rc::Rc;

/// Represents information about a ray hit.
#[derive(Clone)]
pub struct HitRecord {
    /// Point of intersection with the object.
    pub p: Point3,
    /// Normal vector at the point of intersection.
    pub normal: Vec3,
    /// Parameter along the ray where the intersection occurred.
    pub t: f64,
    /// Indicates if the ray hit the front face or the back face of the object.
    pub front_face: bool,
    /// Material of the object at the point of intersection.
    pub mat: Option<Rc<dyn Material>>,
}

impl HitRecord {
    /// Sets the face normal based on the intersection details.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    /// Creates a default `HitRecord`.
    pub(crate) fn default() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: None,
        }
    }
}

/// Represents a hittable object in the scene.
pub trait Hittable {
    /// Determines if a ray intersects with the object.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

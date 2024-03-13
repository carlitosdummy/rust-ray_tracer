use crate::{hittable::*, interval::*, material::*, ray::*, vec3::*};
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    // Center point of the sphere.
    center: Point3,
    // Radius of the sphere.
    radius: f64,
    // Material of the sphere (if any).
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    // Constructs a new sphere with the given center, radius, and material.
    pub fn new(center: Point3, radius: f64, mat: Option<Rc<dyn Material>>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    // Implements the hit function for the sphere.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Vector from ray origin to sphere center.
        let oc: Vec3 = r.origin() - self.center;
        // Squared length of the ray direction.
        let a: f64 = r.direction().length_squared();
        // Half of the dot product of oc and ray direction.
        let half_b: f64 = Vec3::dot(&oc, &r.direction());
        // Squared distance from ray origin to sphere center minus radius squared.
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        // Discriminant of the quadratic equation.
        let discriminat: f64 = half_b * half_b - a * c;
        if discriminat < 0.0 {
            return false;
        }
        // Square root of the discriminant.
        let sqrtd: f64 = f64::sqrt(discriminat);

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        // Record the hit point parameter.
        rec.t = root;
        // Compute the hit point.
        rec.p = r.at(rec.t);
        // Compute the outward normal.
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        // Set the face normal of the hit record.
        rec.set_face_normal(r, &outward_normal);
        // Set the material of the hit record.
        rec.mat = self.mat.as_ref().map(|m| Rc::clone(m));
        true
    }
}

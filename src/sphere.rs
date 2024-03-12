use crate::hittable::*;
use crate::interval::*;
use crate::vec3::*;
use crate::ray::*;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self,r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = Vec3::dot(&oc, &r.direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminat: f64 = half_b * half_b - a*c;
        if discriminat < 0.0 {
            return false;
        }
        let sqrtd: f64 = f64::sqrt(discriminat);

        //Find the nearest root that lies in the acceptable range
        let mut root: f64 = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root){
                return false
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal:Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }
}






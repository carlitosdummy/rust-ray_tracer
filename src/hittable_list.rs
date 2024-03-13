use std::rc::Rc;
use crate::{hittable::*, interval::*, ray::*};

/// Represents a collection of hittable objects in the scene.
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    /// Creates a new empty `HittableList`.
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    /// Adds a hittable object to the list.
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
}

impl Hittable for HittableList {
    /// Checks if the ray intersects with any object in the list.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}

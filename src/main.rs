use std::rc::Rc;

mod color;
mod ray;
mod vec3;
mod hittable;
mod rtweekend;
mod sphere;
mod hittable_list;
mod camera;
mod interval;
use sphere::*;
use crate::{camera::Camera, hittable_list::HittableList, vec3::Point3};



fn main(){
    let mut world:HittableList  = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam: Camera = Camera::default(); // Initialize cam with default value
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    let _ = cam.render(&world); 
}

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
mod material;
use sphere::Sphere;
use vec3::Point3;

use crate::material::*;
use crate::color::*;
use crate::{camera::Camera, hittable_list::HittableList};



fn main(){
    let mut world:HittableList  = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8),0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(material_ground))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Some(material_center))));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left))));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(material_right))));

    let mut cam: Camera = Camera::default(); // Initialize cam with default value
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    let _ = cam.render(&world); 
}

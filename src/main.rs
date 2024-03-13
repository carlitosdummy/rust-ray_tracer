
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
use rand::random;
use rtweekend::random_double;
use rtweekend::random_double_range;
use sphere::Sphere;
use vec3::Point3;
use vec3::Vec3;

use crate::material::*;
use crate::color::*;
use crate::{camera::Camera, hittable_list::HittableList};



fn main(){
    let mut world:HittableList  = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 100.0, Some(ground_material))));

    for a in -11..11 {
        for b in -11..1 {
            let choose_mat = random_double();
            let center: Point3 = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo:Vec3 = Vec3::random() *Vec3::random();
                    sphere_material = Rc::new(Lambertian::new(albedo.into_color()));
                    world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                } else if choose_mat < 0.95 {
                    let albedo:Vec3 = Vec3::random_r(0.5,1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo.into_color(), fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5)); 
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Some(material1))));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))); 
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(material2))));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)); 
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 01.0), 1.0, Some(material3))));

    let mut cam: Camera = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let _ = cam.render(&world); 
}

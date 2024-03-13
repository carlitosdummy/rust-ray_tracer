
use std::f64::INFINITY;
use std::io::Write;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;
use crate::color::*;
use crate::rtweekend::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height:i32,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,

}

impl Camera {

    pub fn default() -> Self {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            samples_per_pixel: 10,
            max_depth: 10,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(mut self, world: &dyn Hittable) -> std::io::Result<()> {
        Camera::initialize(&mut self);
    
        let mut output: std::fs::File = std::fs::File::create("output.ppm")?;
        write!(output, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;
    
        for j in 0..self.image_height {
            println!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {

                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = Camera::get_ray(&mut self, i, j);
                    pixel_color += Camera::ray_color(&r, self.max_depth,world);
                }

                write_color(&output, pixel_color, self.samples_per_pixel);
            }
        }
        println!("Done");
        Ok(())
    }

    fn get_ray(&mut self, i: i32, j:i32) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + Camera::pixel_sample_square(&self);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
    
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
    
        self.center = Point3::new(0.0, 0.0, 0.0);
    
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * self.aspect_ratio;
    
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
    
        let viewport_upper_left = self.center - Vec3::new(viewport_width / 2.0, -viewport_height / 2.0, focal_length);
        self.pixel00_loc = viewport_upper_left + self.pixel_delta_u * 0.5 + self.pixel_delta_v * 0.5;
    }
    

    fn ray_color(r: &Ray, depth: i32 ,world: &dyn Hittable) -> Color {
        let mut rec: HitRecord = HitRecord::default();

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let direction: Vec3 = rec.normal + Vec3::random_unit_vector();
            return Camera::ray_color(&Ray::new(rec.p, direction), depth -1,world) * 0.9;
        }

        let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + (Color::new(0.5, 0.7, 1.0) * a)
    }
}

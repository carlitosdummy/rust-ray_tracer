
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
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height:i32,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

}

impl Camera {

    pub fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 10,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
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

        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {Camera::defocus_disk_sample(self)};
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.e[0]) + (self.defocus_disk_v * p.e[1])
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
    
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
    
        self.center = self.lookfrom;
    
        //let focal_length: f64 = (self.lookfrom - self.lookat).length();

        let theta: f64 = degrees_to_radians(self.vfov);
        let h: f64 = (theta/2.0).tan();
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 = viewport_height * (self.image_width as f64/ self.image_height as f64);
    
        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = Vec3::neg(self.v) * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
    
        let viewport_upper_left = self.center - (self.w * self.focus_dist) - viewport_u/2.0 - viewport_v/2.0;
        //self.pixel00_loc = viewport_upper_left + self.pixel_delta_u * 0.5 + self.pixel_delta_v * 0.5;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    

    fn ray_color(r: &Ray, depth: i32 ,world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
    
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            if let Some(material) = &rec.mat {
                let mut scattered = Ray::default();
                let mut attenuation = Color::zero();
                if material.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * Camera::ray_color(&scattered, depth - 1, world);
                }
            }
    
            let direction = rec.normal + Vec3::random_unit_vector();
            return Camera::ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.9;
        }
    
        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + (Color::new(0.5, 0.7, 1.0) * a)
    }
    
}

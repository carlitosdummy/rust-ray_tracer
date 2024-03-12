
use std::f64::INFINITY;
use std::io::Write;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;
use crate::color::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
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
                let pixel_center: Vec3 = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction: Vec3 = pixel_center - self.center;
                let r: Ray = Ray::new(self.center, ray_direction);
                let pixel_color: Color = Camera::ray_color(&r, world);
                write_color(&output, pixel_color);
            }
        }
        Ok(())
    }
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_u / self.image_height as f64;

        let viewport_upper_left: Vec3 =
        self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc: Vec3 = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec: HitRecord = HitRecord::default();
        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return ((rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5).into_color();
        }

        let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + (Color::new(0.5, 0.7, 1.0) * a)
    }
}

use crate::ray::*;
use crate::hittable::*;
use crate::color::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal{
    albedo: Color,
    fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_unit_vector() * self.fuzz);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal { 
            albedo,
            fuzz,
        }
    }
}




use crate::{color::*, hittable::*, ray::*, rtweekend::*, vec3::*};

/// Represents a material that can interact with rays in the scene.
pub trait Material {
    /// Computes the scattered ray and attenuation after interaction.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

/// Lambertian material, representing matte surfaces with diffuse reflection.
#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    // Represents the color of the material, controlling its appearance and interaction with light.
    albedo: Color,
}

impl Lambertian {
    /// Creates a new Lambertian material with the given albedo color.
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

/// Metal material, representing reflective surfaces.
#[derive(Debug, Copy, Clone)]
pub struct Metal {
    // Represents the color of the material, controlling its appearance and interaction with light.
    albedo: Color,
    // Controls the amount of fuzziness or roughness of the material's reflective surface.
    fuzz: f64,
}

impl Metal {
    /// Creates a new Metal material with the given albedo color and fuzziness factor.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_unit_vector() * self.fuzz);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

/// Dielectric material, representing transparent materials like glass.
#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    // Refractive index of the material, determining how light bends as it passes through.
    ir: f64,
}

impl Dielectric {
    /// Creates a new Dielectric material with the given index of refraction.
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    /// Computes the reflectance of a dielectric surface.
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = Vec3::unit_vector(r_in.direction());
        let cos_theta: f64 = f64::min(Vec3::dot(&(-unit_direction), &rec.normal), 1.0);
        let sin_theta: f64 = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

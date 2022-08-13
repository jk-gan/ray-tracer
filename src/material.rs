use crate::{
    color::Color, hittable::HitRecord, random_f64, random_in_unit_sphere, random_unit_vertor,
    ray::Ray,
};
use glam::DVec3;

pub trait Material {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vertor();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected = reflect(in_ray.direction().normalize(), hit_record.normal);
        *scattered_ray = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        scattered_ray.direction().dot(hit_record.normal) > 0.0
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = in_ray.direction().normalize();

        let cos_theta = f64::min(-unit_direction.dot(hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cant_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cant_refract || Self::reflectance(cos_theta, refraction_ratio) > random_f64() {
                reflect(unit_direction, hit_record.normal)
            } else {
                refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        *scattered_ray = Ray::new(hit_record.point, direction);
        true
    }
}

/// Returns true if the vector is close to zero in all dimensions.
fn near_zero(vector: &DVec3) -> bool {
    let epsilon = 1e-8;

    return (f64::abs(vector.x) < epsilon)
        && (f64::abs(vector.y) < epsilon)
        && (f64::abs(vector.z) < epsilon);
}

fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    // let v = view.clone();
    // let n = normal.clone();

    v - 2.0 * v.dot(n) * n
}

fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(f64::abs(1.0 - r_out_perp.length_squared())).sqrt() * n;

    r_out_perp + r_out_parallel
}

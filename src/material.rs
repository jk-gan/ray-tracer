use crate::{
    color::Color, hittable::HitRecord, random_in_unit_sphere, random_unit_vertor, ray::Ray,
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
        let reflected = reflect(&in_ray.direction().normalize(), &hit_record.normal);
        *scattered_ray = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        scattered_ray.direction().dot(hit_record.normal) > 0.0
    }
}

/// Returns true if the vector is close to zero in all dimensions.
fn near_zero(vector: &DVec3) -> bool {
    let epsilon = 1e-8;

    return (f64::abs(vector.x) < epsilon)
        && (f64::abs(vector.y) < epsilon)
        && (f64::abs(vector.z) < epsilon);
}

fn reflect(view: &DVec3, normal: &DVec3) -> DVec3 {
    let v = view.clone();
    let n = normal.clone();

    v - 2.0 * v.dot(n) * n
}

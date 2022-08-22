use crate::{
    color::Color, hittable::HitRecord, random_f64, random_in_unit_sphere, random_unit_vertor,
    ray::Ray, texture::Texture, Point3,
};
use glam::DVec3;
use std::sync::Arc;

pub enum Material {
    Lambertian { albedo: Arc<dyn Texture> },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
    DiffuseLight { emit: Arc<dyn Texture> },
    Isotropic { albedo: Arc<dyn Texture> },
}

impl Material {
    pub fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        match self {
            Material::DiffuseLight { emit } => emit.value(u, v, point),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn scatter(&self, in_ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vertor();

                // Catch degenerate scatter direction
                if near_zero(&scatter_direction) {
                    scatter_direction = hit_record.normal;
                }

                let scattered_ray = Ray::new(hit_record.point, scatter_direction, in_ray.time);
                let attenuation = albedo.value(hit_record.u, hit_record.v, &hit_record.point);

                Some((attenuation, scattered_ray))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(in_ray.direction.normalize(), hit_record.normal);
                let scattered_ray = Ray::new(
                    hit_record.point,
                    reflected + *fuzz * random_in_unit_sphere(),
                    in_ray.time,
                );
                let attenuation = albedo.clone();

                if scattered_ray.direction.dot(hit_record.normal) > 0.0 {
                    Some((attenuation, scattered_ray))
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if hit_record.front_face {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };
                let unit_direction = in_ray.direction.normalize();

                let cos_theta = f64::min(-unit_direction.dot(hit_record.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cant_refract = refraction_ratio * sin_theta > 1.0;

                let direction = if cant_refract
                    || schlick_reflectance(cos_theta, refraction_ratio) > random_f64()
                {
                    reflect(unit_direction, hit_record.normal)
                } else {
                    refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                let scattered_ray = Ray::new(hit_record.point, direction, in_ray.time);
                Some((attenuation, scattered_ray))
            }
            Material::DiffuseLight { emit: _emit } => None,
            Material::Isotropic { albedo } => {
                let scattered_ray = Ray::new(hit_record.point, random_unit_vertor(), in_ray.time);
                let attenuation = albedo.value(hit_record.u, hit_record.v, &hit_record.point);

                Some((attenuation, scattered_ray))
            }
        }
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
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(f64::abs(1.0 - r_out_perp.length_squared())).sqrt() * n;

    r_out_perp + r_out_parallel
}

// Use Schlick's approximation for reflectance.
fn schlick_reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

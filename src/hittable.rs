use crate::{
    color::Color,
    material::{Lambertian, Material},
    ray::Ray,
    DVec3, Point3,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: DVec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &DVec3) {
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }

    pub fn empty() -> Self {
        Self {
            point: (0.0, 0.0, 0.0).into(),
            normal: (0.0, 0.0, 0.0).into(),
            material: Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: material.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin().clone() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(*ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut hit_record = HitRecord::empty();
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        hit_record.normal = (hit_record.point - self.center) / self.radius;

        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);
        hit_record.material = self.material.clone();

        Some(hit_record)
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<Box<dyn Hittable>>>,
}

impl HittableList {
    pub fn add(&mut self, hittable_object: Box<dyn Hittable>) {
        self.objects.push(Arc::new(hittable_object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_hit_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                Some(hitted_record) => {
                    closest_so_far = hitted_record.t;
                    temp_hit_record = Some(hitted_record);
                }
                None => continue,
            }
        }

        temp_hit_record
    }
}

use crate::{ray::Ray, DVec3, Point3};
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: DVec3,
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
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin().clone() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(*ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        hit_record.normal = (hit_record.point - self.center) / self.radius;
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        true
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<Box<dyn Hittable>>>,
}

impl HittableList {
    pub fn add(&mut self, hittable_object: Box<dyn Hittable>) {
        self.objects.push(Rc::new(hittable_object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                closest_so_far = temp_hit_record.t;
                *hit_record = temp_hit_record.clone();
            }
        }

        hit_anything
    }
}

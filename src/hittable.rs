use crate::{
    aabb::Aabb, interval::Interval, material::Material, ray::Ray, texture::SolidColor, DVec3,
    Point3,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: DVec3,
    pub material: Arc<Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
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
            material: Arc::new(Material::Lambertian {
                albedo: Arc::new(SolidColor::from_rgb(0.0, 0.0, 0.0)),
            }),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<Material>,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<Material>) -> Self {
        let radius_vec = DVec3::new(radius, radius, radius);

        Self {
            center,
            radius,
            material,
            bounding_box: Aabb::from_points(&(center - radius_vec), &(center + radius_vec)),
        }
    }

    fn get_sphere_uv(point: &Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
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
        if !ray_t.contains(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord::empty();
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);

        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        let (u, v) = Self::get_sphere_uv(&outward_normal);
        hit_record.u = u;
        hit_record.v = v;

        hit_record.material = self.material.clone();

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

pub struct MovingSphere {
    center_0: Point3, // center at time = 0
    center_1: Point3, // center at time = 1
    center_vec: DVec3,
    radius: f64,
    material: Arc<Material>,
    bounding_box: Aabb,
}

impl MovingSphere {
    pub fn new(center_0: Point3, center_1: Point3, radius: f64, material: Arc<Material>) -> Self {
        let radius_vec = DVec3::new(radius, radius, radius);
        let box_0 = Aabb::from_points(&(center_0 - radius_vec), &(center_0 + radius_vec));
        let box_1 = Aabb::from_points(&(center_1 - radius_vec), &(center_1 + radius_vec));

        Self {
            center_0,
            center_1,
            center_vec: DVec3::from(center_1 - center_0),
            radius,
            material,
            bounding_box: Aabb::from_aabbs(&box_0, &box_1),
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        // Linearly interpolate from center_0 to center_1 according to time,
        // where
        // t = 0, yields center_0,
        // t = 1, yields center_1
        self.center_0 + time * self.center_vec
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = ray.origin().clone() - self.center(ray.time());
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
        if !ray_t.contains(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord::empty();
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);

        let outward_normal = (hit_record.point - self.center(ray.time())) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);
        hit_record.material = self.material.clone();

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<Box<dyn Hittable>>>,
    bounding_box: Aabb,
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        let mut list = Self::default();
        list.add(object);
        list
    }

    pub fn add(&mut self, hittable_object: Box<dyn Hittable>) {
        self.bounding_box = Aabb::from_aabbs(&self.bounding_box, hittable_object.bounding_box());
        self.objects.push(Arc::new(hittable_object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut temp_hit_record = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            match object.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                Some(hitted_record) => {
                    closest_so_far = hitted_record.t;
                    temp_hit_record = Some(hitted_record);
                }
                None => continue,
            }
        }

        temp_hit_record
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

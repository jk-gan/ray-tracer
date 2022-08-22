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
    pub fn face_normal(&mut self, ray: &Ray, outward_normal: DVec3) -> (bool, DVec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;
}

#[derive(Clone)]
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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
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
        let (front_face, normal) = hit_record.face_normal(ray, outward_normal);
        hit_record.normal = normal;
        hit_record.front_face = front_face;

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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
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

        let outward_normal = (hit_record.point - self.center(ray.time)) / self.radius;
        let (front_face, normal) = hit_record.face_normal(ray, outward_normal);
        hit_record.normal = normal;
        hit_record.front_face = front_face;
        hit_record.material = self.material.clone();

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bounding_box: Aabb,
}

impl HittableList {
    pub fn new(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::default();
        list.add(object);
        list
    }

    pub fn add(&mut self, hittable_object: Arc<dyn Hittable>) {
        self.objects.push(hittable_object.clone());
        self.bounding_box = Aabb::from_aabbs(&self.bounding_box, hittable_object.bounding_box());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_hit_record = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            match object.hit(&ray, Interval::new(ray_t.min, closest_so_far)) {
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

pub struct Quad {
    q: Point3,
    u: DVec3,
    v: DVec3,
    material: Arc<Material>,
    bounding_box: Aabb,
    normal: DVec3,
    d: f64,
    w: DVec3,
}

impl Quad {
    pub fn new(q: Point3, u: DVec3, v: DVec3, material: Arc<Material>) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.dot(n);

        let mut quad = Self {
            q,
            u,
            v,
            material,
            bounding_box: Aabb::default(),
            normal,
            d,
            w,
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        self.bounding_box = Aabb::from_points(&self.q, &(self.q + self.u + self.v));
    }

    fn is_interior(a: f64, b: f64) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the primitive,
        // otherwise return true

        !((a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b))
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        // No hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return None if the hit point parameter t is outside the ray interval
        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let mut hit_record = HitRecord::empty();

        // Determine the hit point lies within the planar shape using its plane coordinates
        let intersection = ray.at(t);
        let planar_hit_point = intersection - self.q;
        let alpha = self.w.dot(planar_hit_point.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_point));

        if Self::is_interior(alpha, beta) {
            hit_record.u = alpha;
            hit_record.v = beta;
        } else {
            return None;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return it
        hit_record.t = t;
        hit_record.point = intersection;
        hit_record.material = self.material.clone();
        let (front_face, normal) = hit_record.face_normal(ray, self.normal);
        hit_record.normal = normal;
        hit_record.front_face = front_face;

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

pub fn create_box(a: &Point3, b: &Point3, material: Arc<Material>) -> HittableList {
    // returns the 3D box (six sides) that contains the two opposite vertices a & b
    let mut sides = HittableList::default();

    // Construct the two opposite vertices with the minimum and maximum coordinates
    let min = Point3::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));
    let max = Point3::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));

    let dx = DVec3::new(max.x - min.x, 0.0, 0.0);
    let dy = DVec3::new(0.0, max.y - min.y, 0.0);
    let dz = DVec3::new(0.0, 0.0, max.z - min.z);

    // front
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        material.clone(),
    )));
    // right
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        -dz,
        dy,
        material.clone(),
    )));
    // back
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        material.clone(),
    )));
    // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        material.clone(),
    )));
    // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        material.clone(),
    )));
    // bottom
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        material.clone(),
    )));

    sides
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: DVec3,
    bounding_box: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, displacement: &DVec3) -> Self {
        let bounding_box = object.bounding_box() + displacement;
        Self {
            object,
            offset: *displacement,
            bounding_box,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        // Determine where (if any) an intersection occurs along the offset_ray
        if let Some(mut hitted_record) = self.object.hit(&offset_ray, ray_t) {
            hitted_record.point += self.offset;
            return Some(hitted_record);
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

pub struct RotationY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Aabb,
}

impl RotationY {
    pub fn new(object: Arc<dyn Hittable>, angle_in_degrees: f64) -> Self {
        let angle_in_radians = angle_in_degrees.to_radians();
        let sin_theta = angle_in_radians.sin();
        let cos_theta = angle_in_radians.cos();
        let bounding_box = object.bounding_box();

        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(-f64::MAX, -f64::MAX, -f64::MAX);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bounding_box.x.max + (1 - i) as f64 * bounding_box.x.min;
                    let y = j as f64 * bounding_box.y.max + (1 - j) as f64 * bounding_box.y.min;
                    let z = k as f64 * bounding_box.z.max + (1 - k) as f64 * bounding_box.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = DVec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bounding_box = Aabb::from_points(&min, &max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotationY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Change the ray from world space to object space
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        // Determine where (if any) an intersection occurs in object space
        if let Some(mut hitted_record) = self.object.hit(&rotated_ray, ray_t) {
            // Change the intersection point from object space to world space
            let mut point = hitted_record.point;
            point[0] =
                self.cos_theta * hitted_record.point[0] + self.sin_theta * hitted_record.point[2];
            point[2] =
                -self.sin_theta * hitted_record.point[0] + self.cos_theta * hitted_record.point[2];

            // Change the normal from object space to world space
            let mut normal = hitted_record.normal;
            normal[0] =
                self.cos_theta * hitted_record.normal[0] + self.sin_theta * hitted_record.normal[2];
            normal[2] = -self.sin_theta * hitted_record.normal[0]
                + self.cos_theta * hitted_record.normal[2];

            hitted_record.point = point;
            hitted_record.normal = normal;

            return Some(hitted_record);
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

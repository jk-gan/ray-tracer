use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    random_isize_range,
    ray::Ray,
};
use std::{cmp::Ordering, sync::Arc};

enum BBoxCompareAxis {
    X,
    Y,
    Z,
}

pub struct Bvh {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: Aabb,
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if self.bounding_box().hit(ray, ray_t) {
            let hit_left = self.left.hit(ray, ray_t);
            let hit_right = match hit_left {
                Some(ref hit_record) => self.right.hit(ray, Interval::new(ray_t.min, hit_record.t)),
                None => self.right.hit(ray, Interval::new(ray_t.min, ray_t.max)),
            };
            if hit_right.is_some() {
                hit_right
            } else {
                hit_left
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

impl Bvh {
    pub fn new(source_objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = random_isize_range(0, 2);

        let mut objects = source_objects.clone();

        let bounding_box_compare_axis = match axis {
            0 => BBoxCompareAxis::X,
            1 => BBoxCompareAxis::Y,
            _ => BBoxCompareAxis::Z,
        };

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match end - start {
            1 => {
                let left = objects[start].clone();
                let right = objects[start].clone();
                (left, right)
            }
            2 => {
                if Self::box_compare(
                    objects[start].clone(),
                    objects[start + 1].clone(),
                    bounding_box_compare_axis,
                ) {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            x => {
                objects.sort_unstable_by(Self::box_compare_a(bounding_box_compare_axis));
                let mid = start + x / 2;
                // let left = Arc::new(Box::new(Self::new(source_objects, start, mid)));
                // let right = Arc::new(Box::new(Self::new(source_objects, mid, end)));
                // (left, right)
                (
                    Arc::new(Self::new(&objects, start, mid)),
                    Arc::new(Self::new(&objects, mid, end)),
                )
            }
        };

        let bounding_box = Aabb::from_aabbs(left.bounding_box(), right.bounding_box());
        Self {
            left,
            right,
            bounding_box,
        }
    }

    pub fn from_list(list: HittableList) -> Self {
        Self::new(&list.objects, 0, list.objects.len())
    }

    fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: BBoxCompareAxis) -> bool {
        let axis_index = match axis {
            BBoxCompareAxis::X => 0,
            BBoxCompareAxis::Y => 1,
            BBoxCompareAxis::Z => 2,
        };

        a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min
    }

    fn box_compare_a(
        axis: BBoxCompareAxis,
    ) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        let axis_index = match axis {
            BBoxCompareAxis::X => 0,
            BBoxCompareAxis::Y => 1,
            BBoxCompareAxis::Z => 2,
        };

        move |a, b| {
            a.bounding_box()
                .axis(axis_index)
                .min
                .partial_cmp(&b.bounding_box().axis(axis_index).min)
                .unwrap()
        }
    }
}

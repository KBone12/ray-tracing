use std::ops::RangeBounds;

use cgmath::{InnerSpace, Point3, Vector3};

use crate::Ray;

pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit<R: Clone + RangeBounds<f64>>(self, ray: &Ray, t_range: R) -> Option<HitRecord>;
}

impl<H: Hittable, I: IntoIterator<Item = H>> Hittable for I {
    fn hit<R: Clone + RangeBounds<f64>>(self, ray: &Ray, t_range: R) -> Option<HitRecord> {
        self.into_iter()
            .filter_map(|hittable| hittable.hit(ray, t_range.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).expect("Hit objects did not found"))
    }
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl<'sphere> Hittable for &'sphere Sphere {
    fn hit<R: Clone + RangeBounds<f64>>(self, ray: &Ray, t_range: R) -> Option<HitRecord> {
        let vec_from_center = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = vec_from_center.dot(ray.direction);
        let c = vec_from_center.dot(vec_from_center) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root = (-half_b - discriminant.sqrt()) / a;
            if t_range.contains(&root) {
                let t = root;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                let front_face = ray.direction.dot((p - self.center) / self.radius) < 0.0;
                Some(HitRecord {
                    p,
                    normal: if front_face { normal } else { -normal },
                    t,
                    front_face,
                })
            } else {
                let root = (-half_b + discriminant.sqrt()) / a;
                if t_range.contains(&root) {
                    let t = root;
                    let p = ray.at(t);
                    let normal = (p - self.center) / self.radius;
                    let front_face = ray.direction.dot((p - self.center) / self.radius) < 0.0;
                    Some(HitRecord {
                        p,
                        normal: if front_face { normal } else { -normal },
                        t,
                        front_face,
                    })
                } else {
                    None
                }
            }
        }
    }
}

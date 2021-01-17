use std::{ops::RangeBounds, rc::Rc};

use cgmath::{InnerSpace, Point3, Vector3};

use crate::{material::Material, Ray};

pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Rc<Box<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit<R: Clone + RangeBounds<f64>>(&self, ray: &Ray, t_range: R) -> Option<HitRecord>;
}

impl<H: Hittable> Hittable for Vec<H> {
    fn hit<R: Clone + RangeBounds<f64>>(&self, ray: &Ray, t_range: R) -> Option<HitRecord> {
        self.iter()
            .filter_map(|hittable| hittable.hit(ray, t_range.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).expect("Hit objects did not found"))
    }
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Rc<Box<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit<R: Clone + RangeBounds<f64>>(&self, ray: &Ray, t_range: R) -> Option<HitRecord> {
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
                    material: Rc::clone(&self.material),
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
                        material: Rc::clone(&self.material),
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

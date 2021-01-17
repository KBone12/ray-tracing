use cgmath::{AbsDiffEq, InnerSpace, Vector3, Zero};
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{hittable::HitRecord, Color, Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
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
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let direction = record.normal + random_unit_vector();
        let direction = if direction.abs_diff_eq(&Vector3::zero(), f64::EPSILON) {
            record.normal
        } else {
            direction
        };
        Some((Ray::new(record.p, direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let normalized_ray_direction = ray.direction.normalize();
        let reflected = normalized_ray_direction
            - 2.0 * normalized_ray_direction.dot(record.normal) * record.normal;
        if reflected.dot(record.normal) > 0.0 {
            Some((Ray::new(record.p, reflected), self.albedo))
        } else {
            None
        }
    }
}

fn random_unit_vector() -> Vector3<f64> {
    let distribution = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    loop {
        let x = distribution.sample(&mut rng);
        let y = distribution.sample(&mut rng);
        let z = distribution.sample(&mut rng);
        if x * x + y * y + z * z <= 1.0 {
            return Vector3::new(x, y, z).normalize();
        }
    }
}

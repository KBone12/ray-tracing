use std::sync::{Arc, Mutex};

use cgmath::{AbsDiffEq, InnerSpace, Vector3, Zero};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::{hittable::HitRecord, Color, Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian<R: Rng> {
    albedo: Color,
    rng: Arc<Mutex<R>>,
}

impl<R: Rng> Lambertian<R> {
    pub fn new(albedo: Color, rng: Arc<Mutex<R>>) -> Self {
        Self { albedo, rng }
    }
}

impl<R: Rng> Material for Lambertian<R> {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let random_unit_vector = {
            let mut rng = self.rng.lock().expect("The mutex is poisoned");
            random_unit_vector(&mut *rng)
        };
        let direction = record.normal + random_unit_vector;
        let direction = if direction.abs_diff_eq(&Vector3::zero(), f64::EPSILON) {
            record.normal
        } else {
            direction
        };
        Some((Ray::new(record.p, direction), self.albedo))
    }
}

pub struct Metal<R: Rng> {
    albedo: Color,
    fuzz: f64,
    rng: Arc<Mutex<R>>,
}

impl<R: Rng> Metal<R> {
    pub fn new(albedo: Color, fuzz: f64, rng: Arc<Mutex<R>>) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
            rng,
        }
    }
}

impl<R: Rng> Material for Metal<R> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let normalized_ray_direction = ray.direction.normalize();
        let reflected = normalized_ray_direction
            - 2.0 * normalized_ray_direction.dot(record.normal) * record.normal;
        if reflected.dot(record.normal) > 0.0 {
            let random_vector = {
                let mut rng = self.rng.lock().expect("The mutex is poisoned");
                random_vector_in_unit_sphere(&mut *rng)
            };
            Some((
                Ray::new(record.p, reflected + self.fuzz * random_vector),
                self.albedo,
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let direction = ray.direction.normalize();
        let cos = (-direction.dot(record.normal)).min(1.0).max(-1.0);
        let sin = (1.0 - cos * cos).sqrt();
        let reflectance = {
            let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
            let r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cos).powi(5)
        };
        let direction = if refraction_ratio * sin > 1.0 || reflectance > random_double() {
            let normalized_ray_direction = ray.direction.normalize();
            normalized_ray_direction
                - 2.0 * normalized_ray_direction.dot(record.normal) * record.normal
        } else {
            let perp = refraction_ratio * (direction + cos * record.normal);
            let parallel = -((1.0 - perp.dot(perp)).abs().sqrt()) * record.normal;
            perp + parallel
        };
        Some((Ray::new(record.p, direction), Color::new(1.0, 1.0, 1.0)))
    }
}

fn random_vector_in_unit_sphere<R: Rng>(rng: &mut R) -> Vector3<f64> {
    let distribution = Uniform::from(0.0..1.0);
    loop {
        let x = distribution.sample(rng);
        let y = distribution.sample(rng);
        let z = distribution.sample(rng);
        if x * x + y * y + z * z <= 1.0 {
            return Vector3::new(x, y, z);
        }
    }
}

fn random_unit_vector<R: Rng>(rng: &mut R) -> Vector3<f64> {
    random_vector_in_unit_sphere(rng).normalize()
}

fn random_double() -> f64 {
    Uniform::from(0.0..1.0).sample(&mut rand::thread_rng())
}

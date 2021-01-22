use cgmath::{AbsDiffEq, InnerSpace, Vector3, Zero};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::{hittable::HitRecord, Color, Ray};

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal { albedo, fuzz }
    }

    pub fn new_dielectric(index_of_refraction: f64) -> Self {
        Self::Dielectric {
            index_of_refraction,
        }
    }

    pub fn scatter<R: Rng>(
        &self,
        ray: &Ray,
        record: &HitRecord,
        rng: &mut R,
    ) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian { albedo } => {
                let direction = record.normal + random_unit_vector(rng);
                let direction = if direction.abs_diff_eq(&Vector3::zero(), f64::EPSILON) {
                    record.normal
                } else {
                    direction
                };
                Some((Ray::new(record.p, direction), *albedo))
            }
            Self::Metal { albedo, fuzz } => {
                let normalized_ray_direction = ray.direction.normalize();
                let reflected = normalized_ray_direction
                    - 2.0 * normalized_ray_direction.dot(record.normal) * record.normal;
                if reflected.dot(record.normal) > 0.0 {
                    Some((
                        Ray::new(
                            record.p,
                            reflected + *fuzz * random_vector_in_unit_sphere(rng),
                        ),
                        *albedo,
                    ))
                } else {
                    None
                }
            }
            Self::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if record.front_face {
                    1.0 / *index_of_refraction
                } else {
                    *index_of_refraction
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

use cgmath::{InnerSpace, Point3, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::Ray;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        position: Point3<f64>,
        at: Point3<f64>,
        up: Vector3<f64>,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let origin = position;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        let lens_radius = aperture / 2.0;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn ray<R: Rng>(&self, s: f64, t: f64, rng: &mut R) -> Ray {
        let rd = self.lens_radius * random_vector_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_vector_in_unit_disk<R: Rng>(rng: &mut R) -> Vector3<f64> {
    let distribution = Uniform::from(0.0..1.0);
    loop {
        let x = distribution.sample(rng);
        let y = distribution.sample(rng);
        if x * x + y * y <= 1.0 {
            return Vector3::new(x, y, 0.0);
        }
    }
}

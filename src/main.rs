use std::io::Write;

use cgmath::{InnerSpace, Point3, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, Rng, SeedableRng};

mod camera;
mod hittable;
use crate::{
    camera::Camera,
    hittable::{Hittable, Sphere},
};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }
}

type Color = Vector3<f64>;

fn random_vector_in_unit_sphere<R: Rng>(rng: &mut R) -> Vector3<f64> {
    let distribution = Uniform::from(0.0..1.0);
    loop {
        let x = distribution.sample(rng);
        let y = distribution.sample(rng);
        let z = distribution.sample(rng);
        if x * x + y * y + z * z < 1.0 {
            return Vector3::new(x, y, z);
        }
    }
}

fn ray_color<H: Hittable, R: Rng>(ray: &Ray, hittable: &H, rng: &mut R, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let record = hittable.hit(ray, 0.0..);
    if let Some(record) = record {
        let target = record.p + record.normal + random_vector_in_unit_sphere(rng);
        return ray_color(
            &Ray::new(record.p, target - record.p),
            hittable,
            rng,
            depth - 1,
        ) / 2.0;
    }
    let unit_direction = ray.direction.normalize();
    let t = (unit_direction.y + 1.0) / 2.0;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color<W: Write>(mut writer: W, color: Color, samples_per_pixel: usize) {
    // with gamma-correction for gamma = 2.0
    let r = (color.x / samples_per_pixel as f64).sqrt();
    let g = (color.y / samples_per_pixel as f64).sqrt();
    let b = (color.z / samples_per_pixel as f64).sqrt();
    writeln!(
        writer,
        "{} {} {}",
        (256.0 * r.max(0.0).min(0.999)) as i32,
        (256.0 * g.max(0.0).min(0.999)) as i32,
        (256.0 * b.max(0.0).min(0.999)) as i32,
    )
    .expect("Couldn't write a color");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let hittables = vec![
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let camera = Camera::new();

    let distribution = Uniform::from(0.0..1.0);
    let mut rng = SmallRng::seed_from_u64(
        0b0101010101010101_0101010101010101_0101010101010101_0101010101010101,
    );

    // Print in PPM Image format
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255"); // max color
    for y in 0..IMAGE_HEIGHT {
        eprintln!("Scan lines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let color = (0..SAMPLES_PER_PIXEL).fold(Color::new(0.0, 0.0, 0.0), |acc, _| {
                let u = (x as f64 + distribution.sample(&mut rng)) / (IMAGE_WIDTH as f64 - 1.0);
                let v = ((IMAGE_HEIGHT - y) as f64 + distribution.sample(&mut rng))
                    / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.ray(u, v);
                acc + ray_color(&ray, &hittables, &mut rng, MAX_DEPTH)
            });
            write_color(std::io::stdout(), color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}

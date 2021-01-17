use std::io::Write;

use cgmath::{ElementWise, InnerSpace, Point3, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, Rng, SeedableRng};

mod camera;
mod hittable;
mod material;
use crate::{
    camera::Camera,
    hittable::{Hittable, Sphere},
    material::{Lambertian, Metal},
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

pub type Color = Vector3<f64>;

fn ray_color<H: Hittable, R: Rng>(ray: &Ray, hittable: &H, rng: &mut R, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let record = hittable.hit(ray, 0.001..);
    if let Some(record) = record {
        if let Some((scattered, attenuation)) = record.material.scatter(ray, &record) {
            return attenuation.mul_element_wise(ray_color(&scattered, hittable, rng, depth - 1));
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
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

    let distribution = Uniform::from(0.0..1.0);
    let mut rng = SmallRng::seed_from_u64(
        0b0101010101010101_0101010101010101_0101010101010101_0101010101010101,
    );

    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let hittable: Vec<Sphere> = vec![
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground),
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center),
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left),
        Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right),
    ];

    let camera = Camera::new();

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
                acc + ray_color(&ray, &hittable, &mut rng, MAX_DEPTH)
            });
            write_color(std::io::stdout(), color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}

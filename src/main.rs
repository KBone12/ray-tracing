use std::io::Write;

use cgmath::{InnerSpace, Point3, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, SeedableRng};

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

    pub fn color<H: Hittable>(&self, hittable: H) -> Color {
        let record = hittable.hit(&self, 0.0..);
        if let Some(record) = record {
            return Color::new(
                record.normal.x + 1.0,
                record.normal.y + 1.0,
                record.normal.z + 1.0,
            ) / 2.0;
        }
        let unit_direction = self.direction.normalize();
        let t = (unit_direction.y + 1.0) / 2.0;
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

type Color = Vector3<f64>;

fn write_color<W: Write>(mut writer: W, color: Color, samples_per_pixel: usize) {
    let r = color.x / samples_per_pixel as f64;
    let g = color.y / samples_per_pixel as f64;
    let b = color.z / samples_per_pixel as f64;
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
                acc + ray.color(&hittables)
            });
            write_color(std::io::stdout(), color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}

use std::io::Write;

use cgmath::{InnerSpace, Point3, Vector3};

mod hittable;
use crate::hittable::{Hittable, Sphere};

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

fn write_color<W: Write>(mut writer: W, color: Color) {
    writeln!(
        writer,
        "{} {} {}",
        (255.999 * color.x) as i32,
        (255.999 * color.y) as i32,
        (255.999 * color.z) as i32
    )
    .expect("Couldn't write a color");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let hittables = vec![
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Print in PPM Image format
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255"); // max color
    for y in 0..IMAGE_HEIGHT {
        eprintln!("Scan lines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let u = (x as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let v = ((IMAGE_HEIGHT - y) as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray.color(&hittables);
            write_color(std::io::stdout(), color);
        }
    }
    eprintln!("Done");
}

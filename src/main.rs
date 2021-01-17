use std::io::Write;

use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};

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

    pub fn color(&self) -> Color {
        let t = self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal = (self.at(t) - Vector3::new(0.0, 0.0, -1.0))
                .to_vec()
                .normalize();
            return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) / 2.0;
        }
        let unit_direction = self.direction.normalize();
        let t = (unit_direction.y + 1.0) / 2.0;
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: &Point3<f64>, radius: f64) -> f64 {
        let vec_from_center = self.origin - center;
        let a = self.direction.dot(self.direction);
        let half_b = vec_from_center.dot(self.direction);
        let c = vec_from_center.dot(vec_from_center) - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
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
            let color = ray.color();
            write_color(std::io::stdout(), color);
        }
    }
    eprintln!("Done");
}

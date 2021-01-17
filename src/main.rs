use std::{io::Write, rc::Rc};

use cgmath::{ElementWise, InnerSpace, Point3, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, Rng, SeedableRng};

mod camera;
mod hittable;
mod material;
use crate::{
    camera::Camera,
    hittable::{Hittable, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: usize = 50;

    let ground_material: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    let mut hittables = Vec::new();
    hittables.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let distribution = Uniform::from(0.0..1.0);
    let mut rng = SmallRng::seed_from_u64(
        0b0101010101010101_0101010101010101_0101010101010101_0101010101010101,
    );
    let dielectric: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    for a in -11..11 {
        for b in -11..11 {
            let material_probability = distribution.sample(&mut rng);
            let center = Point3::new(
                a as f64 + 0.9 * distribution.sample(&mut rng),
                0.2,
                b as f64 + 0.9 * distribution.sample(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0))
                .dot(center - Point3::new(4.0, 0.2, 0.0))
                .sqrt()
                > 0.9
            {
                let material: Rc<Box<dyn Material>> = if material_probability < 0.8 {
                    let albedo = Color::new(
                        distribution.sample(&mut rng),
                        distribution.sample(&mut rng),
                        distribution.sample(&mut rng),
                    )
                    .mul_element_wise(Color::new(
                        distribution.sample(&mut rng),
                        distribution.sample(&mut rng),
                        distribution.sample(&mut rng),
                    ));
                    Rc::new(Box::new(Lambertian::new(albedo)))
                } else if material_probability < 0.95 {
                    let distribution = Uniform::from(0.5..1.0);
                    let albedo = Color::new(
                        distribution.sample(&mut rng),
                        distribution.sample(&mut rng),
                        distribution.sample(&mut rng),
                    );
                    let distribution = Uniform::from(0.0..0.5);
                    let fuzz = distribution.sample(&mut rng);
                    Rc::new(Box::new(Metal::new(albedo, fuzz)))
                } else {
                    Rc::clone(&dielectric)
                };
                hittables.push(Sphere::new(center, 0.2, material));
            }
        }
    }
    hittables.push(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::clone(&dielectric),
    ));
    hittables.push(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))),
    ));
    hittables.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))),
    ));

    let camera_position = Point3::new(13.0, 3.0, 2.0);
    let camera_look_at = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let camera = Camera::new(
        camera_position,
        camera_look_at,
        up,
        20.0,
        ASPECT_RATIO,
        aperture,
        10.0,
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

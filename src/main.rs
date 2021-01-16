use std::io::Write;

use cgmath::Vector3;

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
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Print in PPM Image format
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255"); // max color
    for y in 0..IMAGE_HEIGHT {
        eprintln!("Scan lines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let color = Color::new(
                (x as f64) / (IMAGE_WIDTH as f64 - 1.0),
                ((IMAGE_HEIGHT - y) as f64) / (IMAGE_HEIGHT as f64 - 1.0),
                0.25,
            );
            write_color(std::io::stdout(), color);
        }
    }
    eprintln!("Done");
}

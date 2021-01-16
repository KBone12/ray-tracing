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
            let r = (x as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let g = ((IMAGE_HEIGHT - y) as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done");
}

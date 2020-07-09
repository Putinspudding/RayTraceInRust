use ray::Vec3;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() {
    println! {"P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT};
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in (0..IMAGE_WIDTH) {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25 as f32;
            let ir = unsafe { (255.999 * r).to_int_unchecked::<u16>() };
            let ig = unsafe { (255.999 * g).to_int_unchecked::<u16>() };
            let ib = unsafe { (255.999 * b).to_int_unchecked::<u16>() };
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone");
}

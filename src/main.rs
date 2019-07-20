mod vec3;
use vec3::*;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let m = 255.99_f64;
            let col = Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2);
            let r = (m * col.r()).round() as i32;
            let g = (m * col.g()).round() as i32;
            let b = (m * col.b()).round() as i32;
            println!("{} {} {}", r, g, b);
        }
    }
}

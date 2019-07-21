use raytracing_in_one_weekend::*;

fn color<T: Hitable>(ray: Ray, world: &T) -> Vec3 {
  let mut rec: HitRecord = Default::default();
  if world.hit(&ray, 0.0, std::f64::MAX, &mut rec) {
    return 0.5
      * Vec3::new(
        rec.normal.x() + 1.0,
        rec.normal.y() + 1.0,
        rec.normal.z() + 1.0,
      );
  } else {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
  }
}

const M: f64 = 255.99_f64;

fn main() {
  let nx = 400;
  let ny = 200;
  println!("P3\n{} {}\n255", nx, ny);

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  let world = HitableVec::new(vec![
    Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
    Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
  ]);

  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = i as f64 / nx as f64;
      let v = j as f64 / ny as f64;
      let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
      let col = color(r, &world);
      let r = (M * col.r()).round() as i32;
      let g = (M * col.g()).round() as i32;
      let b = (M * col.b()).round() as i32;
      println!("{} {} {}", r, g, b);
    }
  }
}

use rand::prelude::*;
use raytracing_in_one_weekend::*;

fn color<T: Hitable>(ray: Ray, world: &T) -> Vec3 {
  let mut rec: HitRecord = Default::default();
  if world.hit(&ray, 0.001, std::f64::MAX, &mut rec) {
    let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
    return 0.5 * color(Ray::new(rec.p, target - rec.p), world);
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
  let ns = 100;
  println!("P3\n{} {}\n255", nx, ny);

  let world = HitableVec::new(vec![
    Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
    Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
  ]);
  let cam = Camera::default();

  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::default();
      for _ in 0..ns {
        let u = (i as f64 + random::<f64>()) / nx as f64;
        let v = (j as f64 + random::<f64>()) / ny as f64;
        let r = cam.get_ray(u, v);
        col += color(r, &world);
      }
      col /= ns as f64;
      col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
      let r = (M * col.r()).round() as i32;
      let g = (M * col.g()).round() as i32;
      let b = (M * col.b()).round() as i32;
      println!("{} {} {}", r, g, b);
    }
  }
}

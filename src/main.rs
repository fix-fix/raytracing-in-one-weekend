use rand::prelude::*;
use raytracing_in_one_weekend::*;
use std::sync::Arc;

fn color<T: Hitable>(ray: Ray, world: &T, depth: i64) -> Vec3 {
  match world.hit(&ray, 0.001, std::f64::MAX) {
    Some(rec) => {
      if depth < 50 {
        if let Some(Scatter {
          attenuation,
          scattered,
        }) = rec.material.scatter(&ray, &rec)
        {
          return attenuation * color(scattered, world, depth + 1);
        }
      }
      Vec3::new(0.0, 0.0, 0.0)
    }
    None => {
      let unit_direction = ray.direction().unit_vector();
      let t = 0.5 * (unit_direction.y() + 1.0);
      return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
  }
}

const M: f64 = 255.99_f64;

fn main() {
  let nx = 400;
  let ny = 200;
  let ns = 100;

  let world = HitableVec::new(vec![
    Sphere::new(
      Vec3::new(0.0, 0.0, -1.0),
      0.5,
      Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    ),
    Sphere::new(
      Vec3::new(0.0, -100.5, -1.0),
      100.0,
      Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    ),
    Sphere::new(
      Vec3::new(1.0, 0.0, -1.0),
      0.5,
      Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    ),
    Sphere::new(
      Vec3::new(-1.0, 0.0, -1.0),
      0.5,
      Arc::new(Dielectric::new(1.5)),
    ),
    Sphere::new(
      Vec3::new(-1.0, 0.0, -1.0),
      -0.45,
      Arc::new(Dielectric::new(1.5)),
    ),
  ]);

  let lookfrom = Vec3::new(3., 3., 2.);
  let lookat = Vec3::new(0., 0., -1.);
  let dist_to_focus = (lookfrom - lookat).length();
  let aperture = 2.0;
  let cam = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    20.0,
    nx as f64 / ny as f64,
    aperture,
    dist_to_focus,
  );
  dbg!(cam);

  println!("P3\n{} {}\n255", nx, ny);
  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::default();
      for _ in 0..ns {
        let u = (i as f64 + random::<f64>()) / nx as f64;
        let v = (j as f64 + random::<f64>()) / ny as f64;
        let r = cam.get_ray(u, v);
        col += color(r, &world, 0);
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

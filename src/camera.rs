use crate::{ray::Ray, vec3::Vec3};
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  origin: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  lens_radius: f64,
}

pub fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = 2.0 * Vec3::new(random(), random(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    if Vec3::dot(p, p) < 1.0 {
      return p;
    }
  }
}

impl Camera {
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f64,
    aspect: f64,
    aperture: f64,
    focus_dist: f64,
  ) -> Self {
    let lens_radius = aperture / 2.0;
    let theta = vfov * std::f64::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let w = (lookfrom - lookat).unit_vector();
    let u = Vec3::cross(vup, w).unit_vector();
    let v = Vec3::cross(w, u);
    let origin = lookfrom;
    Self {
      lower_left_corner: origin
        - focus_dist * half_width * u
        - focus_dist * half_height * v
        - focus_dist * w,
      horizontal: 2.0 * focus_dist * half_width * u,
      vertical: 2.0 * focus_dist * half_height * v,
      origin,
      u,
      v,
      w,
      lens_radius,
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.y() + self.v * rd.y();
    Ray::new(
      self.origin + offset,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
    )
  }
}

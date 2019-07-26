use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  origin: Vec3,
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
    let theta = vfov * std::f64::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let w = (lookfrom - lookat).unit_vector();
    let u = Vec3::cross(vup, w).unit_vector();
    let v = Vec3::cross(w, u);
    let origin = lookfrom;
    Self {
      lower_left_corner: origin - half_width * u - half_height * v - w,
      horizontal: 2.0 * half_width * u,
      vertical: 2.0 * half_height * v,
      origin,
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    )
  }
}

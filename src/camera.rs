use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  origin: Vec3,
}

impl Camera {
  pub fn new(lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Self {
    Self {
      lower_left_corner,
      horizontal,
      vertical,
      origin,
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + u * self.horizontal + v * self.vertical,
    )
  }
}

impl Default for Camera {
  fn default() -> Self {
    Self::new(
      Vec3::new(-2.0, -1.0, -1.0),
      Vec3::new(4.0, 0.0, 0.0),
      Vec3::new(0.0, 2.0, 0.0),
      Vec3::new(0.0, 0.0, 0.0),
    )
  }
}

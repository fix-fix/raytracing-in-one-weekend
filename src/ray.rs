use crate::vec3::Vec3;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct Ray {
  A: Vec3,
  B: Vec3,
}

impl Ray {
  #[allow(non_snake_case)]
  pub fn new(A: Vec3, B: Vec3) -> Self {
    Ray { A, B }
  }

  pub fn origin(&self) -> Vec3 {
    self.A
  }

  pub fn direction(&self) -> Vec3 {
    self.B
  }

  pub fn point_at_parameter(&self, t: f64) -> Vec3 {
    self.A + t * self.B
  }
}

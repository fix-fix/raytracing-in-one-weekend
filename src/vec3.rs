pub struct Vec3 {
  e0: f64,
  e1: f64,
  e2: f64,
}

impl Vec3 {
  pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
    Vec3 { e0, e1, e2 }
  }
  pub fn r(&self) -> f64 {
    self.e0
  }
  pub fn g(&self) -> f64 {
    self.e1
  }
  pub fn b(&self) -> f64 {
    self.e2
  }
}

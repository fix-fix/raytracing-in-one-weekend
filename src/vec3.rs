use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
  e: [f64; 3],
}

impl Vec3 {
  pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
    Vec3 { e: [e0, e1, e2] }
  }

  pub fn length(self) -> f64 {
    (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
  }

  pub fn unit_vector(self) -> Self {
    self / self.length()
  }

  pub fn x(&self) -> f64 {
    self.e[0]
  }
  pub fn y(&self) -> f64 {
    self.e[1]
  }
  pub fn z(&self) -> f64 {
    self.e[2]
  }

  pub fn r(&self) -> f64 {
    self.e[0]
  }
  pub fn g(&self) -> f64 {
    self.e[1]
  }
  pub fn b(&self) -> f64 {
    self.e[2]
  }

  pub fn dot(v1: Self, v2: Self) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
  }
}

impl Default for Vec3 {
  fn default() -> Self {
    Vec3::new(0.0, 0.0, 0.0)
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3::new(
      self.e[0] + other.e[0],
      self.e[1] + other.e[1],
      self.e[2] + other.e[2],
    )
  }
}

impl Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Vec3 {
    Vec3::new(
      self.e[0] - other.e[0],
      self.e[1] - other.e[1],
      self.e[2] - other.e[2],
    )
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, other: Self::Output) -> Self::Output {
    Vec3::new(other.e[0] * self, other.e[1] * self, other.e[2] * self)
  }
}

impl Div for Vec3 {
  type Output = Vec3;

  fn div(self, other: Vec3) -> Self::Output {
    Vec3::new(
      self.e[0] / other.e[0],
      self.e[1] / other.e[1],
      self.e[2] / other.e[2],
    )
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, other: f64) -> Self::Output {
    Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
  }
}

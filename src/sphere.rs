use crate::{
  hitable::{HitRecord, Hitable},
  ray::Ray,
  vec3::Vec3,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
  radius: f64,
  center: Vec3,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Self {
    Sphere { center, radius }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc = r.origin() - self.center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - self.radius.powi(2);
    let discriminant = b.powi(2) - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
      temp = (-b + discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
    }
    return false;
  }
}

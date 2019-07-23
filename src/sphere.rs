use crate::{
  hitable::{HitRecord, Hitable},
  material::MaterialPtr,
  ray::Ray,
  vec3::Vec3,
};

#[derive(Clone)]
pub struct Sphere {
  radius: f64,
  center: Vec3,
  material: MaterialPtr,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64, material: MaterialPtr) -> Sphere {
    Sphere {
      center,
      radius,
      material,
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin() - self.center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - self.radius.powi(2);
    let discriminant = b.powi(2) - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        let t = temp;
        let p = r.point_at_parameter(temp);
        let normal = (p - self.center) / self.radius;
        return Some(HitRecord::new(t, p, normal, self.material.clone()));
      }
      temp = (-b + discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        let t = temp;
        let p = r.point_at_parameter(temp);
        let normal = (p - self.center) / self.radius;
        return Some(HitRecord::new(t, p, normal, self.material.clone()));
      }
    }
    None
  }
}

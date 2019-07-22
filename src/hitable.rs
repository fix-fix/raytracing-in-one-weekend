use crate::{material::Material, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
  pub fn new(t: f64, p: Vec3, normal: Vec3, material: &'a Material) -> Self {
    HitRecord {
      t,
      p,
      normal,
      material,
    }
  }
}

pub trait Hitable {
  #[allow(unused_variables)]
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    None
  }
}

pub struct HitableVec<T: Hitable> {
  vec: Vec<T>,
}

impl<T: Hitable> HitableVec<T> {
  pub fn new(vec: Vec<T>) -> Self {
    Self { vec }
  }
}

impl<T: Hitable> Hitable for HitableVec<T> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut hit_anything: Option<HitRecord> = None;
    let mut closest_so_far = t_max;
    for item in self.vec.iter() {
      if let Some(rec) = item.hit(r, t_min, closest_so_far) {
        closest_so_far = rec.t;
        hit_anything = Some(rec);
      }
    }
    hit_anything
  }
}

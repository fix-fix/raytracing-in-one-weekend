use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
}

impl HitRecord {
  pub fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
    Self { t, p, normal }
  }
}

pub trait Hitable {
  #[allow(unused_variables)]
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    false
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
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let mut temp_rec: HitRecord = Default::default();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for item in self.vec.iter() {
      if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec;
      }
    }
    hit_anything
  }
}

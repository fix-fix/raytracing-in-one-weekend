use crate::{hitable::HitRecord, ray::Ray, vec3::Vec3};
pub use lambertian::*;
pub use metal::*;
use std::sync::Arc;

pub const LAMBERTIAN: Lambertian = Lambertian::new(Vec3::new(0.0, 0.0, 0.0));

pub trait Material: Send + Sync {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub type MaterialPtr = Arc<Material>;

pub struct Scatter {
  pub attenuation: Vec3,
  pub scattered: Ray,
}

mod lambertian {
  use super::*;

  #[derive(Default, Clone, Copy)]
  pub struct Lambertian {
    albedo: Vec3,
  }

  impl Lambertian {
    pub const fn new(albedo: Vec3) -> Self {
      Self { albedo }
    }
  }

  impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
      let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
      return Some(Scatter {
        scattered: Ray::new(rec.p, target - rec.p),
        attenuation: self.albedo,
      });
    }
  }
}

mod metal {
  use super::*;

  #[derive(Default, Clone, Copy)]
  pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
  }

  impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
      Self {
        albedo,
        fuzz: fuzz.min(1.0),
      }
    }
  }

  impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
      let reflected = Vec3::reflect(ray_in.direction().unit_vector(), rec.normal);
      let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
      if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
        return Some(Scatter {
          scattered,
          attenuation: self.albedo,
        });
      }
      None
    }
  }
}

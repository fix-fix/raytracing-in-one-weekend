use crate::{hitable::HitRecord, ray::Ray, vec3::Vec3};
pub use dielectric::*;
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

mod dielectric {
  use super::*;
  use rand::prelude::*;

  #[derive(Default, Clone, Copy)]
  pub struct Dielectric {
    ref_idx: f64,
  }

  impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
      Self { ref_idx }
    }
  }

  impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
      let outward_normal: Vec3;
      let ni_over_nt: f64;
      let reflected = Vec3::reflect(ray_in.direction(), rec.normal);
      let attenuation = Vec3::new(1.0, 1.0, 1.0);
      let cosine: f64;
      if Vec3::dot(ray_in.direction(), rec.normal) > 0.0 {
        outward_normal = -rec.normal;
        ni_over_nt = self.ref_idx;
        cosine =
          self.ref_idx * Vec3::dot(ray_in.direction(), rec.normal) / ray_in.direction().length();
      } else {
        outward_normal = rec.normal;
        ni_over_nt = 1.0 / self.ref_idx;
        cosine = -Vec3::dot(ray_in.direction(), rec.normal) / ray_in.direction().length();
      }
      match (
        Vec3::refract(ray_in.direction(), outward_normal, ni_over_nt),
        shlick(cosine, self.ref_idx),
        random::<f64>(),
      ) {
        (Some(refracted), reflect_prob, r) if r >= reflect_prob => Some(Scatter {
          attenuation,
          scattered: Ray::new(rec.p, refracted),
        }),
        _ => Some(Scatter {
          attenuation,
          scattered: Ray::new(rec.p, reflected),
        }),
      }
    }
  }

  fn shlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
  }
}

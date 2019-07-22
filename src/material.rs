use crate::{hitable::HitRecord, ray::Ray, vec3::Vec3};
pub use lambertian::*;
pub use metal::*;

pub const LAMBERTIAN: Lambertian = Lambertian::new(Vec3::new(0.0, 0.0, 0.0));

pub trait Material: Send + Sync {
  fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>>;
}

pub struct Scatter<'a> {
  pub rec: &'a HitRecord<'a>,
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
    fn scatter<'a>(&self, _ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>> {
      let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
      return Some(Scatter {
        rec,
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
    fn scatter<'a>(&self, ray_in: &Ray, rec: &'a HitRecord) -> Option<Scatter<'a>> {
      let reflected = Vec3::reflect(ray_in.direction().unit_vector(), rec.normal);
      let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
      if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
        return Some(Scatter {
          rec,
          scattered,
          attenuation: self.albedo,
        });
      }
      None
    }
  }
}

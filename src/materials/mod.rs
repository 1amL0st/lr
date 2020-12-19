use crate::objects::{ HitData, Ray };

use crate::utils;
use crate::utils::NlmVec3Ext;
use crate::nlm;

type Scatter = (bool, nlm::Vec3, Ray);

pub trait Material: Send + Sync {
    fn scatter(&self, hit_data: &mut HitData) -> Scatter;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: nlm::Vec3
}

impl Lambertian {
    pub fn new(color: nlm::Vec3) -> Lambertian {
        Lambertian {
            albedo: color
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit_data: &mut HitData) -> Scatter {
        let mut new_ray_dir = utils::point_in_unit_sphere() + &hit_data.normal.dir;

        let s: f32 = 1e-6;
        if new_ray_dir.x.abs() < s && new_ray_dir.y.abs() < s && new_ray_dir.z.abs() < s {
            new_ray_dir = hit_data.normal.dir;
            panic!("You must remove this panic!");
        }

        (
            true,
            self.albedo,
            Ray::new(hit_data.normal.pos, new_ray_dir.normalize())
        )
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: nlm::Vec3,
    pub fuzinnes: f32,
}

impl Metal {
    pub fn new(color: nlm::Vec3, fuzinnes: f32) -> Metal {
        Metal {
            albedo: color,
            fuzinnes
        }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_data: &mut HitData) -> Scatter {
        let mut reflected = hit_data.ray.dir.reflect(&hit_data.normal.dir);
        reflected = reflected + &(utils::point_in_unit_sphere() * self.fuzinnes);

        let ray = Ray::new(hit_data.normal.pos, reflected);

        (
            ray.dir.dot(&hit_data.normal.dir) > 0.,
            self.albedo,
            ray
        )
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f32, // Index of refraction
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric {
            ir
        }
    }
}

use rand::random;

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}

fn refract(v: &nlm::Vec3, n: &nlm::Vec3, ni_over_nt: f32) -> Option<nlm::Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: &mut HitData) -> Scatter {
        let outward_normal: nlm::Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        let r_in = &hit.ray;

        if r_in.dir.dot(&hit.normal.dir) > 0.0 {
            outward_normal = -hit.normal.dir;
            ni_over_nt = self.ir;
            cosine = self.ir * r_in.dir.dot(&hit.normal.dir) / r_in.dir.magnitude();
        } else {
            outward_normal = hit.normal.dir;
            ni_over_nt = 1.0 / self.ir;
            cosine = -r_in.dir.dot(&hit.normal.dir) / r_in.dir.magnitude();
        }

        match refract(&r_in.dir, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                if random::<f32>() > schlick(cosine, self.ir) {
                    return (
                        true,
                        nlm::Vec3::new_color(255, 255, 255),
                        Ray::new(hit.normal.pos, refracted)
                    );
                }
            }
            None => {}
        }

        (
            true,
            nlm::Vec3::new_color(255, 255, 255),
            Ray::new(hit.normal.pos, r_in.dir.reflect(&hit.normal.dir))
        )
    }
}
use crate::objects::{ HitData, Ray };

use crate::utils;
use crate::utils::NlmVec3Ext;
use crate::nlm;

pub trait Material: Send + Sync {
    fn scatter(&self, hit_data: &mut HitData, color: &mut nlm::Vec3, ray: &mut Ray) -> bool;
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
    fn scatter(&self, hit_data: &mut HitData, color: &mut nlm::Vec3, ray: &mut Ray) -> bool {
        let mut new_ray_dir = utils::point_in_unit_sphere() + &hit_data.normal.dir;

        let s: f32 = 1e-6;
        if new_ray_dir.x.abs() < s && new_ray_dir.y.abs() < s && new_ray_dir.z.abs() < s {
            new_ray_dir = hit_data.normal.dir;
            panic!("You must remove this panic!");
        }

        ray.pos = hit_data.normal.pos;
        ray.dir = new_ray_dir;

        *color = self.albedo;

        return true;
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
    fn scatter(&self, hit_data: &mut HitData, color: &mut nlm::Vec3, ray: &mut Ray) -> bool {
        let mut reflected = hit_data.ray.dir.reflect(&hit_data.normal.dir);
        reflected = reflected + &(utils::point_in_unit_sphere() * self.fuzinnes);

        ray.pos = hit_data.normal.pos;
        ray.dir = reflected;

        *color = self.albedo;

        ray.dir.dot(&hit_data.normal.dir) > 0.
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

    fn refract(uv: &nlm::Vec3, n: &nlm::Vec3, eoe: f32) -> nlm::Vec3 {
        let cos_theta = nlm::dot(&(-uv), &n).min(1.0);
        let r_out_perp = eoe * (uv + (cos_theta * n));
        let fabs = (1.0 - nlm::length(&r_out_perp).powi(2)).abs();
        let r_out_parallel = -fabs.sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_data: &mut HitData, color: &mut nlm::Vec3, ray: &mut Ray) -> bool {
        *color = nlm::Vec3::new_color(255, 255, 255);

        let mut refraction_ratio = self.ir;
        if !hit_data.front_face {
            refraction_ratio = 1.0 / self.ir;
        }

        let unit_direction = hit_data.ray.dir;
        let refracted = Dielectric::refract(&unit_direction, &hit_data.normal.dir, refraction_ratio);

        *ray = Ray::new(hit_data.normal.pos, refracted);
        true
    }
}
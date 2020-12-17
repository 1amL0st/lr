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
        ray.dir = nlm::normalize(&new_ray_dir);

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
        let fabs = (1.0 - nlm::magnitude(&r_out_perp).powi(2)).abs();
        let r_out_parallel = -fabs.sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: &mut HitData, color: &mut nlm::Vec3, ray: &mut Ray) -> bool {
        *color = nlm::Vec3::new_color(255, 255, 255);

        let out_normal;
        let ratio;

        if hit.ray.dir.dot(&hit.normal.dir) > 0.0 {
            ratio = self.ir;
            out_normal = -hit.normal.dir;
        } else {
            ratio = 1.0 / self.ir;
            out_normal = hit.normal.dir;
        }

        let unit_dir = hit.normal.dir;
        let refracted = Dielectric::refract(&unit_dir, &out_normal, ratio);

        *ray = Ray::new(hit.normal.pos, refracted);
        true


        // let outward_normal: nlm::Vec3;
        // let ni_over_nt: f32;
        // let cosine: f32;

        // if hit.ray.dir.dot(&hit.normal.dir) > 0.0 {
        //     outward_normal = -hit.normal.dir;
        //     ni_over_nt = self.ir;
        //     cosine = self.ir * hit.ray.dir.dot(&hit.normal.dir) / hit.ray.dir.magnitude();
        // } else {
        //     outward_normal = hit.normal.dir;
        //     ni_over_nt = 1.0 / self.ir;
        //     cosine = -hit.ray.dir.dot(&hit.normal.dir) / hit.ray.dir.magnitude();
        // }

        // match refract(&hit.ray.dir, &outward_normal, ni_over_nt) {
        //     Some(refracted) => {
        //         if random::<f32>()>schlick(cosine, self.ir) {
        //             ray.pos = hit.normal.pos;
        //             ray.dir = refracted;
        //             return true;
        //             // return Scatter {
        //             //     color: WHITE,
        //             //     ray: Some(Ray::new(hit.p, refracted))
        //             // }
        //         }
        //     }
        //     None => {}
        // }

        // ray.pos = hit.normal.pos;
        // ray.dir = hit.ray.dir.reflect(&hit.normal.dir);
        // true
    }
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

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}
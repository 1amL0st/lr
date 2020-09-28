use crate::objects::geometry:: {Geometry, Ray, Normal, HitData };

extern crate math;
use math::vec3::Vec3;

pub struct Sphere {
    pos: Vec3,
    radius: f32,
    color: Vec3
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32, color: Vec3) -> Sphere {
        Sphere {
            pos,
            radius,
            color
        }
    }
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray) -> HitData {
        let len = ray.pos.sub(&self.pos);
        let a = ray.dir.dot(&ray.dir);
        let b = 2. * ray.dir.dot(&len);
        let c = len.dot(&len) - self.radius * self.radius;
        
        let d = b * b - 4. * a * c;

        let mut is_hit = d >= 0.0;
        let mut result_t = 0.;

        if is_hit {
            let sqr = d.sqrt();
            let t1 = (-b + sqr) / 2. * a;
            let t2 = (-b - sqr) / 2. * a;
            if t1 < 0. && t2 < 0. {
                is_hit = false;
            } else {
                result_t = t1.min(t2);
            }
        }

        HitData {
            is_hit: is_hit,
            t: result_t,
            normal: Normal::empty()
        }
    }

    fn get_color(&self, hit_data: &HitData) -> Vec3 {
        Vec3::copy(&self.color)
    }

    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal {
        let pos = ray.pos.add(&ray.dir.scale(hit_data.t));
        let mut dir = pos.sub(&self.pos);
        dir = dir.norm();
        Normal {
            pos: pos,
            dir: dir
        }
    }
}
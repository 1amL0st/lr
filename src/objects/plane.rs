use crate::objects::geometry:: {Geometry, Ray, Normal, HitData };

use math:: { Vec3 };

pub struct Plane {
    pub pos: Vec3,
    pub normal: Vec3,
    pub color: Vec3
}

impl Plane {
    pub fn new(pos: Vec3, normal: Vec3, color: Vec3) -> Plane {
        Plane {
            pos,
            normal,
            color
        }
    }
}

impl Geometry for Plane {
    fn hit(&self, ray: &Ray) -> HitData {
        let n = self.pos.dot(&self.normal) - ray.pos.dot(&self.normal);
        let d = ray.dir.dot(&self.normal);
        let t = n / d;
        HitData {
            is_hit: t >= 0.,
            t: t,
            normal: Normal::empty()
        }
    }

    fn get_color(&self, hit: &HitData) -> Vec3 {
        Vec3::copy(&self.color)
    }

    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal {
        let pos = ray.dir.scale(hit_data.t);
        Normal::new(pos, Vec3::copy(&self.normal))
    }
}
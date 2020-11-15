use crate::objects::{ Geometry, Ray, Normal, HitData };

use nlm;

pub struct Plane {
    pub pos: nlm::Vec3,
    pub normal: nlm::Vec3,
    pub color: nlm::Vec3
}

impl Plane {
    pub fn new(pos: nlm::Vec3, normal: nlm::Vec3, color: nlm::Vec3) -> Plane {
        Plane {
            pos,
            normal,
            color
        }
    }
}

use crate::constants;

impl Geometry for Plane {
    fn hit(&self, ray: &Ray) -> HitData {
        let n = self.pos.dot(&self.normal) - ray.pos.dot(&self.normal);
        let d = ray.dir.dot(&self.normal);
        let t = n / d;
        HitData {
            is_hit: t > 0.,
            t: t - constants::EPSILON,
            normal: Normal::empty()
        }
    }

    fn get_color(&self, hit: &HitData) -> nlm::Vec3 {
        nlm::Vec3::new(self.color.x, self.color.y, self.color.z)
    }

    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal {
        let pos = ray.pos + &(ray.dir * hit_data.t);
        let n = nlm::Vec3::new(self.normal.x, self.normal.y, self.normal.z);
        Normal::new(pos, n)
    }
}
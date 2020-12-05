use crate::objects::{ Geometry, Ray, Normal, HitData };
use crate::materials::{ Material, Lambertian };

use nlm;
use std::sync::Arc;
// use std::rc::Arc;

pub struct Plane {
    pub pos: nlm::Vec3,
    pub normal: nlm::Vec3,
    pub material: Arc<Material>
}

impl Plane {
    pub fn new(pos: nlm::Vec3, normal: nlm::Vec3, material: Arc<Material>) -> Plane {
        Plane {
            pos,
            normal,
            material
        }
    }

    pub fn matte_with_color(pos: nlm::Vec3, normal: nlm::Vec3, color: nlm::Vec3) -> Box<Plane> {
        let lambertian = Arc::new(Lambertian::new(color));
        Box::new(Plane::new(pos, normal, lambertian.clone()))
    }
}

use crate::constants;

impl Geometry for Plane {
    fn hit(&self, ray: &Ray) -> HitData {
        let n = self.pos.dot(&self.normal) - ray.pos.dot(&self.normal);
        let d = ray.dir.dot(&self.normal);
        let t = n / d;

        let mut hit_data = HitData::empty();
        hit_data.is_hit = t > 0.;
        hit_data.t = t - constants::EPSILON;
        hit_data
    }

    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal {
        let pos = ray.pos + &(ray.dir * hit_data.t);
        let n = nlm::Vec3::new(self.normal.x, self.normal.y, self.normal.z);
        Normal::new(pos, n)
    }

    fn set_pos(&mut self, new_pos: nlm::Vec3) {

    }

    fn rotate(&mut self, angels: nlm::Vec3) {

    }

    fn scale(&mut self, scale: nlm::Vec3) {
        
    }

    fn set_material(&mut self, material: Arc<Material>) {
        self.material = material;
    }

    fn get_mateial(&self) -> Arc<Material> {
        self.material.clone()
    }
}
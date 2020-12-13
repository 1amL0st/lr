use crate::objects::{ Geometry, Ray, Normal, HitData };
use crate::materials::{ Material, Lambertian };

use nlm;
use std::sync::Arc;
// use std::rc::Arc;

pub struct Plane {
    pub normal: nlm::Vec3,
    pub material: Arc<Material>,
    pub inverse_matrix: nlm::Mat4x4,
    pub matrix: nlm::Mat4x4,
}

impl Plane {
    pub fn new(pos: nlm::Vec3, normal: nlm::Vec3, material: Arc<Material>) -> Plane {
        let mut matrix = nlm::identity();
        matrix = nlm::translate(&matrix, &pos);
        let inverse_matrix = matrix.try_inverse().unwrap();

        Plane {
            normal,
            material,
            inverse_matrix,
            matrix,
        }
    }

    pub fn matte_with_color(pos: nlm::Vec3, normal: nlm::Vec3, color: nlm::Vec3) -> Box<Plane> {
        let lambertian = Arc::new(Lambertian::new(color));
        Box::new(Plane::new(pos, normal, lambertian.clone()))
    }
}

use crate::constants;

impl Geometry for Plane {
    fn hit(&self, r: &Ray) -> HitData {
        let ray = r.transform_to_matrix(&self.inverse_matrix);
        
        let n = nlm::Vec3::new(0., 0., 0.).dot(&self.normal) - ray.pos.dot(&self.normal);
        let d = ray.dir.dot(&self.normal);
        let t = n / d;

        let mut hit_data = HitData::empty();
        hit_data.is_hit = t > 0.;
        hit_data.t = t - constants::EPSILON;
        hit_data
    }

    fn get_normal(&self, r: &Ray, hit_data: &HitData) -> Normal {
        let ray = r.transform_to_matrix(&self.inverse_matrix);

        let mut pos = ray.pos + (ray.dir * hit_data.t);
        let mut dir = pos - nlm::Vec3::new(0., 0., 0.);

        pos = nlm::vec4_to_vec3(&(self.matrix * nlm::Vec4::new(pos.x, pos.y, pos.z, 1.)));
        dir = nlm::vec4_to_vec3(&(self.matrix * nlm::Vec4::new(dir.x, dir.y, dir.z, 0.)));

        Normal::new(pos, dir)
        // let pos = ray.pos + &(ray.dir * hit_data.t);
        // let n = nlm::Vec3::new(self.normal.x, self.normal.y, self.normal.z);
        // Normal::new(pos, n)
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
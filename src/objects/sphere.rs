use crate::objects::{ Geometry, Ray, Normal, HitData };
use crate::materials;

use materials::{ Material, Lambertian };
use nlm;
use std::sync::Arc;

// use std::rc::Arc;

use crate::constants;

pub struct Sphere {
    radius: f32,
    matrix: nlm::Mat4x4,
    inverse_matrix: nlm::Mat4x4,
    material: Arc<Material>
}

impl Sphere {
    pub fn new(pos: nlm::Vec3, radius: f32, material: Arc<Material>) -> Sphere {
        let mut matrix = nlm::identity();
        matrix = nlm::translate(&matrix, &pos);
        let inverse_matrix = matrix.try_inverse().unwrap();

        Sphere {
            radius,
            matrix,
            inverse_matrix,
            material
        }
    }

    pub fn matte_with_color(pos: nlm::Vec3, radius: f32, color: nlm::Vec3) -> Box<Sphere> {
        let lambertian = Arc::new(Lambertian::new(color));
        Box::new(Sphere::new(pos, radius, lambertian))
    }
}

impl Geometry for Sphere {
    fn hit(&self, r: &Ray) -> HitData {
        let ray = r.transform_to_matrix(&self.inverse_matrix);

        let len = ray.pos - nlm::Vec3::new(0., 0., 0.);
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

        let mut hit_data = HitData::empty();
        hit_data.is_hit = is_hit;
        hit_data.t = result_t - constants::EPSILON;
        hit_data
    }

    fn get_normal(&self, r: &Ray, hit_data: &HitData) -> Normal {
        let ray = r.transform_to_matrix(&self.inverse_matrix);

        let mut pos = ray.pos + (ray.dir * hit_data.t);
        let mut dir = pos - nlm::Vec3::new(0., 0., 0.);

        pos = nlm::vec4_to_vec3(&(self.matrix * nlm::Vec4::new(pos.x, pos.y, pos.z, 1.)));
        dir = nlm::vec4_to_vec3(&(self.matrix * nlm::Vec4::new(dir.x, dir.y, dir.z, 0.)));

        Normal {
            pos,
            dir
        }
    }

    fn set_pos(&mut self, new_pos: nlm::Vec3) {
        self.matrix.set_column(3, &nlm::Vec4::new(new_pos.x, new_pos.y, new_pos.z, 1.));
        self.inverse_matrix = self.matrix.try_inverse().unwrap();
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
use materials::Material;
use nlm;
use crate::materials;
use crate::utils::NlmVec3Ext;

pub mod sphere;
pub mod plane;

pub struct Ray
{
    pub pos: nlm::Vec3,
    pub dir: nlm::Vec3
}

impl Ray
{
    pub fn new(pos: nlm::Vec3, dir: nlm::Vec3) -> Ray {
        Ray {
            pos: pos,
            dir: dir
        }
    }

    pub fn zeros() -> Ray {
        Ray {
            pos: nlm::Vec3::zeros(),
            dir: nlm::Vec3::zeros()
        }
    }

    pub fn transform_to_matrix(&self, matrix: &nlm::Mat4x4) -> Ray {
        let r = self;
        let new_pos = matrix * nlm::Vec4::new(r.pos.x, r.pos.y, r.pos.z, 1.0);
        let new_dir = matrix * nlm::Vec4::new(r.dir.x, r.dir.y, r.dir.z, 0.0);
        Ray::new(nlm::vec4_to_vec3(&new_pos), nlm::vec4_to_vec3(&new_dir))
    }

    pub fn copy(&self) -> Ray {
        Ray {
            pos: self.pos.copy(),
            dir: self.dir.copy()
        }
    }
}

pub struct Normal
{
    pub pos: nlm::Vec3,
    pub dir: nlm::Vec3
}

impl Normal {
    pub fn new(pos: nlm::Vec3, dir: nlm::Vec3) -> Normal {
        Normal {
            pos,
            dir
        }
    }

    pub fn empty() -> Normal {
        Normal {
            pos: nlm::Vec3::new(0., 0., 0.),
            dir: nlm::Vec3::new(0., 0., 0.)
        }
    }

    pub fn clone(&self) -> Normal {
        Normal {
            pos: self.pos.clone(),
            dir: self.dir.clone()
        }
    }
}

pub struct HitData {
    pub is_hit: bool,
    pub t: f32,
    pub normal: Normal,
    pub ray: Ray,
    pub front_face: bool,
}

impl HitData {
    pub fn empty() -> HitData {
        let is_hit = false;
        return HitData {
            is_hit,
            t: 0.,
            normal: Normal::empty(),
            ray: Ray::zeros(),
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, out_normal: &nlm::Vec3) {
        self.front_face = ray.dir.dot(&out_normal) < 0.;
        if self.front_face {
            self.normal.dir = *out_normal;
        } else {
            self.normal.dir = -out_normal;
        }
    } 
}

use std::sync::Arc;

pub trait Geometry: Send + Sync {
    fn hit(&self, ray: &Ray) -> HitData;
    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal;

    fn set_pos(&mut self, new_pos: nlm::Vec3);
    fn rotate(&mut self, angels: nlm::Vec3);
    fn scale(&mut self, scale: nlm::Vec3);

    fn set_material(&mut self, material: Arc<Material>);
    fn get_mateial(&self) -> Arc<Material>;
}
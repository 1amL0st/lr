use nlm;

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
    pub normal: Normal
}

impl HitData {
    pub fn empty() -> HitData {
        let is_hit = false;
        return HitData {
            is_hit,
            t: 0.,
            normal: Normal::empty()
        }
    }
}

pub trait Geometry: Send + Sync {
    fn hit(&self, ray: &Ray) -> HitData;
    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal;
    fn get_color(&self, hit: &HitData) -> nlm::Vec3;
}
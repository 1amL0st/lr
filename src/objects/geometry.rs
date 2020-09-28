use math::Vec3;

pub struct Ray
{
    pub pos: Vec3,
    pub dir: Vec3
}

impl Ray
{
    pub fn new(pos: Vec3, dir: Vec3) -> Ray {
        Ray {
            pos: pos,
            dir: dir
        }
    }

    pub fn empty() -> Ray {
        Ray {
            pos: Vec3::empty(),
            dir: Vec3::empty()
        }
    }
}

pub struct Normal
{
    pub pos: Vec3,
    pub dir: Vec3
}

impl Normal {
    pub fn new(pos: Vec3, dir: Vec3) -> Normal {
        Normal {
            pos,
            dir
        }
    }

    pub fn empty() -> Normal {
        Normal {
            pos: Vec3::empty(),
            dir: Vec3::empty()
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

pub trait Geometry {
    fn hit(&self, ray: &Ray) -> HitData;
    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal;
    fn get_color(&self, hit: &HitData) -> Vec3;
}
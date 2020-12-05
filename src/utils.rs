use rand::Rng;

use nlm;

pub trait NlmVec3Ext {
    fn zeros() -> nlm::Vec3;
    fn new_color(r: u8, g: u8, b: u8) -> nlm::Vec3;
    fn copy(&self) -> nlm::Vec3;
    fn reflect(&self, normal: &nlm::Vec3) -> nlm::Vec3;
}

impl NlmVec3Ext for nlm::Vec3 {
    fn zeros() -> nlm::Vec3 {
        nlm::Vec3::new(0., 0., 0.)
    }

    fn new_color(r: u8, g: u8, b: u8) -> nlm::Vec3 {
        let factor = 1. / 255.;
        nlm::Vec3::new(
            (r as f32) * factor,
            (g as f32) * factor,
            (b as f32) * factor
        )
    }

    fn copy (&self) -> nlm::Vec3 {
        nlm::Vec3::new(self.x, self.y, self.z)
    }

    fn reflect(&self, normal: &nlm::Vec3) -> nlm::Vec3 {
        (self - (2.0 * self.dot(normal) * normal)).normalize()
    }
}

pub fn point_in_unit_sphere() -> nlm::Vec3 {
    let mut rng = rand::thread_rng();
    let mut point = nlm::Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    while point.magnitude() >= 1.0 {
        point = nlm::Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    }
    point
}

pub fn point_in_unit_sphere_custom_rng(rng: &mut rand::rngs::ThreadRng) -> nlm::Vec3 {
    let mut point = nlm::Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    while point.magnitude() >= 1.0 {
        point = nlm::Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    }
    point
}